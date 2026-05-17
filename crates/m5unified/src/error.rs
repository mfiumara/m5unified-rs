use std::fmt;

/// Errors returned by the high-level wrapper.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// M5Unified initialization failed.
    BeginFailed,
    /// The provided string contained an interior NUL byte.
    InvalidString,
    /// Requested operation is not available on this board/build.
    Unavailable(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BeginFailed => f.write_str("M5Unified initialization failed"),
            Self::InvalidString => f.write_str("string contains an interior NUL byte"),
            Self::Unavailable(feature) => write!(f, "M5Unified feature unavailable: {feature}"),
        }
    }
}

impl std::error::Error for Error {}
