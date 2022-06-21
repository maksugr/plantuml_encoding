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

pub fn append_3_bytes(b1: &u8, b2: &u8, b3: &u8) -> String {
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

fn decode_6_bit(s: String) -> u8 {
    let c = s.chars().next().unwrap() as u8;

    if s == "_" {
        return 63;
    };
    if s == "-" {
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

pub fn extract_3_bytes(s: &str) -> [u8; 3] {
    let mut chars = s.chars();

    let c1 = decode_6_bit(String::from(chars.next().unwrap()));
    let c2 = decode_6_bit(String::from(chars.next().unwrap()));
    let c3 = decode_6_bit(String::from(chars.next().unwrap()));
    let c4 = decode_6_bit(String::from(chars.next().unwrap()));

    let b1 = c1 << 2 | (c2 >> 4) & 0x3F;
    let b2 = (c2 << 4) & 0xF0 | (c3 >> 2) & 0xF;
    let b3 = (c3 << 6) & 0xC0 | c4 & 0x3F;

    [b1, b2, b3]
}
