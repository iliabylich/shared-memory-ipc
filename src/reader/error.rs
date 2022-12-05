use crate::capi::strerror;

#[derive(PartialEq)]
pub enum ReaderConnectError {
    ShmOpenError(i32),
    MmapError(i32),
}

impl std::fmt::Debug for ReaderConnectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ShmOpenError(code) => f
                .debug_tuple("ShmOpenError")
                .field(&strerror(*code))
                .finish(),
            Self::MmapError(code) => f.debug_tuple("MmapError").field(&strerror(*code)).finish(),
        }
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
