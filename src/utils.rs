fn encode_6_bit(mut b: u8) -> String {
    if b < 10 {
        return String::from((48 + b) as char);
    }

    b -= 10;

    if b < 26 {
        return String::from((65 + b) as char);
    }

    b -= 26;

    if b < 26 {
        return String::from((97 + b) as char);
    }

    b -= 26;

    if b == 0 {
        return String::from("-");
    }

    if b == 1 {
        return String::from("_");
    }

    String::from("?")
}

fn append_3_bytes(b1: &u8, b2: &u8, b3: &u8) -> String {
    let c1 = b1 >> 2;
    let c2 = ((b1 & 0x3) << 4) | (b2 >> 4);
    let c3 = ((b2 & 0xF) << 2) | (b3 >> 6);
    let c4 = b3 & 0x3F;

    let mut result = String::new();

    result += &encode_6_bit(c1 & 0x3F);
    result += &encode_6_bit(c2 & 0x3F);
    result += &encode_6_bit(c3 & 0x3F);
    result += &encode_6_bit(c4 & 0x3F);

    result
}

pub fn encode_plantuml_for_deflate(encoded_bytes: &[u8]) -> String {
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

fn decode_6_bit(s: String) -> Option<u8> {
    let c = s.chars().next()? as u8;

    if s == "_" {
        return Some(63);
    };
    if s == "-" {
        return Some(62);
    }
    if c >= 97 {
        return Some(c - 61);
    }
    if c >= 65 {
        return Some(c - 55);
    }
    if c >= 48 {
        return Some(c - 48);
    }

    Some(0)
}

fn extract_3_bytes(chars: &[char]) -> Option<[u8; 3]> {
    let mut chars = chars.iter();

    let c1 = decode_6_bit(String::from(*chars.next()?))?;
    let c2 = decode_6_bit(String::from(*chars.next()?))?;
    let c3 = decode_6_bit(String::from(*chars.next()?))?;
    let c4 = decode_6_bit(String::from(*chars.next()?))?;

    let b1 = c1 << 2 | (c2 >> 4) & 0x3F;
    let b2 = (c2 << 4) & 0xF0 | (c3 >> 2) & 0xF;
    let b3 = (c3 << 6) & 0xC0 | c4 & 0x3F;

    Some([b1, b2, b3])
}

pub fn decode_plantuml_for_deflate(decoded_string: &str) -> Option<Vec<u8>> {
    let mut result = vec![];

    for chunk in decoded_string.chars().collect::<Vec<char>>().chunks(4) {
        result.extend(extract_3_bytes(chunk)?);
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::{decode_plantuml_for_deflate, encode_plantuml_for_deflate};

    use crate::tests::constants::{
        plantuml_for_deflate_str::{
            PLANTUML_FOR_DEFLATE_ENCODED_LARGE, PLANTUML_FOR_DEFLATE_ENCODED_SMALL,
        },
        plantuml_for_deflate_u8::{PLANTUML_FOR_DEFLATE_RAW_LARGE, PLANTUML_FOR_DEFLATE_RAW_SMALL},
    };

    #[test]
    fn it_encode_plantuml_for_deflate_small() {
        assert_eq!(
            encode_plantuml_for_deflate(&PLANTUML_FOR_DEFLATE_RAW_SMALL),
            PLANTUML_FOR_DEFLATE_ENCODED_SMALL
        );
    }

    #[test]
    fn it_decode_plantuml_for_deflate_small() {
        assert_eq!(
            decode_plantuml_for_deflate(PLANTUML_FOR_DEFLATE_ENCODED_SMALL),
            Some(PLANTUML_FOR_DEFLATE_RAW_SMALL.to_vec())
        );
    }

    #[test]
    fn it_encode_plantuml_for_deflate_large() {
        assert_eq!(
            encode_plantuml_for_deflate(&PLANTUML_FOR_DEFLATE_RAW_LARGE),
            PLANTUML_FOR_DEFLATE_ENCODED_LARGE
        );
    }

    #[test]
    fn it_decode_plantuml_for_deflate_large() {
        assert_eq!(
            decode_plantuml_for_deflate(PLANTUML_FOR_DEFLATE_ENCODED_LARGE),
            Some(PLANTUML_FOR_DEFLATE_RAW_LARGE.to_vec())
        );
    }

    #[test]
    fn it_decode_plantuml_for_deflate_out_of_bounds_error() {
        assert_eq!(decode_plantuml_for_deflate("some strange string"), None);
    }
}
