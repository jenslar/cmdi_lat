//! Various errors for parsing, processing and building EAF-files.

use std::{fmt, path::PathBuf};

use crate::Cmd;

#[derive(Debug)]
/// Various errors for parsing, processing and building EAF-files.
pub enum CmdError {
    DeError(quick_xml::DeError),
    IOError(std::io::Error),
    InvalidPath(PathBuf),
}

impl std::error::Error for CmdError {}
impl fmt::Display for CmdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DeError(de_error) => write!(f, "{de_error}"),
            Self::IOError(io_error) => write!(f, "{io_error}"),
            Self::InvalidPath(path) => write!(f, "No such file '{}'", path.display()),
        }
    }
}

/// Converts CmdError to std::io::Error
impl From<CmdError> for std::io::Error {
    fn from(err: CmdError) -> std::io::Error {
        std::io::Error::new(std::io::ErrorKind::Other, err) // for returning EafErrors in main()
    }
}

/// Converts std::io::Error to CmdError
impl From<std::io::Error> for CmdError {
    fn from(err: std::io::Error) -> CmdError {
        CmdError::IOError(err)
    }
}

/// Converts CmdError to std::io::Error
impl From<quick_xml::DeError> for CmdError {
    fn from(err: quick_xml::DeError) -> CmdError {
        CmdError::DeError(err) // for returning EafErrors in main()
    }
}
