use crate::utils::{plantuml_decode, plantuml_encode};

pub fn encode_plantuml_hex(plantuml: String) -> String {
    let hex = hex::encode(plantuml);
    let encoded_bytes = hex.as_bytes();

    let result = plantuml_encode(encoded_bytes);

    String::from("~h") + &result
}

pub fn decode_plantuml_hex(plantuml_hex: String) -> String {
    let plantuml_hex_trimmed = plantuml_hex.trim_start_matches("~h");

    let result = plantuml_decode(plantuml_hex_trimmed);

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
