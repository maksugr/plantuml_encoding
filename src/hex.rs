use crate::errors;

/// Encode plantuml to hex
/// (with [additional prefix `~h`](https://plantuml.com/text-encoding))
///
/// ## Example
///
/// ```rust
/// use plantuml_encoding::{encode_plantuml_hex, FromPlantumlError};
///
/// fn main() -> Result<(), FromPlantumlError> {
///     let encoded_hex = encode_plantuml_hex("@startuml\nPUML -> RUST\n@enduml")?;
///
///     assert_eq!(encoded_hex, "~h407374617274756d6c0a50554d4c202d3e20525553540a40656e64756d6c");
///
///     Ok(())
/// }
/// ```
pub fn encode_plantuml_hex<T: AsRef<str>>(
    plantuml: T,
) -> Result<String, errors::FromPlantumlError> {
    let hex = hex::encode(plantuml.as_ref());

    Ok(String::from("~h") + &hex)
}

/// Decode plantuml from hex
/// (with [additional prefix `~h`](https://plantuml.com/text-encoding))
///
/// ## Example
///
/// ```rust
/// use plantuml_encoding::{decode_plantuml_hex, FromPlantumlError};
///
/// fn main() -> Result<(), FromPlantumlError> {
///     let decoded_hex = decode_plantuml_hex("~h407374617274756d6c0a50554d4c202d3e20525553540a40656e64756d6c")?;
///
///     assert_eq!(decoded_hex, "@startuml\nPUML -> RUST\n@enduml");
///
///     Ok(())
/// }
/// ```
pub fn decode_plantuml_hex<T: AsRef<str>>(
    plantuml_hex: T,
) -> Result<String, errors::FromPlantumlError> {
    let plantuml_hex_trimmed = plantuml_hex.as_ref().trim_start_matches("~h");

    let decoded_bytes = hex::decode(plantuml_hex_trimmed)?;

    Ok(String::from_utf8(decoded_bytes)?)
}

#[cfg(test)]
mod tests {
    use super::{decode_plantuml_hex, encode_plantuml_hex};

    use crate::errors;
    use crate::tests::constants::{
        plantuml_hex_str::{PLANTUML_HEX_LARGE, PLANTUML_HEX_SMALL},
        plantuml_str::{PLANTUML_LARGE, PLANTUML_SMALL},
    };

    #[test]
    fn it_encode_plantuml_hex_small() {
        assert_eq!(
            encode_plantuml_hex(PLANTUML_SMALL),
            Ok(PLANTUML_HEX_SMALL.to_string())
        );
    }

    #[test]
    fn it_encode_plantuml_hex_small_string() {
        assert_eq!(
            encode_plantuml_hex(String::from(PLANTUML_SMALL)),
            Ok(PLANTUML_HEX_SMALL.to_string())
        );
    }

    #[test]
    fn it_decode_plantuml_hex_small() {
        assert_eq!(
            decode_plantuml_hex(PLANTUML_HEX_SMALL),
            Ok(PLANTUML_SMALL.to_string())
        );
    }

    #[test]
    fn it_decode_plantuml_hex_small_string() {
        assert_eq!(
            decode_plantuml_hex(String::from(PLANTUML_HEX_SMALL)),
            Ok(PLANTUML_SMALL.to_string())
        );
    }

    #[test]
    fn it_encode_plantuml_hex_large() {
        assert_eq!(
            encode_plantuml_hex(PLANTUML_LARGE),
            Ok(PLANTUML_HEX_LARGE.to_string())
        );
    }

    #[test]
    fn it_decode_plantuml_hex_large() {
        assert_eq!(
            decode_plantuml_hex(PLANTUML_HEX_LARGE),
            Ok(PLANTUML_LARGE.to_string())
        );
    }

    #[test]
    fn it_decode_plantuml_hex_regular_error() {
        assert_eq!(
            decode_plantuml_hex("12345"),
            Err(errors::FromPlantumlError(
                "there is a problem during hex decoding: `Odd number of digits`".to_string()
            ))
        );
    }
}
