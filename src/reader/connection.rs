use libc::{MAP_SHARED, O_RDWR, PROT_WRITE, S_IRUSR, S_IWUSR};

use crate::{
    capi::{mmap, shm_open},
    reader::{queue::Queue, ReaderConnectError},
    ConnectionType,
};

#[derive(Debug)]
pub struct ReaderConnection<const QUEUE_SIZE: usize> {
    addr: *mut std::ffi::c_void,
    connection_type: ConnectionType,
}

impl<const QUEUE_SIZE: usize> ReaderConnection<QUEUE_SIZE> {
    pub fn new(connection_type: ConnectionType) -> Result<Self, ReaderConnectError> {
        let fd = shm_open(
            connection_type.id(),
            O_RDWR,
            (S_IRUSR | S_IWUSR) as std::ffi::c_uint,
        )
        .map_err(ReaderConnectError::ShmOpenError)?;

        let addr = mmap(
            std::ptr::null_mut(),
            QUEUE_SIZE,
            PROT_WRITE,
            MAP_SHARED,
            fd,
            0,
        )
        .map_err(ReaderConnectError::MmapError)?;

        println!("reader: addr = {:?}", addr);

        let conn = Self {
            addr,
            connection_type,
        };

        println!("[Reader] connected to {:?}", conn.id());

        Ok(conn)
    }

    pub(crate) fn id(&self) -> &std::ffi::CStr {
        self.connection_type.id()
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
        let connection_type = ConnectionType::random();
        let writer = WriterConnection::<10>::new(connection_type.clone()).unwrap();
        let reader = ReaderConnection::<10>::new(connection_type.clone()).unwrap();

        let write_ptr = writer.addr.cast::<u8>();
        let read_ptr = reader.addr.cast::<u8>();
        unsafe {
            std::ptr::write(write_ptr, 42);
        }
        assert_eq!(unsafe { read_ptr.read() }, 42);
    }

    #[test]
    fn test_reader_without_writer() {
        let connection_type = ConnectionType::random();

        let err = ReaderConnection::<10>::new(connection_type).unwrap_err();

        assert_eq!(
            format!("{:?}", err),
            "ShmOpenError(\"No such file or directory\")"
        )
    }
}
