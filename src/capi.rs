use std::ffi::{c_int, c_uint, c_void, CStr};

pub(crate) fn strerror(code: c_int) -> &'static str {
    let err = unsafe { libc::strerror(code) };
    let s = unsafe { std::ffi::CStr::from_ptr(err) };
    s.to_str().unwrap()
}

pub(crate) fn errno() -> i32 {
    std::io::Error::last_os_error().raw_os_error().unwrap()
}

pub(crate) fn shm_open(name: &CStr, oflag: c_int, mode: c_uint) -> Result<i32, i32> {
    let fd = unsafe { libc::shm_open(name.as_ptr(), oflag, mode) };
    if fd == -1 {
        Err(errno())
    } else {
        Ok(fd)
    }
}

pub(crate) fn ftruncate(fd: c_int, length: i64) -> Result<(), i32> {
    let res = unsafe { libc::ftruncate(fd, length) };
    if res == -1 {
        Err(errno())
    } else {
        Ok(())
    }
}

pub(crate) fn mmap(
    addr: *mut c_void,
    length: usize,
    protection: c_int,
    flags: c_int,
    fd: c_int,
    offset: i64,
) -> Result<*mut c_void, i32> {
    let addr = unsafe { libc::mmap(addr, length, protection, flags, fd, offset) };
    if addr == libc::MAP_FAILED {
        Err(errno())
    } else {
        Ok(addr)
    }
}

pub(crate) fn munmap(addr: *mut c_void, length: usize) -> Result<(), i32> {
    let code = unsafe { libc::munmap(addr, length) };
    if code == -1 {
        Err(errno())
    } else {
        Ok(())
    }
}

pub(crate) fn shm_unlink(name: &CStr) -> Result<(), i32> {
    let code = unsafe { libc::shm_unlink(name.as_ptr()) };
    if code == -1 {
        Err(errno())
    } else {
        Ok(())
    }
}
