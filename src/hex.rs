use crate::utils::{append_3_bytes, extract_3_bytes};

pub fn encode_plantuml_hex(plantuml: String) -> String {
    let hex = hex::encode(plantuml);
    let encoded_bytes = hex.as_bytes();

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

    String::from("~h") + &result
}

pub fn decode_plantuml_hex(plantuml_hex: String) -> String {
    let mut result = vec![];

    let plantuml_hex_trimmed = plantuml_hex.trim_start_matches("~h");

    for (index, _) in plantuml_hex_trimmed.chars().enumerate().step_by(4) {
        let extract_3_bytes = extract_3_bytes(&plantuml_hex_trimmed[index..index + 4]);

        result.push(extract_3_bytes[0]);
        result.push(extract_3_bytes[1]);
        result.push(extract_3_bytes[2]);
    }

    let hex = String::from_utf8(result).unwrap();
    let hexh_trimed = hex.trim_matches(char::from(0));

    String::from_utf8(hex::decode(hexh_trimed).unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::constants::TEST_PLANTUML_MIN;
    const TEST_PLANTUML_MIN_HEX: &str = "~hD34oC39aCsKoC3GoCs4oC3GuDZKsOpPZDcO0";

    #[test]
    fn it_encode_plantuml_hex() {
        assert_eq!(
            encode_plantuml_hex(String::from(TEST_PLANTUML_MIN)),
            TEST_PLANTUML_MIN_HEX
        );
    }

    #[test]
    fn it_decode_plantuml_hex() {
        assert_eq!(
            decode_plantuml_hex(String::from(TEST_PLANTUML_MIN_HEX)),
            TEST_PLANTUML_MIN
        );
    }
}
