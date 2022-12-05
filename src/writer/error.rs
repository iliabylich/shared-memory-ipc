use crate::capi::strerror;

#[derive(PartialEq)]
pub enum WriterConnectError {
    ShmOpenError(Option<i32>),
    FtruncateError(Option<i32>),
    MmapError(Option<i32>),
}

impl std::fmt::Debug for WriterConnectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (name, code) = match self {
            Self::ShmOpenError(code) => ("ShmOpenError", *code),
            Self::FtruncateError(code) => ("FtruncateError", *code),
            Self::MmapError(code) => ("MmapError", *code),
        };

        f.debug_tuple(name)
            .field(&code.map(strerror).unwrap_or("Unspecified Error"))
            .finish()
    }
}

#[derive(PartialEq)]
pub enum WriterDisconnectError {
    MunMapError(Option<i32>),
    ShmUnlinkError(Option<i32>),
}

impl std::fmt::Debug for WriterDisconnectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (name, code) = match self {
            Self::MunMapError(code) => ("MunMapError", *code),
            Self::ShmUnlinkError(code) => ("ShmUnlinkError", *code),
        };

        f.debug_tuple(name)
            .field(&code.map(strerror).unwrap_or("Unspecified Error"))
            .finish()
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
