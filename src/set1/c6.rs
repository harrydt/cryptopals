#![allow(unused)]
use std::env;
use std::fs::File;
use std::io::Read;

/// Calculate the edit distance/Hamming distance between two byte slice
/// The Hamming distance is just the number of differing bits at the same position.
fn hamming_distance(s1: &[u8], s2: &[u8]) -> u32 {
    s1.iter()
        .zip(s2.iter())
        // for every pair of bytes from s1 and s2
        // 1. convert them to binary strings bin1 and bin2
        // 2. for every pair of bit chars in bin1 and bin2, check if they differ
        // 3. If yes, increment dist
        .fold(0_u32, |dist, (x1, x2)| {
            let bin1 = format!("{:08b}", x1);
            let bin2 = format!("{:08b}", x2);

            dist + bin1
                .chars()
                .zip(bin2.chars())
                .fold(0_u32, |d, (ch1, ch2)| if ch1 == ch2 { d } else { d + 1 })
        })
}

/// For each KEYSIZE, take the first KEYSIZE worth of bytes, and the second KEYSIZE worth of bytes,
/// and find the edit distance between them.
/// Normalize this result by dividing by KEYSIZE.
fn hamming_distance_for_key_size(file_path: &str, size: u32) -> u32 {
    let current_dir = env::current_dir().unwrap();
    let full_path = current_dir.join(file_path);
    let mut file = File::open(full_path).unwrap();

    // Create a buffer to store the bytes.
    let mut first = vec![0; size as usize];
    let mut second = vec![0; size as usize];
    // use read_exact instead of read since we know the file and the exact bytes we need
    file.read_exact(&mut first).unwrap();
    file.read_exact(&mut second).unwrap();

    hamming_distance(&first, &second) / size
}

///  The KEYSIZE with the smallest normalized edit distance is probably the key.
///  You could proceed perhaps with the smallest 2-3 KEYSIZE values.
///  Or take 4 KEYSIZE blocks instead of 2 and average the distances.
///
///  Why this works?
///  https://crypto.stackexchange.com/questions/8115/repeating-key-xor-and-hamming-distance/8118#8118
fn guess_key() -> u8 {
    (2..=40)
        .map(|n| {
            let distance = hamming_distance_for_key_size("src/set1/6.txt", n);
            (n, distance) // Pair each 'n' with its computed distance
        })
        .min_by_key(|&(_n, distance)| distance) // Find the minimum based on the distance
        .map(|(n, _distance)| n) // Extract the 'n' part of the pair
        .unwrap() as u8 // Convert to u8 and unwrap       .unwrap()
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn hamming_distance_works() {
        assert_eq!(
            37,
            hamming_distance("this is a test".as_bytes(), "wokka wokka!!!".as_bytes())
        );
    }

    #[test]
    fn guess_key_works() {
        assert_eq!(2, guess_key());
    }
}
