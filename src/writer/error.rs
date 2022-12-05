use crate::capi::strerror;

#[derive(PartialEq)]
pub enum WriterConnectError {
    ShmOpenError(i32),
    FtruncateError(i32),
    MmapError(i32),
}

impl std::fmt::Debug for WriterConnectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ShmOpenError(code) => f
                .debug_tuple("ShmOpenError")
                .field(&strerror(*code))
                .finish(),
            Self::FtruncateError(code) => f
                .debug_tuple("FtruncateError")
                .field(&strerror(*code))
                .finish(),
            Self::MmapError(code) => f.debug_tuple("MmapError").field(&strerror(*code)).finish(),
        }
    }
}

#[derive(PartialEq)]
pub enum WriterDisconnectError {
    MunMapError(i32),
    ShmUnlinkError(i32),
}

impl std::fmt::Debug for WriterDisconnectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MunMapError(code) => f
                .debug_tuple("MunMapError")
                .field(&strerror(*code))
                .finish(),
            Self::ShmUnlinkError(code) => f
                .debug_tuple("ShmUnlinkError")
                .field(&strerror(*code))
                .finish(),
        }
    }
}

#[derive(Debug)]
pub enum WriterError {
    ConnectError(WriterConnectError),
    DisconnectError(WriterDisconnectError),
}

impl From<WriterConnectError> for WriterError {
    fn from(err: WriterConnectError) -> Self {
        Self::ConnectError(err)
    }
}

impl From<WriterDisconnectError> for WriterError {
    fn from(err: WriterDisconnectError) -> Self {
        Self::DisconnectError(err)
    }
}
