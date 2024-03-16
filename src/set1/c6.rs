#![allow(unused)]

/// Calculate the edit distance/Hamming distance between two strings.
/// The Hamming distance is just the number of differing bits.
fn hamming_distance(s1: &str, s2: &str) -> u32 {
    s1.as_bytes()
        .iter()
        .zip(s2.as_bytes().iter())
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

#[cfg(test)]
pub mod tests {
    use super::*;
    #[test]
    fn hamming_distance_works() {
        assert_eq!(37, hamming_distance("this is a test", "wokka wokka!!!"));
    }
}
