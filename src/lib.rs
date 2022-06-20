use flate2::{
    write::{DeflateDecoder, DeflateEncoder},
    Compression,
};
use std::io::prelude::*;

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

pub fn encode(plantuml: String) -> String {
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

fn decode_reader(bytes: Vec<u8>) -> String {
    let mut writer = Vec::new();
    let mut deflater = DeflateDecoder::new(writer);

    deflater.write_all(&bytes[..]).unwrap();
    writer = deflater.finish().unwrap();

    String::from_utf8(writer).unwrap()
}

fn decode_6_bit(cc: String) -> u8 {
    let c = cc.chars().next().unwrap() as u8;

    if cc == "_" {
        return 63;
    };
    if cc == "-" {
        return 62;
    }
    if c >= 97 {
        return c - 61;
    }
    if c >= 65 {
        return c - 55;
    }
    if c >= 48 {
        return c - 48;
    }

    0
}

fn extract_3_bytes(data: &str) -> [u8; 3] {
    let mut chars = data.chars();

    let c1 = decode_6_bit(String::from(chars.next().unwrap()));
    let c2 = decode_6_bit(String::from(chars.next().unwrap()));
    let c3 = decode_6_bit(String::from(chars.next().unwrap()));
    let c4 = decode_6_bit(String::from(chars.next().unwrap()));

    let b1 = c1 << 2 | (c2 >> 4) & 0x3F;
    let b2 = (c2 << 4) & 0xF0 | (c3 >> 2) & 0xF;
    let b3 = (c3 << 6) & 0xC0 | c4 & 0x3F;

    [b1, b2, b3]
}

pub fn decode(encoded_plantuml: String) -> String {
    let mut result = vec![];

    for (index, _) in encoded_plantuml.chars().enumerate().step_by(4) {
        let extract_3_bytes = extract_3_bytes(&encoded_plantuml[index..index + 4]);

        result.push(extract_3_bytes[0]);
        result.push(extract_3_bytes[1]);
        result.push(extract_3_bytes[2]);
    }

    decode_reader(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    const PLANTUML: &str = "A -> B: Hello";
    const ENCODED_PLANTUML: &str = "SrJGjLDmibBmICt9oGS0";

    #[test]
    fn it_encode() {
        assert_eq!(encode(String::from(PLANTUML)), ENCODED_PLANTUML);
    }

    #[test]
    fn it_decode() {
        assert_eq!(decode(String::from(ENCODED_PLANTUML)), PLANTUML);
    }
}
