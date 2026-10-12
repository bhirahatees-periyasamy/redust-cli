#[derive(Default)]
pub struct Codec;

pub trait CodecOperations {
    fn new() -> Self;
    fn encode(&self, value: String) -> Vec<u8>;
    fn decode(&self, bytes: &[u8]) -> String;
}

impl CodecOperations for Codec {
    fn new() -> Self {
        Self
    }

    fn encode(&self, value: String) -> Vec<u8> {
        value.into_bytes()
    }

    fn decode(&self, bytes: &[u8]) -> String {
        String::from_utf8_lossy(bytes).into_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_returns_utf8_bytes() {
        let codec = Codec::new();
        assert_eq!(codec.encode("abc".to_string()), vec![97, 98, 99]);
    }

    #[test]
    fn encode_empty_string_gives_no_bytes() {
        let codec = Codec::new();
        assert_eq!(codec.encode(String::new()), Vec::<u8>::new());
    }

    #[test]
    fn multibyte_characters_use_more_than_one_byte() {
        let codec = Codec::new();
        // 'é' is two bytes in UTF-8
        assert_eq!(codec.encode("é".to_string()), vec![0xC3, 0xA9]);
    }

    #[test]
    fn roundtrip_preserves_value() {
        let codec = Codec::new();
        for s in ["", "hello", "with space", "line\nbreak", "日本語 🦀"] {
            let bytes = codec.encode(s.to_string());
            assert_eq!(codec.decode(&bytes), s);
        }
    }

    #[test]
    fn decode_replaces_invalid_utf8() {
        let codec = Codec::new();
        // 0xFF is never valid in UTF-8
        assert_eq!(codec.decode(&[b'a', 0xFF, b'b']), "a\u{FFFD}b");
    }

    #[test]
    fn decode_empty_slice() {
        let codec = Codec::new();
        assert_eq!(codec.decode(&[]), "");
    }
}
