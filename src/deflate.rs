use flate2::{
    write::{DeflateDecoder, DeflateEncoder},
    Compression,
};
use std::io::prelude::*;

use crate::utils::{decode_plantuml_for_deflate, encode_plantuml_for_deflate};

pub fn encode_plantuml_deflate<T: AsRef<str>>(plantuml: T) -> String {
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(plantuml.as_ref().as_bytes()).unwrap();

    let encoded_bytes = encoder.finish().unwrap();

    encode_plantuml_for_deflate(&encoded_bytes)
}

pub fn decode_plantuml_deflate<T: AsRef<str>>(plantuml_deflated: T) -> String {
    let result = decode_plantuml_for_deflate(plantuml_deflated.as_ref());

    let mut deflater = DeflateDecoder::new(Vec::new());
    deflater.write_all(&result).unwrap();

    String::from_utf8(deflater.finish().unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{decode_plantuml_deflate, encode_plantuml_deflate};

    use crate::tests::constants::{
        plantuml_deflated_str::{PLANTUML_DEFLATED_LARGE, PLANTUML_DEFLATED_SMALL},
        plantuml_str::{PLANTUML_LARGE, PLANTUML_SMALL},
    };

    #[test]
    fn it_encode_plantuml_deflate_small() {
        assert_eq!(
            encode_plantuml_deflate(PLANTUML_SMALL),
            PLANTUML_DEFLATED_SMALL
        );
    }

    #[test]
    fn it_encode_plantuml_deflate_small_string() {
        assert_eq!(
            encode_plantuml_deflate(String::from(PLANTUML_SMALL)),
            PLANTUML_DEFLATED_SMALL
        );
    }

    #[test]
    fn it_decode_plantuml_deflate_small() {
        assert_eq!(
            decode_plantuml_deflate(PLANTUML_DEFLATED_SMALL),
            PLANTUML_SMALL
        );
    }

    #[test]
    fn it_decode_plantuml_deflate_small_string() {
        assert_eq!(
            decode_plantuml_deflate(String::from(PLANTUML_DEFLATED_SMALL)),
            PLANTUML_SMALL
        );
    }

    #[test]
    fn it_encode_plantuml_deflate_large() {
        assert_eq!(
            encode_plantuml_deflate(PLANTUML_LARGE),
            PLANTUML_DEFLATED_LARGE
        );
    }

    #[test]
    fn it_decode_plantuml_deflate_large() {
        assert_eq!(
            decode_plantuml_deflate(PLANTUML_DEFLATED_LARGE),
            PLANTUML_LARGE
        );
    }
}
