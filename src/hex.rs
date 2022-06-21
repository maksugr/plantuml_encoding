pub fn encode_plantuml_hex<T: AsRef<str>>(plantuml: T) -> String {
    let hex = hex::encode(plantuml.as_ref());

    String::from("~h") + &hex
}

pub fn decode_plantuml_hex<T: AsRef<str>>(plantuml_hex: T) -> String {
    let plantuml_hex_trimmed = plantuml_hex.as_ref().trim_start_matches("~h");

    String::from_utf8(hex::decode(plantuml_hex_trimmed).unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::{decode_plantuml_hex, encode_plantuml_hex};

    use crate::tests::constants::{
        plantuml_hex_str::{PLANTUML_HEX_LARGE, PLANTUML_HEX_SMALL},
        plantuml_str::{PLANTUML_LARGE, PLANTUML_SMALL},
    };

    #[test]
    fn it_encode_plantuml_hex_small() {
        assert_eq!(encode_plantuml_hex(PLANTUML_SMALL), PLANTUML_HEX_SMALL);
    }

    #[test]
    fn it_encode_plantuml_hex_small_string() {
        assert_eq!(
            encode_plantuml_hex(String::from(PLANTUML_SMALL)),
            PLANTUML_HEX_SMALL
        );
    }

    #[test]
    fn it_decode_plantuml_hex_small() {
        assert_eq!(decode_plantuml_hex(PLANTUML_HEX_SMALL), PLANTUML_SMALL);
    }

    #[test]
    fn it_decode_plantuml_hex_small_string() {
        assert_eq!(
            decode_plantuml_hex(String::from(PLANTUML_HEX_SMALL)),
            PLANTUML_SMALL
        );
    }

    #[test]
    fn it_encode_plantuml_hex_large() {
        assert_eq!(encode_plantuml_hex(PLANTUML_LARGE), PLANTUML_HEX_LARGE);
    }

    #[test]
    fn it_decode_plantuml_hex_large() {
        assert_eq!(decode_plantuml_hex(PLANTUML_HEX_LARGE), PLANTUML_LARGE);
    }
}
