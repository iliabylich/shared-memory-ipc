use libc::{
    ftruncate, mmap, munmap, shm_open, shm_unlink, MAP_FAILED, MAP_SHARED, O_CREAT, O_RDWR,
    PROT_WRITE, S_IRUSR, S_IWUSR,
};

use crate::{
    errno,
    writer::error::{WriterConnectError, WriterDisconnectError},
    ConnectionType, Queue,
};

#[derive(Debug)]
pub struct WriterConnection<const QUEUE_SIZE: usize> {
    fd: i32,
    pub(crate) addr: *mut std::ffi::c_void,
    connection_type: ConnectionType,
    prefix: &'static str,
}

impl<const QUEUE_SIZE: usize> WriterConnection<QUEUE_SIZE> {
    pub fn new(
        connection_type: ConnectionType,
        prefix: &'static str,
    ) -> Result<Self, WriterConnectError> {
        let fd = unsafe {
            shm_open(
                connection_type.id(prefix).into_raw(),
                O_RDWR | O_CREAT,
                (S_IRUSR | S_IWUSR) as std::ffi::c_uint,
            )
        };

        if fd == -1 {
            return Err(WriterConnectError::ShmOpenError(errno().unwrap()));
        }

        let res = unsafe { ftruncate(fd, QUEUE_SIZE as i64) };
        if res == -1 {
            return Err(WriterConnectError::FtruncateError(errno().unwrap()));
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
            return Err(WriterConnectError::MmapError(errno().unwrap()));
        }

        let conn = Self {
            fd,
            addr,
            connection_type,
            prefix,
        };
        println!("[Writer] Connected {:?}", conn.id());

        Ok(conn)
    }

    pub(crate) fn id(&self) -> String {
        self.connection_type.id(self.prefix).into_string().unwrap()
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

        let code = unsafe { munmap(addr, QUEUE_SIZE) };
        if code == -1 {
            return Err(WriterDisconnectError::MunMapError(code));
        }

        let code = unsafe { shm_unlink(self.connection_type.id(self.prefix).into_raw()) };
        if code == -1 {
            return Err(WriterDisconnectError::ShmUnlinkError(code));
        }

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
        let connection_type = ConnectionType::Dummy("writer-success");
        let connection = WriterConnection::<10>::new(connection_type, "prefix").unwrap();

        let write_ptr = connection.addr.cast::<u8>();
        unsafe {
            std::ptr::write(write_ptr, 42);
        }
        assert_eq!(unsafe { write_ptr.read() }, 42);
    }

    #[test]
    fn test_invalid_name() {
        let connection_type = ConnectionType::Empty;

        let err = WriterConnection::<10>::new(connection_type, "prefix").unwrap_err();

        assert_eq!(format!("{:?}", err), "ShmOpenError(\"Invalid argument\")")
    }
}
