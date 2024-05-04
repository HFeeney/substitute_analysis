/**
 * This module represents a simple substitution cipher.
 */
use std::vec::Vec;
use std::collections::HashSet;

pub struct SubCipher {
    to_ciphertext: Vec<char>,
    to_plaintext: Vec<char>,
}


impl SubCipher {
    pub fn new(shifter: &str) -> Self {
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

        // Generate the maps for encoding and decoding.
        let alphabet = ('a'..='z').collect::<Vec<char>>();
        let to_ciphertext = shifter.chars().collect::<Vec<char>>();
        let to_plaintext = alphabet.iter().map(
                |c| -> char {
                    alphabet[
                        to_ciphertext.iter()
                                     .position(|search_for| search_for == c)
                                     .unwrap()
                    ]
                }
            ).collect::<Vec<char>>();

        Self {
            to_ciphertext,
            to_plaintext,
        }
    }

    pub fn encrypt_to(&self, plaintext: &[char], output: &mut [char]) {
        plaintext.iter()
                 .enumerate()
                 .for_each(|(i, &c)| {
                      if c.is_ascii_lowercase() {
                          output[i] = self.to_ciphertext[c as usize - 'a' as usize];
                      } else {
                          output[i] = c;
                      }  
                  });
    }

    pub fn decrypt_to(&self, ciphertext: &[char], output: &mut [char]) {
        ciphertext.iter()
                  .enumerate()
                  .for_each(|(i, &c)| {
                      if c.is_ascii_lowercase() {
                          output[i] = self.to_plaintext[c as usize - 'a' as usize];
                      } else {
                          output[i] = c;
                      }  
                  });
    }

    pub fn encrypt(&self, plaintext: &String) -> String {
        let mut res = String::with_capacity(plaintext.capacity());

        plaintext.to_lowercase()
                 .chars()
                 .for_each(|c| {
                      if c.is_ascii_lowercase() {
                          res.push(self.to_ciphertext[c as usize - 'a' as usize]);
                      } else {
                          res.push(c);
                      }  
                  });

        res
    }

    pub fn decrypt(&self, ciphertext: &String) -> String {
        let mut res = String::with_capacity(ciphertext.capacity());

        ciphertext.to_lowercase()
                  .chars()
                  .for_each(|c| {
                      if c.is_ascii_lowercase() {
                          res.push(self.to_plaintext[c as usize - 'a' as usize]);
                      } else {
                          res.push(c);
                      }  
                  });

        res
    }
}
