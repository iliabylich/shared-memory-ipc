#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionType<'p> {
    Root {
        prefix: &'p str,
    },
    Worker {
        n: usize,
        prefix: &'p str,
    },
    Exact(Vec<u8>),

    #[cfg(test)]
    Dummy(&'static str),

    #[cfg(test)]
    Empty,
}

impl<'p> ConnectionType<'p> {
    pub fn id(&self) -> std::ffi::CString {
        let id = match self {
            Self::Root { prefix } => format!("/{}-root", prefix),
            Self::Worker { n, prefix } => format!("/{}-worker-{}", prefix, n),
            Self::Exact(name) => return std::ffi::CString::new(name.to_owned()).unwrap(),

            #[cfg(test)]
            Self::Dummy(s) => format!("test-{}", s),

            #[cfg(test)]
            Self::Empty => return std::ffi::CString::new("").unwrap(),
        };

        unsafe { std::ffi::CString::new(id).unwrap_unchecked() }
    }
}
