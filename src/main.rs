/* Program that decodes a substitution cipher using frequency analysis.
To accomplish this:
- In parallel, count the frequency of all:
     - characters
     - bigrams
     - trigrams
- Use this data to construct a most likely shifter based on character frequency
- Use metropolis hastings to evolve the shifter.
- The scoring function will compare the frequency of bigrams in decrypted text
 to the expected frequency of those bigrams in normal english text.
*/

mod freq_analysis;
mod load_data;
mod sub_cipher;

use rand;
use std::collections::HashMap;
use std::io;
use std::string::String;
use std::vec::Vec;

use crate::freq_analysis::*;
use crate::load_data::*;
use crate::sub_cipher::*;

use dashmap::DashMap;
use itertools::Itertools;

use std::process;

const KEEP_CHANCE: f64 = 0.0;
const ITERATIONS: usize = 1000;
const kB: f64 = 0.2; // Controls the influence of bigram difference in scoring
const kT: f64 = 0.9; // Controls the influence of trigram difference in scoring

/**
 * Score the provided shifter by evaluating how likely it was used to encrypt
 * the provided cipher text. Lower scores indicate higher likelihood.
 */
fn score_shifter(
    freq_data: &FrequencyData, 
    cipher_text: &[char],
    shifter: &str
) -> f64 {
    // Decrypt the ciphertext with the shifter.
    let decrypted = SubCipher::new(shifter).decrypt(&cipher_text);

    // Count the frequencies of the bigrams in the decrypted text. Compare
    // these with expected frequencies for the most common bigrams in in normal
    // english text.
    let mut bigram_diff = 0;
    let bigram_frequencies = count_bigram_freq(&decrypted);
    for (bigram, &freq) in freq_data.bigram_frequencies.iter() {
        let measured_freq = match bigram_frequencies.get(bigram) {
            Some(f) => *f,
            None => 0,
        };
        bigram_diff += abs_diff(freq, measured_freq);
    }
    
    // Count the frequencies of the trigrams in the decrypted text. Compare
    // these with expected frequencies for the most common trigrams in in normal
    // english text.
    let mut trigram_diff = 0;
    let trigram_frequencies = count_trigram_freq(&decrypted);
    for (trigram, &freq) in freq_data.trigram_frequencies.iter() {
        let measured_freq = match trigram_frequencies.get(trigram) {
            Some(f) => *f,
            None => 0,
        };
        trigram_diff += abs_diff(freq, measured_freq);
    }

    (bigram_diff as f64) * kB + (trigram_diff as f64) * kT
}

/** Returns the absolute value of the difference between a and b. */
fn abs_diff(a: usize, b: usize) -> usize {
    if a > b {
        return a - b;
    } else {
        return b - a;
    }
}

/**
 * Generates a likely shifter based on the observed character frequencies and
 * the expected character frequencies.
 *
 * For example, e is the most common alphabetic character in English text. If
 * the cipher text's most common alphabetic character is z, then e is probably
 * being substituted for z.
 */
fn generate_likely_shifter(
    expected_char_freq: &HashMap<char, usize>,
    observed_char_freq: &DashMap<char, usize>,
) -> String {
    let mut mapping: HashMap<char, char> = HashMap::with_capacity(26);

    // Sort the iterators for each map, and zip them together. Store the
    // mapping from plaintext to ciphertext in mapping.
    expected_char_freq
        .iter()
        .map(|(k, v)| (*v, *k)) // Swap k, v references with v, k values
        .sorted() // Now sort on values
        .zip(
            observed_char_freq
                .iter()
                .map(|rm| -> (usize, char) {
                    let (k, v) = rm.pair();
                    (*v, *k) // Swap keys and values
                })
                .sorted(),
        )
        .for_each(|(expected, observed)| {
            // Now map characters to those in the cipher text with similar
            // relative frequency
            mapping.insert(expected.1, observed.1);
        });

    let mut res = String::with_capacity(26);
    ('a'..='z').for_each(|c| {
        res.push(*mapping.get(&c).unwrap());
    });
    res
}

/**
 * Create a proposed new shifter based off the provided shifter. The new
 * shifter will be minimally different, and possibly the same.
 */
fn create_proposal_shifter(curr_shifter: &str) -> String {
    let mut proposed_shifter_vec = curr_shifter.chars().collect::<Vec<char>>();
    proposed_shifter_vec.swap(
        (rand::random::<u8>() % 26) as usize,
        (rand::random::<u8>() % 26) as usize,
    );
    let proposed_shifter = proposed_shifter_vec.into_iter().collect::<String>();

    proposed_shifter
}


fn main() {
    // Load frequency data
    let freq_data = load_frequency_data();

    // Get lowercase input from stdin as cipher text
    let cipher_text = io::stdin()
        .lines()
        .map(|r| r.unwrap())
        .collect::<Vec<String>>()
        .join("\n")
        .to_lowercase()
        .chars()
        .collect::<Vec<char>>();

    // Uncomment to encrypt a text
    println!("{}", sub_cipher::SubCipher::new("zyxwvutsrqponmlkjihgfedcba").encrypt(&cipher_text).iter().collect::<String>());

    // Count the character, bigram frequency
    let char_frequencies: DashMap<char, usize> = count_char_freq(&cipher_text);

    // Record the frequency of all bigrams.
    let bigram_frequencies: DashMap<String, usize> = count_bigram_freq(&cipher_text);

    // Record the frequency of all trigrams.
    let trigram_frequencies: DashMap<String, usize> = count_trigram_freq(&cipher_text);

    // As initial guess, choose shifter such that character frequencies aligns
    // with frequency data.
    let mut curr_shifter =
        generate_likely_shifter(&freq_data.character_frequencies, &char_frequencies);

    // Store this shifter and its score as the best so far
    let mut best_shifter = curr_shifter.clone();
    let mut best_score = score_shifter(
        &freq_data,
        &cipher_text,
        &curr_shifter
    );
    let mut curr_score = best_score;

    // Loop for some amount of time
    for _ in 0..ITERATIONS {
        let proposed_shifter = create_proposal_shifter(&curr_shifter); 

        // Score this new shifter
        let proposed_score = score_shifter(
            &freq_data,
            &cipher_text,
            &proposed_shifter
        );
        println!("proposed score {}", proposed_score);

        // If this is the loweset scoring shifter so far, save it as the best.
        if proposed_score < best_score {
            best_score = proposed_score;
            best_shifter = proposed_shifter.clone();
        }

        // If the score of this shifter is lower than the previous shifter's
        // score, keep the shifter and update its score
        if proposed_score < curr_score || rand::random::<f64>() < KEEP_CHANCE {
            curr_shifter = proposed_shifter;
            curr_score = proposed_score;
        }

        println!("Best score: {}", best_score);
    }

    // The best shifter so far is the best guess for the actual shifter.
    // Decrypt the ciphertext and output this result to the console.
    let sub_cipher = SubCipher::new(&best_shifter);
    println!("Best guess decrypted: {}", 
        sub_cipher.decrypt(&cipher_text).iter().collect::<String>());
}

#[cfg(test)]
mod test {
    use dashmap::DashMap;

    #[test]
    fn test_gen_likely_shifter_matching_maps() {
        let freq_data = super::load_frequency_data();
        let char_frequencies: DashMap<char, usize> = DashMap::new();
        freq_data.character_frequencies.iter().for_each(|(k, v)| {
            char_frequencies.insert(*k, *v);
        });

        let curr_shifter =
            super::generate_likely_shifter(&freq_data.character_frequencies, &char_frequencies);

        assert_eq!(curr_shifter, "abcdefghijklmnopqrstuvwxyz".to_string());
    }

    #[test]
    fn test_gen_likely_shifter_rev_abc_ordered_freq() {
        let freq_data = super::load_frequency_data();
        let char_frequencies: DashMap<char, usize> = DashMap::new();
        ('a'..='z').zip(1..=26).for_each(|(c, f)| {
            char_frequencies.insert(c, f);
        });

        let curr_shifter =
            super::generate_likely_shifter(&freq_data.character_frequencies, &char_frequencies);

        assert_eq!(curr_shifter, "xgnqzljsvcepmuwhbrtyofkdia".to_string());
    }

    #[test]
    fn test_gen_likely_shifter_abc_ordered_freq() {
        let freq_data = super::load_frequency_data();
        let char_frequencies: DashMap<char, usize> = DashMap::new();
        ('a'..='z').zip((1..=26).rev()).for_each(|(c, f)| {
            char_frequencies.insert(c, f);
        });

        let curr_shifter =
            super::generate_likely_shifter(&freq_data.character_frequencies, &char_frequencies);

        assert_eq!(curr_shifter, "ctmjaoqhexvknfdsyigblupwrz".to_string());
    }
}
