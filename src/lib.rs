//! Encoding and decoding text plantuml diagrams to facilitate communication of them through URL.
//!
//! ## Overview
//!
//! Consider the next plain text plantuml diagram:
//!
//! ```plantuml
//! @startuml
//! PUML -> RUST: HELLO
//! @enduml
//! ```
//!
//! It can be encoded to `0IO0sVz0StHXSdHrRMmAK5LDJ20jFY1ILLDKEY18HKnCJo0AG6LkP7LjR000` and with the help of the plantuml server (`https://www.plantuml.com/plantuml/uml/`) it can be shared [through URL](https://www.plantuml.com/plantuml/uml/0IO0sVz0StHXSdHrRMmAK5LDJ20jFY1ILLDKEY18HKnCJo0AG6LkP7LjR000).
//!
//! Also, it can be decoded in the opposite direction.
//!
//! Plantuml [declares support](https://plantuml.com/text-encoding) for the following compression algorithms:
//!
//! * [deflate](https://en.wikipedia.org/wiki/Deflate)
//! * [brotli](https://en.wikipedia.org/wiki/Brotli)
//! * [hex](https://en.wikipedia.org/wiki/Hexadecimal)
//!
//! But in fact, plantuml supports only `deflate` and `hex` ([`brotli` is turned off](https://forum.plantuml.net/15341/encoding-does-brotli-not-work-anymore-programatically-curl?show=15349)). So the crate supports only `deflate` and `hex` too.
//!
//! ## Installation
//!
//! In order to use this crate, you have to add it under `[dependencies]` to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! plantuml_encoding = "0.1.4"
//! ```

mod deflate;
mod errors;
mod hex;
mod tests;
mod utils;

pub use crate::deflate::{decode_plantuml_deflate, encode_plantuml_deflate};
pub use crate::errors::PlantumDecodeError;
pub use crate::hex::{decode_plantuml_hex, encode_plantuml_hex};
