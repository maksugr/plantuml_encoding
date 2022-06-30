use std::{convert, io, string};

/// If error appear, the crate always faults to error type `FromPlantumlError`.
/// All other error types converted to this one.
/// No panic is expected.
#[derive(Debug, PartialEq)]
pub struct FromPlantumlError(pub String);

impl convert::From<string::FromUtf8Error> for FromPlantumlError {
    fn from(err: string::FromUtf8Error) -> Self {
        FromPlantumlError(format!("there is a problem during decoding: `{}`", err))
    }
}

impl convert::From<io::Error> for FromPlantumlError {
    fn from(err: io::Error) -> Self {
        FromPlantumlError(format!(
            "there is a problem during deflate decoding: `{}`",
            err
        ))
    }
}

impl convert::From<hex::FromHexError> for FromPlantumlError {
    fn from(err: hex::FromHexError) -> Self {
        FromPlantumlError(format!("there is a problem during hex decoding: `{}`", err))
    }
}
