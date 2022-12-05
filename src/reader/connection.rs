use libc::{mmap, shm_open, MAP_FAILED, MAP_SHARED, O_RDWR, PROT_WRITE, S_IRUSR, S_IWUSR};

use crate::{errno, reader::ReaderConnectError, ConnectionType, Queue};

#[derive(Debug)]
pub struct ReaderConnection<const QUEUE_SIZE: usize> {
    addr: *mut std::ffi::c_void,
    connection_type: ConnectionType,
    prefix: &'static str,
}

impl<const QUEUE_SIZE: usize> ReaderConnection<QUEUE_SIZE> {
    pub fn new(
        connection_type: ConnectionType,
        prefix: &'static str,
    ) -> Result<Self, ReaderConnectError> {
        let fd = unsafe {
            shm_open(
                connection_type.id(prefix).into_raw(),
                O_RDWR,
                (S_IRUSR | S_IWUSR) as std::ffi::c_uint,
            )
        };

        if fd == -1 {
            return Err(ReaderConnectError::ShmOpenError(errno().unwrap()));
        }

        let addr = unsafe {
            mmap(
                std::ptr::null_mut(),
                QUEUE_SIZE,
                PROT_WRITE,
                MAP_SHARED,
                fd,
                0,
            )
        };
        if addr == MAP_FAILED {
            return Err(ReaderConnectError::MmapError(errno().unwrap()));
        }

        let conn = Self {
            addr,
            connection_type,
            prefix,
        };

        println!("[Reader] connected to {:?}", conn.id());

        Ok(conn)
    }

    pub(crate) fn id(&self) -> String {
        self.connection_type.id(self.prefix).into_string().unwrap()
    }

    pub(crate) fn queue(&self) -> &'static mut Queue<QUEUE_SIZE> {
        Queue::from_ptr(self.addr)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ConnectionType, ReaderConnection, WriterConnection};

    #[test]
    fn test_success() {
        let connection_type = ConnectionType::Dummy("reader-success");
        let writer = WriterConnection::<10>::new(connection_type.clone(), "prefix").unwrap();
        let reader = ReaderConnection::<10>::new(connection_type.clone(), "prefix").unwrap();

        let write_ptr = writer.addr.cast::<u8>();
        let read_ptr = reader.addr.cast::<u8>();
        unsafe {
            std::ptr::write(write_ptr, 42);
        }
        assert_eq!(unsafe { read_ptr.read() }, 42);
    }

    #[test]
    fn test_reader_without_writer() {
        let connection_type = ConnectionType::Dummy("reader-wo-writer");

        let err = ReaderConnection::<10>::new(connection_type, "prefix").unwrap_err();

        assert_eq!(
            format!("{:?}", err),
            "ShmOpenError(\"No such file or directory\")"
        )
    }
}
