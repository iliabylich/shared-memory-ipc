use libc::strerror;

pub(crate) fn c_code_to_err(code: i32) -> &'static str {
    let err = unsafe { strerror(code) };
    let s = unsafe { std::ffi::CStr::from_ptr(err) };
    s.to_str().unwrap()
}
