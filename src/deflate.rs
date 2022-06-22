use flate2::write;
use std::io::prelude::*;

use crate::errors;
use crate::utils;

pub fn encode_plantuml_deflate<T: AsRef<str>>(
    plantuml: T,
) -> Result<String, errors::PlantumDecodeError> {
    let mut encoder = write::DeflateEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(plantuml.as_ref().as_bytes())?;

    let encoded_bytes = encoder.finish()?;

    Ok(utils::encode_plantuml_for_deflate(&encoded_bytes))
}

pub fn decode_plantuml_deflate<T: AsRef<str>>(
    plantuml_deflated: T,
) -> Result<String, errors::PlantumDecodeError> {
    let result = match utils::decode_plantuml_for_deflate(plantuml_deflated.as_ref()) {
        Some(r) => r,
        None => {
            return Err(errors::PlantumDecodeError::Deflate(
                "internal decoding error (out of bounds or similar)".to_string(),
            ));
        }
    };

    let mut deflater = write::DeflateDecoder::new(Vec::new());
    deflater.write_all(&result)?;

    let decoded_bytes = deflater.finish()?;

    Ok(String::from_utf8(decoded_bytes)?)
}

#[cfg(test)]
mod tests {
    use super::{decode_plantuml_deflate, encode_plantuml_deflate};

    use crate::errors;
    use crate::tests::constants::{
        plantuml_deflated_str::{PLANTUML_DEFLATED_LARGE, PLANTUML_DEFLATED_SMALL},
        plantuml_str::{PLANTUML_LARGE, PLANTUML_SMALL},
    };

    #[test]
    fn it_encode_plantuml_deflate_small() {
        assert_eq!(
            encode_plantuml_deflate(PLANTUML_SMALL),
            Ok(PLANTUML_DEFLATED_SMALL.to_string())
        );
    }

    #[test]
    fn it_encode_plantuml_deflate_small_string() {
        assert_eq!(
            encode_plantuml_deflate(String::from(PLANTUML_SMALL)),
            Ok(PLANTUML_DEFLATED_SMALL.to_string())
        );
    }

    #[test]
    fn it_decode_plantuml_deflate_small() {
        assert_eq!(
            decode_plantuml_deflate(PLANTUML_DEFLATED_SMALL),
            Ok(PLANTUML_SMALL.to_string())
        );
    }

    #[test]
    fn it_decode_plantuml_deflate_small_string() {
        assert_eq!(
            decode_plantuml_deflate(String::from(PLANTUML_DEFLATED_SMALL)),
            Ok(PLANTUML_SMALL.to_string())
        );
    }

    #[test]
    fn it_encode_plantuml_deflate_large() {
        assert_eq!(
            encode_plantuml_deflate(PLANTUML_LARGE),
            Ok(PLANTUML_DEFLATED_LARGE.to_string())
        );
    }

    #[test]
    fn it_decode_plantuml_deflate_large() {
        assert_eq!(
            decode_plantuml_deflate(PLANTUML_DEFLATED_LARGE),
            Ok(PLANTUML_LARGE.to_string())
        );
    }

    #[test]
    fn it_decode_plantuml_deflate_regular_error() {
        assert_eq!(
            decode_plantuml_deflate("4444"),
            Err(errors::PlantumDecodeError::Deflate(
                "there is problem a problem during deflate deconding: `deflate decompression error`".to_string()
            ))
        );
    }

    #[test]
    fn it_decode_plantuml_deflate_out_of_bounds_error() {
        assert_eq!(
            decode_plantuml_deflate("some strange string"),
            Err(errors::PlantumDecodeError::Deflate(
                "internal decoding error (out of bounds or similar)".to_string()
            ))
        );
    }
}
