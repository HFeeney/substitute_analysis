use rayon::prelude::*;
/**
 * This module represents a simple substitution cipher.
 */
use std::collections::HashMap;

pub struct SubCipher {
    to_ciphertext: HashMap<char, char>,
    to_plaintext: HashMap<char, char>,
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
        let mut to_ciphertext = HashMap::new();
        let mut to_plaintext = HashMap::new();
        shifter
            .chars()
            .zip('a'..='z')
            .for_each(|(replace, original)| {
                to_ciphertext.insert(original, replace);
                to_plaintext.insert(replace, original);
            });

        Self {
            to_ciphertext,
            to_plaintext,
        }
    }

    pub fn encrypt(&self, plaintext: &str) -> String {
        let mut output = String::with_capacity(plaintext.len());

        plaintext.chars().for_each(|c| {
            if c.is_ascii_lowercase() {
                output.push(*self.to_ciphertext.get(&c).unwrap());
            } else {
                output.push(c);
            }
        });
        
        output
    }

    pub fn decrypt(&self, ciphertext: &str) -> String {
        let mut output = String::with_capacity(ciphertext.len());

        ciphertext.chars().for_each(|c| {
            if c.is_ascii_lowercase() {
                output.push(*self.to_plaintext.get(&c).unwrap());
            } else {
                output.push(c);
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
