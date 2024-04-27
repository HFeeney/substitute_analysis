mod sub_cipher;
mod freq_analysis;
mod load_data;

use std::io;
use std::vec::Vec;
use std::string::String;
use std::collections::HashMap;

use crate::freq_analysis::*;
use crate::sub_cipher::*;

use rayon::prelude::*;
use dashmap::DashMap;
use itertools::Itertools;

/**
 * Program that decodes a substitution cipher using frequency analysis.
 * To accomplish this:
 * - In parallel, count the frequency of all:
 *      - characters
 *      - bigrams
 *      - trigrams
 * - Compare this to data
 *      - IDEA: use aho-corasick or some sort of random walk. score function
 *      would compute the difference between ideal and actual distribution.
 *      - Use lingua library to evaluate whether final selected is english
 *      https://docs.rs/lingua/latest/lingua/
 */

/**
 * Score the provided shifter by evaluating how likely it was used to encrypt
 * the provided cipher text.
 */
fn score_shifter(shifter: &String, cipher_text: &String) -> usize {
    // Decrypt the cipher text with the provided shifter providing substitutions
    let plain_text = SubCipher::new(shifter).decrypt(cipher_text);

    // Score how "english-like" the text is

    0
}

/**
 * Todo: validate that this works 😅
 */
fn generate_likely_shifter(actual_char_freq: &HashMap<char, usize>,
        c_text_char_freq: &DashMap<char, usize>) -> String {
    // e is the most common character according to actual_char_freq
    // say z is the most common according to c_text_char_freq. Then
    // the shifter should indicate that e is replaced with z
    let mut mapping: HashMap<char, char> = HashMap::with_capacity(26);
   
    // Sort the iterators for each map, and zip them together. Store the
    // mapping from plaintext to ciphertext in mapping.
    actual_char_freq.iter().sorted().zip(
            c_text_char_freq.iter().map(
                |rm| -> (char, usize) {
                    let (a, b) = rm.pair();
                    (*a, *b)
                }).sorted())
        .for_each(|(actual, cipher)| { mapping.insert(*actual.0, cipher.0); });

    let mut res = String::with_capacity(26);
    ('a'..='z').for_each(|c| { res.push(*mapping.get(&c).unwrap()); });
    res
}

fn main() {
    // Load frequency data
    println!("Loading frequency data...");
    let freq_data = load_data::load_frequency_data();
    println!("Done loading frequency data");

    // Get input from stdin as cipher text
    let cipher_text = io::stdin().lines()
        .map(|r| r.unwrap()).collect::<Vec<String>>().join("");
    println!("Cipher text: {}", cipher_text);

    // Count the character, bigram frequency
    let char_frequencies: DashMap<char, usize> = count_char_freq(&cipher_text);

    // Record the frequency of the 20 most frequent bigrams.
    let bigram_frequencies: DashMap<String, usize> = count_bigram_freq(&cipher_text);

    // As initial guess, choose shifter such that character frequencies aligns
    // with frequency data.
    let curr_shifter = generate_likely_shifter(
                            &freq_data.character_frequencies,
                            &char_frequencies);
    // Store this shifter and its score as the best so far
    let best_shifter = curr_shifter.clone();
    let best_score = score_shifter(&best_shifter, &cipher_text);

    // Loop for some amount of time
    // Create a new proposal shifter by randomly swapping two characters
    // Score this new shifter
    // If the score is better than the previous score, keep it
    // Otherwise, keep it with some probability

    // The best shifter so far is the best guess for the actual shifter.
    // Decrypt the ciphertext and output.


//    let sub_cipher = SubCipher::new("zyxwvutsrqponmlkjihgfedcba");
//    println!("{}", sub_cipher.encrypt(&"hey".to_string()));
//
//    println!("{}", sub_cipher.decrypt(&sub_cipher.encrypt(&"data".to_string())));

}
