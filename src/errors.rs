use std::{convert, io, string};

#[derive(Debug, PartialEq)]
pub enum PlantumDecodeError {
    Deflate(String),
    Hex(String),
}

impl convert::From<string::FromUtf8Error> for PlantumDecodeError {
    fn from(err: string::FromUtf8Error) -> Self {
        PlantumDecodeError::Deflate(format!(
            "there is problem a problem during deflate deconding: `{}`",
            err
        ))
    }
}

impl convert::From<io::Error> for PlantumDecodeError {
    fn from(err: io::Error) -> Self {
        PlantumDecodeError::Deflate(format!(
            "there is problem a problem during deflate deconding: `{}`",
            err
        ))
    }
}

impl convert::From<hex::FromHexError> for PlantumDecodeError {
    fn from(err: hex::FromHexError) -> Self {
        PlantumDecodeError::Hex(format!(
            "there is problem a problem during hex deconding: `{}`",
            err
        ))
    }
}
