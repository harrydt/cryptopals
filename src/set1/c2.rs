#![allow(unused)]

fn xor_buffers(buffer_1: &[u8], buffer_2: &[u8]) -> Result<Vec<u8>, String> {
    if buffer_1.len() != buffer_2.len() {
        return Err("buffers have different lengths".to_string());
    }

    let mut result = Vec::with_capacity(buffer_1.len());
    for (b1, b2) in buffer_1.iter().zip(buffer_2.iter()) {
        result.push(b1 ^ b2);
    }

    Ok(result)
}

/// xor_hex_strings takes two equal-length buffers and produces their XOR combination.
fn xor_hex_strings(hex_1: &str, hex_2: &str) -> Result<Vec<u8>, String> {
    let buffer_1 = hex::decode(hex_1).map_err(|e| e.to_string())?;
    let buffer_2 = hex::decode(hex_2).map_err(|e| e.to_string())?;
    xor_buffers(&buffer_1, &buffer_2)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_xor_hex_strings_with_cryptopals_example() {
        let hex_1 = "1c0111001f010100061a024b53535009181c";
        let hex_2 = "686974207468652062756c6c277320657965";
        let expected = "746865206b696420646f6e277420706c6179";

        let result = xor_hex_strings(hex_1, hex_2).expect("XOR should succeed");
        let encoded = hex::encode(result);

        assert_eq!(encoded, expected);
    }

    #[test]
    fn xor_invalid_hex_strings() {
        let got = xor_hex_strings("4G616e", "686974207468652062756c6c277320657965");
        assert!(got.is_err());
    }

    #[test]
    fn xor_unequal_hex_strings() {
        let got = xor_hex_strings(
            "1c0111001f010100061a024b53535009181cf",
            "686974207468652062756c6c277320657965",
        );
        assert!(got.is_err());
    }
}
