use flate2::{
    write::{DeflateDecoder, DeflateEncoder},
    Compression,
};
use std::io::prelude::*;

use crate::utils::{append_3_bytes, extract_3_bytes};

pub fn encode_plantuml_deflate(plantuml: String) -> String {
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(plantuml.as_bytes()).unwrap();

    let encoded_bytes = encoder.finish().unwrap();

    let mut result = String::new();

    for (index, byte) in encoded_bytes.iter().enumerate().step_by(3) {
        if index + 2 == encoded_bytes.len() {
            result += &append_3_bytes(byte, &encoded_bytes[index + 1], &0);
            continue;
        }

        if index + 1 == encoded_bytes.len() {
            result += &append_3_bytes(byte, &0, &0);
            continue;
        }

        result += &append_3_bytes(byte, &encoded_bytes[index + 1], &encoded_bytes[index + 2]);
    }

    result
}

pub fn decode_plantuml_deflate(plantuml_deflated: String) -> String {
    let mut result = vec![];

    for (index, _) in plantuml_deflated.chars().enumerate().step_by(4) {
        let extract_3_bytes = extract_3_bytes(&plantuml_deflated[index..index + 4]);

        result.push(extract_3_bytes[0]);
        result.push(extract_3_bytes[1]);
        result.push(extract_3_bytes[2]);
    }

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
