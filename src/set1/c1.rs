#![allow(dead_code)]

use base64;
use hex;

fn hex_to_base64(hex: &str) -> Result<String, &str> {
    // Decode the hex string to a Vec<u8>
    let bytes = match hex::decode(hex) {
        Ok(bytes) => bytes,
        Err(_) => return Err("Invalid hex string"),
    };

    let base64 = base64::encode(bytes);
    Ok(base64)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn valid_hex_to_base64() {
        let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
        let expected_base64 = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
        assert_eq!(expected_base64, hex_to_base64(hex).unwrap())
    }

    #[test]
    fn invalid_hex_to_base64_error() {
        let hex = "4G616e";
        assert!(hex_to_base64(hex).is_err());
    }
}
