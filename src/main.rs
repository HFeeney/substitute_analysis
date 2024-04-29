mod sub_cipher;
mod freq_analysis;
mod load_data;

use std::io;
use std::vec::Vec;
use std::string::String;
use std::collections::HashMap;
use rand;

use crate::freq_analysis::*;
use crate::sub_cipher::*;
use crate::load_data::*;

use rayon::prelude::*;
use dashmap::DashMap;
use itertools::Itertools;
use lingua::Language::{English, German, French, Spanish};
use lingua::LanguageDetector;
use lingua::LanguageDetectorBuilder;


/**
 * Program that decodes a substitution cipher using frequency analysis.
 * To accomplish this:
 * - In parallel, count the frequency of all:
 *      - characters
 *      - bigrams
 *      - trigrams
 * - Compare this to data
 *      - IDEA: use metropolis hastings or some sort of random walk. score function
 *      would compute the difference between ideal and actual distribution.
 *      - Use lingua library to evaluate whether final selected is english
 *      https://docs.rs/lingua/latest/lingua/
 *  https://crypto.stackexchange.com/questions/74457/most-efficient-way-to-crack-a-monoalphabetic-substitution-cipher-with-spacing-in
 * TODO: figure out how to use the bigram frequencies........
 */

/**
 * Score the provided shifter by evaluating how likely it was used to encrypt
 * the provided cipher text.
 */
fn score_shifter(detector: &LanguageDetector, shifter: &String, cipher_text: &String) -> usize {
    // Decrypt the cipher text with the provided shifter providing substitutions
    let plain_text = SubCipher::new(shifter.as_str()).decrypt(cipher_text);

    // Score how "english-like" the text is
    let confidence = detector.compute_language_confidence(plain_text.as_str(), English);

    (confidence * 100000.0) as usize
}

/**
 * Generates a likely shifter based on the observed character frequencies and
 * the expected character frequencies. 
 *
 * For example, e is the most common alphabetic character in English text. If
 * the cipher text's most common alphabetic character is z, then e is probably
 * being substituted for z.
 */
fn generate_likely_shifter(expected_char_freq: &HashMap<char, usize>,
        observed_char_freq: &DashMap<char, usize>) -> String {
    let mut mapping: HashMap<char, char> = HashMap::with_capacity(26);
   
    // Sort the iterators for each map, and zip them together. Store the
    // mapping from plaintext to ciphertext in mapping.
    expected_char_freq
        .iter()
        .map(|(k, v)| (*v, *k)) // Swap k, v references with v, k values
        .sorted()               // Now sort on values
        .zip(
            observed_char_freq
                .iter()
                .map(
                    |rm| -> (usize, char) {
                        let (k, v) = rm.pair();
                        (*v, *k) // Swap keys and values
                    })
                .sorted())
        .for_each(|(expected, observed)| {
            // Now map characters to those in the cipher text with similar
            // relative frequency
            mapping.insert(expected.1, observed.1); 
        });

    let mut res = String::with_capacity(26);
    ('a'..='z').for_each(|c| { res.push(*mapping.get(&c).unwrap()); });
    res
}

#[cfg(test)]
mod test {
    use dashmap::DashMap;

    #[test]
    fn test_gen_likely_shifter_matching_maps() {
        let freq_data = super::load_frequency_data();
        let char_frequencies: DashMap<char, usize> = DashMap::new();
        freq_data.character_frequencies
            .iter()
            .for_each(|(k, v)| { char_frequencies.insert(*k, *v); } );
        
        let curr_shifter = super::generate_likely_shifter(
                                &freq_data.character_frequencies,
                                &char_frequencies);

        assert_eq!(curr_shifter, "abcdefghijklmnopqrstuvwxyz".to_string());
    }

    
    #[test]
    fn test_gen_likely_shifter_rev_abc_ordered_freq() {
        let freq_data = super::load_frequency_data();
        let char_frequencies: DashMap<char, usize> = DashMap::new();
        ('a'..='z')
            .zip(1..=26)
            .for_each(|(c, f)| { char_frequencies.insert(c, f); });
    
        let curr_shifter = super::generate_likely_shifter(
                                &freq_data.character_frequencies,
                                &char_frequencies);


        assert_eq!(curr_shifter, "xgnqzljsvcepmuwhbrtyofkdia".to_string());
    }

    #[test]
    fn test_gen_likely_shifter_abc_ordered_freq() {
        let freq_data = super::load_frequency_data();
        let char_frequencies: DashMap<char, usize> = DashMap::new();
        ('a'..='z')
            .zip((1..=26).rev())
            .for_each(|(c, f)| { char_frequencies.insert(c, f); });
    
        let curr_shifter = super::generate_likely_shifter(
                                &freq_data.character_frequencies,
                                &char_frequencies);

        assert_eq!(curr_shifter, "ctmjaoqhexvknfdsyigblupwrz".to_string());
    }
}

fn main() {
    // Load frequency data
    let freq_data = load_frequency_data();

    // Get input from stdin as cipher text
    let cipher_text = io::stdin().lines()
        .map(|r| r.unwrap()).collect::<Vec<String>>().join("\n");

// Uncomment to encrypt a text
//    println!("{}", sub_cipher::SubCipher::new("zyxwvutsrqponmlkjihgfedcba").encrypt(&cipher_text));

    

    // Count the character, bigram frequency
    let char_frequencies: DashMap<char, usize> = count_char_freq(&cipher_text);

    // Record the frequency of the 20 most frequent bigrams.
    let bigram_frequencies: DashMap<String, usize> = count_bigram_freq(&cipher_text);

    // Create language detector
    let detector = LanguageDetectorBuilder::from_languages(&[
            English, French, Spanish, German
        ]).build();

    // As initial guess, choose shifter such that character frequencies aligns
    // with frequency data.
    let mut curr_shifter = generate_likely_shifter(
                            &freq_data.character_frequencies,
                            &char_frequencies);
    
    // Store this shifter and its score as the best so far
    let mut best_shifter = curr_shifter.clone();
    let mut best_score = score_shifter(&detector, &best_shifter, &cipher_text);
    let mut curr_score = best_score;

    // Loop for some amount of time
    for _ in 0..100 {
        // Create a new proposal shifter by randomly swapping two characters
        let mut proposed_shifter_vec = curr_shifter.chars().collect::<Vec<char>>();
        proposed_shifter_vec.swap(
            (rand::random::<u8>() % 26) as usize, (rand::random::<u8>() % 26) as usize);
        
        let proposed_shifter = proposed_shifter_vec.into_iter().collect::<String>();

        println!("proposed shifter {}", proposed_shifter);

        // Score this new shifter
        let proposed_score = score_shifter(&detector, &proposed_shifter, &cipher_text);
        
        // If this is the highest scoring shifter so far, save it as the best.
        if proposed_score > best_score {
            best_score = proposed_score;
            best_shifter = proposed_shifter.clone();
        }

        // If the score of this shifter is better than the previous shifter's
        // score, keep the shifter and update its score
        if proposed_score > curr_score {
            curr_shifter = proposed_shifter; 
            curr_score = proposed_score;
        }
        // TODO: Otherwise, keep it with some probability
        
        println!("Best score: {}", best_score);
    }

    // The best shifter so far is the best guess for the actual shifter.
    // Decrypt the ciphertext and output.
    let sub_cipher = SubCipher::new(&best_shifter);
    println!("Best guess decrypted:");
    println!("\x1b[31m{}", sub_cipher.decrypt(&cipher_text));
}
