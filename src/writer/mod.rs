mod connection;
pub use connection::WriterConnection;

mod error;
pub use error::{WriterConnectError, WriterDisconnectError, WriterError};

use crate::ConnectionType;

pub struct Writer<'p, const QUEUE_SIZE: usize> {
    root_connection: WriterConnection<1_000>,
    connections: Vec<WriterConnection<QUEUE_SIZE>>,
    prefix: &'p str,
}

impl<'p, const QUEUE_SIZE: usize> Writer<'p, QUEUE_SIZE> {
    pub fn new(prefix: &'p str) -> Result<Self, WriterError> {
        let root_connection = WriterConnection::new(ConnectionType::root(prefix))?;
        let mut writer = Self {
            root_connection,
            connections: vec![],
            prefix,
        };
        writer.provision_new_queue_connection()?;

        Ok(writer)
    }

    pub fn cleanup(&mut self) -> Result<(), WriterError> {
        for connection in &mut self.connections {
            if connection.is_stale() {
                connection.disconnect()?;
            }
        }

        Ok(())
    }

    pub(crate) fn provision_new_queue_connection(&mut self) -> Result<(), WriterError> {
        self.cleanup()?;

        let connection =
            WriterConnection::new(ConnectionType::worker(self.connections.len(), self.prefix))?;

        self.connections.push(connection);
        self.notify_about_new_queue();

        Ok(())
    }

    fn notify_about_new_queue(&mut self) {
        let new_conn_id = self.connections.last().unwrap().id();
        println!("[Writer] Notifying about new queue {:?}", new_conn_id);
        self.root_connection.queue().push(new_conn_id.to_bytes())
    }

    pub fn ipc_push(&mut self, message: &[u8]) -> Result<(), WriterError> {
        let mut current_queue = self.connections.last().unwrap().queue();

        if !current_queue.can_push(message) {
            self.provision_new_queue_connection()?;
            current_queue.done_writing = true;
            current_queue = self.connections.last().unwrap().queue();
        }

        current_queue.push(message);

        Ok(())
    }
}

impl<'p, const QUEUE_SIZE: usize> Drop for Writer<'p, QUEUE_SIZE> {
    fn drop(&mut self) {
        self.root_connection.disconnect().unwrap();

        for conn in &mut self.connections {
            conn.disconnect().unwrap()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const QUEUE_SIZE: usize = 20;

    #[test]
    fn test_queue_provisioning() {
        let mut writer = Writer::<QUEUE_SIZE>::new("writer").unwrap();

        writer.ipc_push(b"111111111").unwrap();
        writer.ipc_push(b"222222222").unwrap();
        writer.ipc_push(b"333333333").unwrap();
        writer.ipc_push(b"444444444").unwrap();

        let root_queue = writer.root_connection.queue();
        assert_eq!(
            root_queue.messages(),
            vec!["/writer-worker-0", "/writer-worker-1",]
        );

        let queue1 = writer.connections[0].queue();
        assert_eq!(queue1.messages(), vec!["111111111", "222222222"]);

        let queue2 = writer.connections[1].queue();
        assert_eq!(queue2.messages(), vec!["333333333", "444444444"]);
    }

    #[test]
    fn test_cleanup() {
        let mut writer = Writer::<QUEUE_SIZE>::new("writer-2").unwrap();

        writer.ipc_push(b"111111111").unwrap();
        writer.ipc_push(b"222222222").unwrap();
        writer.ipc_push(b"333333333").unwrap();
        writer.ipc_push(b"444444444").unwrap();

        // manually mark writer queues as stale
        for conn in &mut writer.connections {
            conn.queue().done_reading = true;
        }

        writer.cleanup().unwrap();

        assert_eq!(writer.connections[0].addr, std::ptr::null_mut());
        assert_eq!(writer.connections[1].addr, std::ptr::null_mut());

        // check how they are ignored on the next cleanup
        writer.cleanup().unwrap();
    }
}
