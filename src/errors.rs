use std::{convert, io, string};

#[derive(Debug, PartialEq)]
pub enum PlantumDecodeError {
    Deflate(String),
    Hex(String),
}

impl convert::From<string::FromUtf8Error> for PlantumDecodeError {
    fn from(err: string::FromUtf8Error) -> Self {
        PlantumDecodeError::Deflate(format!(
            "there is a problem during deflate decoding: `{}`",
            err
        ))
    }
}

impl convert::From<io::Error> for PlantumDecodeError {
    fn from(err: io::Error) -> Self {
        PlantumDecodeError::Deflate(format!(
            "there is a problem during deflate decoding: `{}`",
            err
        ))
    }
}

impl convert::From<hex::FromHexError> for PlantumDecodeError {
    fn from(err: hex::FromHexError) -> Self {
        PlantumDecodeError::Hex(format!("there is a problem during hex decoding: `{}`", err))
    }
}
