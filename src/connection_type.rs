use std::ffi::{CStr, CString};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionType {
    id: CString,
}

impl ConnectionType {
    pub fn root(prefix: &str) -> Self {
        let id = format!("/{}-root", prefix);
        Self {
            id: CString::new(id).unwrap(),
        }
    }

    pub fn worker(n: usize, prefix: &str) -> Self {
        let id = format!("/{}-worker-{}", prefix, n);
        Self {
            id: CString::new(id).unwrap(),
        }
    }

    pub fn exact(name: &[u8]) -> Self {
        Self {
            id: CString::new(name.to_vec()).unwrap(),
        }
    }

    #[cfg(test)]
    pub fn dummy(name: &str) -> Self {
        let id = format!("/{}", name);
        Self {
            id: CString::new(id).unwrap(),
        }
    }

    #[cfg(test)]
    pub fn empty() -> Self {
        Self {
            id: CString::new("").unwrap(),
        }
    }

    pub fn id(&self) -> &CStr {
        self.id.as_c_str()
    }
}
