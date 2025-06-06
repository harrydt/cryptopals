const ENCRYPTED: &str = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

// https://en.wikipedia.org/wiki/Letter_frequency
const LETTER_FREQ: [f64; 27] = [
    0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015, // A-G
    0.06094, 0.06966, 0.00153, 0.00772, 0.04025, 0.02406, 0.06749, // H-N
    0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056, 0.02758, // O-U
    0.00978, 0.02360, 0.00150, 0.01974, 0.00074, 0.19181, // V-Z & space char
];

/// https://crypto.stackexchange.com/questions/30209/developing-algorithm-for-detecting-plain-text-via-frequency-analysis
/// For a given string, see how well it scores against the normal frequency
/// of letters in the English language.
fn calc_freq(s: &str) -> f64 {
    let mut counts = [0u32; 27];
    let mut score = 0f64;

    // Score each char
    for c in s.chars() {
        match c {
            'a'..='z' => {
                counts[c as usize - 97] += 1;
            }
            'A'..='Z' => {
                counts[c as usize - 65] += 1;
            }
            ' ' => {
                counts[26] += 1;
            }
            _ => {}
        }
    }

    // Sum all scores for s
    for i in 0..27 {
        score += (counts[i] as f64) * LETTER_FREQ[i];
    }

    score
}

type XorScore = (String, f64);

pub fn decipher_xor_single_byte(hex_string: &str) -> Result<XorScore, String> {
    let cipher_bytes = match hex::decode(hex_string) {
        Ok(bytes) => bytes,
        Err(_) => return Err("could not convert string to bytes".to_string()),
    };
    let mut deciphered = String::new();
    let mut max_score = f64::MIN;

    for c in 0..=255 {
        // XOR the message with every ASCII char as key
        let key_byte = c as u8;
        let msg_bytes: Vec<u8> = cipher_bytes.iter().map(|&b| b ^ key_byte).collect();

        // score each XOR'd string and update the new best if application
        let msg = String::from_utf8_lossy(&msg_bytes);
        let score = calc_freq(&msg);
        if score > max_score {
            max_score = score;
            deciphered = msg.into(); // Equivalent to String::from(msg)
        }
    }

    Ok((deciphered, max_score))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn xor_valid_hex_strings() {
        // freq(a) + freq(b)
        let expected = LETTER_FREQ[0] + LETTER_FREQ[1];
        assert_eq!(expected, calc_freq("ab"));
    }

    #[test]
    fn decipher_xor_single_byte_correct() {
        let deciphered = decipher_xor_single_byte(ENCRYPTED).unwrap();
        assert_eq!("Cooking MC's like a pound of bacon", deciphered.0);
    }
}
