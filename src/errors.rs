use std::{convert, io, string};

/// If error appear, the crate always faults to error type `PlantumlDecodingError`.
/// All other error types converted to this one.
/// No panic is expected.
#[derive(Debug, PartialEq)]
pub enum PlantumlDecodingError {
    /// Source of the error - deflate decoding
    Deflate(String),
    /// Source of the error - hex decoding
    Hex(String),
}

impl convert::From<string::FromUtf8Error> for PlantumlDecodingError {
    fn from(err: string::FromUtf8Error) -> Self {
        PlantumlDecodingError::Deflate(format!(
            "there is a problem during deflate decoding: `{}`",
            err
        ))
    }
}

impl convert::From<io::Error> for PlantumlDecodingError {
    fn from(err: io::Error) -> Self {
        PlantumlDecodingError::Deflate(format!(
            "there is a problem during deflate decoding: `{}`",
            err
        ))
    }
}

impl convert::From<hex::FromHexError> for PlantumlDecodingError {
    fn from(err: hex::FromHexError) -> Self {
        PlantumlDecodingError::Hex(format!("there is a problem during hex decoding: `{}`", err))
    }
}
