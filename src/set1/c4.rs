#![allow(unused)]
use super::c3::decipher_xor_single_byte;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn decrypt_lines(file_path: &str) -> Result<String, io::Error> {
    let current_dir = env::current_dir()?;
    let full_path = current_dir.join(file_path);
    let file = File::open(full_path)?;
    let reader = BufReader::new(file);
    let mut decrypted = String::new();
    let mut max_score = f64::MIN;

    for line in reader.lines() {
        let content = line?;
        let (deciphered, score) = decipher_xor_single_byte(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        if score > max_score {
            max_score = score;
            decrypted = deciphered;
        }
    }

    if decrypted.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "No decrypted content found.",
        ));
    }
    Ok(decrypted)
}

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn decrypt_lines_valid() {
        let decrypted = decrypt_lines("src/set1/4.txt").unwrap();
        assert_eq!("Now that the party is jumping\n", decrypted);
    }
}
