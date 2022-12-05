mod connection;
pub use connection::ReaderConnection;

mod error;
pub use error::{ReaderConnectError, ReaderError};

use crate::ConnectionType;

pub struct Reader<'p, const QUEUE_SIZE: usize> {
    root_connection: ReaderConnection<'p, 1_000>,
    current_connection: ReaderConnection<'p, QUEUE_SIZE>,
}

impl<'p, const QUEUE_SIZE: usize> Reader<'p, QUEUE_SIZE> {
    pub fn new(prefix: &'p str) -> Result<Self, ReaderError> {
        let mut root_connection = ReaderConnection::new(ConnectionType::Root { prefix })?;
        let current_connection = Self::fetch_new_queue_connection(&mut root_connection)?;
        Ok(Self {
            root_connection,
            current_connection,
        })
    }

    fn fetch_new_queue_connection<const ROOT_QUEUE_SIZE: usize>(
        root_connection: &mut ReaderConnection<ROOT_QUEUE_SIZE>,
    ) -> Result<ReaderConnection<'p, QUEUE_SIZE>, ReaderError> {
        let root_queue = root_connection.queue();
        if let Some(queue_name) = root_queue.pop() {
            Ok(ReaderConnection::new(ConnectionType::Exact(queue_name))?)
        } else {
            Err(ReaderError::FailedToGetNextQueue)
        }
    }

    pub fn ipc_pop(&mut self) -> Result<Option<Vec<u8>>, ReaderError> {
        let mut current_queue = self.current_connection.queue();
        if current_queue.done_reading || !current_queue.can_pop() {
            // This queue is over
            self.current_connection = Self::fetch_new_queue_connection(&mut self.root_connection)?;
            current_queue = self.current_connection.queue();
        }

        Ok(current_queue.pop())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Writer;

    #[test]
    fn test_reader() {
        let mut writer = Writer::<20>::new("full-test").unwrap();
        let mut reader = Reader::<20>::new("full-test").unwrap();

        // queue 1
        writer.ipc_push(b"111111111").unwrap();
        assert_eq!(reader.ipc_pop().unwrap(), Some(b"111111111".to_vec()));

        // queue 1
        writer.ipc_push(b"222222222").unwrap();
        assert_eq!(reader.ipc_pop().unwrap(), Some(b"222222222".to_vec()));

        // queue 2
        writer.ipc_push(b"333333333").unwrap();
        assert_eq!(reader.ipc_pop().unwrap(), Some(b"333333333".to_vec()));
    }
}
