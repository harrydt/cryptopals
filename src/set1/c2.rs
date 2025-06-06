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
    let buffer_1 = hex_string_to_bytes(hex_1);
    let buffer_2 = hex_string_to_bytes(hex_2);

    match (buffer_1, buffer_2) {
        (Some(b1), Some(b2)) => xor_buffers(&b1, &b2),
        _ => Err("failed to convert hex string to bytes".to_string()),
    }
}

pub fn hex_string_to_bytes(hex_string: &str) -> Option<Vec<u8>> {
    // Check if the input string has an even number of characters
    if hex_string.len() % 2 != 0 {
        return None;
    }

    // Use the `from_str_radix` function to parse the hex string
    let bytes: Result<Vec<u8>, _> = (0..hex_string.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex_string[i..i + 2], 16))
        .collect();

    bytes.ok()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn xor_valid_hex_strings() {
        let expected = hex_string_to_bytes("746865206b696420646f6e277420706c6179").unwrap();
        let got = xor_hex_strings(
            "1c0111001f010100061a024b53535009181c",
            "686974207468652062756c6c277320657965",
        )
        .unwrap();
        assert_eq!(expected, got);
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
