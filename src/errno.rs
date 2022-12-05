pub(crate) fn errno() -> Option<i32> {
    std::io::Error::last_os_error().raw_os_error()
}
