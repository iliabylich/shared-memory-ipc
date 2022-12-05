#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionType {
    Root,
    Worker(usize),
    Exact(Vec<u8>),

    #[cfg(test)]
    Dummy(&'static str),
    #[cfg(test)]
    Empty,
}

impl ConnectionType {
    pub fn id(&self, prefix: &str) -> std::ffi::CString {
        let id = match self {
            Self::Root => String::from("root"),
            Self::Worker(idx) => format!("worker-{}", idx),
            Self::Exact(name) => return std::ffi::CString::new(name.to_owned()).unwrap(),
            #[cfg(test)]
            Self::Dummy(s) => format!("test-{}", s),
            #[cfg(test)]
            Self::Empty => return std::ffi::CString::new("").unwrap(),
        };

        unsafe { std::ffi::CString::new(format!("/{}-{}", prefix, id)).unwrap_unchecked() }
    }
}
