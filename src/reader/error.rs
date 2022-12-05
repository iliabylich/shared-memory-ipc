use crate::capi::strerror;

#[derive(PartialEq)]
pub enum ReaderConnectError {
    ShmOpenError(Option<i32>),
    MmapError(Option<i32>),
}

impl std::fmt::Debug for ReaderConnectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (name, code) = match self {
            Self::ShmOpenError(code) => ("ShmOpenError", *code),
            Self::MmapError(code) => ("MmapError", *code),
        };

        f.debug_tuple(name)
            .field(&code.map(strerror).unwrap_or("Unspecified Error"))
            .finish()
    }
}

#[derive(PartialEq, Debug)]
pub enum ReaderError {
    ReaderConnectError(ReaderConnectError),
    FailedToGetNextQueue,
}

impl From<ReaderConnectError> for ReaderError {
    fn from(err: ReaderConnectError) -> Self {
        Self::ReaderConnectError(err)
    }
}
