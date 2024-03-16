#![allow(unused)]
use super::c2::hex_string_to_bytes;
use hex;

fn repeating_key_xor(msg: &str, key: &str) -> String {
    // Create len(msg)/len(key) cycles where each cycle is [chars in key]
    // Then collect the cycles as one continous String
    let keys = key.chars().cycle().take(msg.len()).collect::<String>();

    let msg_bytes = hex_string_to_bytes(&hex::encode(msg)).unwrap();
    let keys_bytes = hex_string_to_bytes(&hex::encode(keys)).unwrap();

    // this works too
    // let msg_bytes = msg.as_bytes();
    // let keys_bytes = keys.as_bytes();

    let xor_bytes = msg_bytes
        .iter()
        .zip(keys_bytes.iter())
        .map(|(b1, b2)| b1 ^ b2)
        .collect::<Vec<u8>>();

    hex::encode(xor_bytes)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn repreating_key_xor_works() {
        let msg = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
        let want = "0b3637272a2b2e63622c2e69692a23693a2a3c6324202d623d63343c2a26226324272765272a282b2f20430a652e2c652a3124333a653e2b2027630c692b20283165286326302e27282f";
        let got = repeating_key_xor(msg, "ICE");
        assert_eq!(want, got);
    }
}
