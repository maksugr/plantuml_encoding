use flate2::{
    write::{DeflateDecoder, DeflateEncoder},
    Compression,
};
use std::io::prelude::*;

use crate::utils::{plantuml_decode, plantuml_encode};

pub fn encode_plantuml_deflate(plantuml: String) -> String {
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(plantuml.as_bytes()).unwrap();

    let encoded_bytes = encoder.finish().unwrap();

    plantuml_encode(&encoded_bytes)
}

pub fn decode_plantuml_deflate(plantuml_deflated: String) -> String {
    let result = plantuml_decode(&plantuml_deflated);

    let mut deflater = DeflateDecoder::new(Vec::new());
    deflater.write_all(&result).unwrap();

    String::from_utf8(deflater.finish().unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::constants::TEST_PLANTUML_MIN;
    const TEST_PLANTUML_MIN_DEFLATED: &str = "SrJGjLDmibBmICt9oGS0";

    #[test]
    fn it_encode_plantuml_deflate() {
        assert_eq!(
            encode_plantuml_deflate(String::from(TEST_PLANTUML_MIN)),
            TEST_PLANTUML_MIN_DEFLATED
        );
    }

    #[test]
    fn it_decode_plantuml_deflate() {
        assert_eq!(
            decode_plantuml_deflate(String::from(TEST_PLANTUML_MIN_DEFLATED)),
            TEST_PLANTUML_MIN
        );
    }
}
