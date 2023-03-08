use libc::{MAP_SHARED, O_CREAT, O_RDWR, PROT_WRITE, S_IRUSR, S_IWUSR};

use crate::{
    capi::{ftruncate, mmap, munmap, shm_open, shm_unlink},
    writer::{
        error::{WriterConnectError, WriterDisconnectError},
        queue::Queue,
    },
    ConnectionType,
};

#[derive(Debug)]
pub struct WriterConnection<const QUEUE_SIZE: usize> {
    fd: i32,
    pub(crate) addr: *mut std::ffi::c_void,
    connection_type: ConnectionType,
}

impl<const QUEUE_SIZE: usize> WriterConnection<QUEUE_SIZE> {
    pub fn new(connection_type: ConnectionType) -> Result<Self, WriterConnectError> {
        let fd = shm_open(
            connection_type.id(),
            O_RDWR | O_CREAT,
            (S_IRUSR | S_IWUSR) as std::ffi::c_uint,
        )
        .map_err(WriterConnectError::ShmOpenError)?;

        ftruncate(fd, QUEUE_SIZE as i64).map_err(WriterConnectError::FtruncateError)?;

        let addr = mmap(
            std::ptr::null_mut(),
            QUEUE_SIZE,
            PROT_WRITE,
            MAP_SHARED,
            fd,
            0,
        )
        .map_err(WriterConnectError::MmapError)?;

        println!("writer: addr = {:?}", addr);

        let conn = Self {
            fd,
            addr,
            connection_type,
        };
        println!("[Writer] Connected {:?}", conn.id());

        Ok(conn)
    }

    pub(crate) fn id(&self) -> &std::ffi::CStr {
        self.connection_type.id()
    }

    pub(crate) fn is_stale(&self) -> bool {
        !self.addr.is_null() && self.queue().done_reading
    }

    pub(crate) fn disconnect(&mut self) -> Result<(), WriterDisconnectError> {
        let addr = self.addr;
        let fd = self.fd;

        if addr.is_null() && fd == 0 {
            return Ok(());
        }

        self.addr = std::ptr::null_mut();
        self.fd = 0;

        munmap(addr, QUEUE_SIZE).map_err(WriterDisconnectError::MunMapError)?;
        shm_unlink(self.connection_type.id()).map_err(WriterDisconnectError::ShmUnlinkError)?;

        Ok(())
    }

    pub(crate) fn queue(&self) -> &'static mut Queue<QUEUE_SIZE> {
        Queue::from_ptr(self.addr)
    }
}

impl<const N: usize> Drop for WriterConnection<N> {
    fn drop(&mut self) {
        println!("[Writer] Disconnecting {:?}", self.id());
        self.disconnect().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::{ConnectionType, WriterConnection};

    #[test]
    fn test_success() {
        let connection_type = ConnectionType::dummy("writer-success");
        let connection = WriterConnection::<10>::new(connection_type).unwrap();

        let write_ptr = connection.addr.cast::<u8>();
        unsafe {
            std::ptr::write(write_ptr, 42);
        }
        assert_eq!(unsafe { write_ptr.read() }, 42);
    }

    #[test]
    fn test_invalid_name() {
        let connection_type = ConnectionType::empty();

        let err = WriterConnection::<10>::new(connection_type).unwrap_err();

        assert_eq!(format!("{:?}", err), "ShmOpenError(\"Invalid argument\")")
    }
}
