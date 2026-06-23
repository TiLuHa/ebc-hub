// Encoding to prevent bytes > 240 in the byte stream, allowing 0xfa and 0xf8
// to be safely used as SOF and EOF markers.
pub fn encode_base240(value: u16) -> (u8, u8) {
    debug_assert!(
        value < 0xf0 * 0xf0 + 0xf0,
        "Value too large to encode in base240: {value}"
    );
    let h = (value / 0xf0) as u8;
    let l = (value % 0xf0) as u8;
    (h, l)
}

pub fn decode_base240(h: u8, l: u8) -> u16 {
    0xf0 * h as u16 + l as u16
}

pub fn xor_checksum(data: &[u8]) -> u8 {
    data.iter().fold(0, |acc, &b| acc ^ b)
}
