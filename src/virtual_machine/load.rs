use zip::result::ZipError;

use super::VirtualMachine;

pub type VMLoadResult = Result<VirtualMachine, VMLoadError>;

#[derive(Debug)]
pub enum VMLoadError {
    Io(std::io::Error),
    Parse(serde_json::Error),
    Zip(ZipError),
}

impl From<std::io::Error> for VMLoadError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<serde_json::Error> for VMLoadError {
    fn from(err: serde_json::Error) -> Self {
        Self::Parse(err)
    }
}

impl From<ZipError> for VMLoadError {
    fn from(err: ZipError) -> Self {
        Self::Zip(err)
    }
}

impl std::fmt::Display for VMLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "{}", err),
            Self::Parse(err) => write!(f, "{}", err),
            Self::Zip(err) => write!(f, "{}", err),
        }
    }
}
