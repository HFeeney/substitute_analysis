/**
 * This module loads in the character and bigram frequency data from the site:
 * https://www3.nd.edu/~busiforc/handouts/cryptography/Letter%20Frequencies.html
 */
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::string::String;

pub struct FrequencyData {
    pub character_frequencies: HashMap<char, usize>,
    pub bigram_frequencies: HashMap<String, usize>,
    pub trigram_frequencies: HashMap<String, usize>,
}

pub fn load_frequency_data() -> FrequencyData {
    let filepath = Path::new("freq_data.txt");
    let file_disp = filepath.display();

    let mut file = match File::open(filepath) {
        Err(_) => panic!("Couldn't open file {}", file_disp),
        Ok(file) => file,
    };

    let mut file_content = String::new();
    if let Err(_) = file.read_to_string(&mut file_content) {
        panic!("Failed to read file contents");
    }

    let mut character_frequencies: HashMap<char, usize> = HashMap::new();
    let mut bigram_frequencies: HashMap<String, usize> = HashMap::new();
    let mut trigram_frequencies: HashMap<String, usize> = HashMap::new();

    let mut chunks = file_content.split('#');
    
    // Read in the characters header
    chunks.next()
        .unwrap()
        .lines()
        .filter(|l| l.ends_with(')'))
        .for_each(|c_freq_line| {
            // Parse the character and frequency, adding it to the map.
            let c = c_freq_line.chars().find(|c| c.is_alphabetic()).unwrap();
            let freq = c_freq_line
                .split(&[' ', '(', ','][..]) // Split on ' '  and '('
                .nth(3)
                .unwrap() // The fourth item is the frequency
                .parse::<usize>()
                .unwrap();
            character_frequencies.insert(c, freq);
        });

    chunks.next()
        .unwrap()
        .lines()
        .filter(|l| l.ends_with(')'))
        .for_each(|bi_freq_line| {
            // Get iterator over the tokens of the line
            let mut tokens = bi_freq_line.split(&[' ', '(', ','][..]);

            // Store the second token as the bigram, the fourth as the frequency
            let bigram = tokens.nth(1).unwrap().to_string();
            let freq = tokens.nth(1).unwrap().parse::<usize>().unwrap();
            bigram_frequencies.insert(bigram, freq);
        });

    chunks.next()
        .unwrap()
        .lines()
        .filter(|l| l.ends_with(')'))
        .for_each(|tri_freq_line| {
            // Get iterator over the tokens of the line
            let mut tokens = tri_freq_line.split(&[' ', '(', ','][..]);

            // Store the second token as the bigram, the fourth as the frequency
            let trigram = tokens.nth(1).unwrap().to_string();
            let freq = tokens.nth(1).unwrap().parse::<usize>().unwrap();
            trigram_frequencies.insert(trigram, freq);
        });

    FrequencyData {
        character_frequencies,
        bigram_frequencies,
        trigram_frequencies,
    }
}
