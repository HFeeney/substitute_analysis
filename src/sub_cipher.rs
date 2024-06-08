use rayon::prelude::*;
/**
 * This module represents a simple substitution cipher.
 */
use std::collections::HashMap;

pub struct SubCipher {
    to_cipher_text: HashMap<char, char>,
    to_plain_text: HashMap<char, char>,
}

impl SubCipher {
    pub fn new(shifter: &str) -> Self {
        /* Disabled for speed.
        // Verify that the shifter has no duplicates and is 26 characters long
        // with only alphabetic characters.
        if shifter.len() != 26 {
            panic!("Incorrect shifter length!");
        }
        let charset = shifter.chars().collect::<HashSet<char>>();
        if charset.len() != 26 {
            panic!("Shifter must contain unique characters!");
        }
        if !charset.eq(&('a'..='z').collect::<HashSet<char>>()) {
            panic!("Shifter must contain only lowercase alphabet characters!");
        }
        */

        // Generate the maps for encoding and decoding.
        let mut to_cipher_text = HashMap::new();
        let mut to_plain_text = HashMap::new();
        shifter
            .chars()
            .zip('a'..='z')
            .for_each(|(replace, original)| {
                to_cipher_text.insert(original, replace);
                to_plain_text.insert(replace, original);
            });

        Self {
            to_cipher_text,
            to_plain_text,
        }
    }

    pub fn encrypt(&self, plain_text: &[char]) -> Vec<char> {
        let mut output = vec![' '; plain_text.len()];

        plain_text.iter()
                 .enumerate()
                 .for_each(|(idx, c)| {
                     if c.is_ascii_lowercase() {
                         output[idx] = *self.to_cipher_text.get(&c).unwrap();
                     } else {
                         output[idx] = *c;
                     }
                 });
        
        output
    }

    pub fn decrypt(&self, cipher_text: &[char]) -> Vec<char> {
        let mut output = vec![' '; cipher_text.len()];

        cipher_text.iter()
                 .enumerate()
                 .for_each(|(idx, c)| {
                     if c.is_ascii_lowercase() {
                         output[idx] = *self.to_plain_text.get(&c).unwrap();
                     } else {
                         output[idx] = *c;
                     }
                 });
        
        output
    }
}

// #[cfg(test)]
// pub mod test {
//     use super::*;
// 
//     #[test]
//     fn 
// }
