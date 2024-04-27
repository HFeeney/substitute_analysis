/**
 * This module loads in the character and bigram frequency data from the site:
 * https://www3.nd.edu/~busiforc/handouts/cryptography/Letter%20Frequencies.html
 */
use std::collections::HashMap;
use std::string::String;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

pub struct FrequencyData {
    pub character_frequencies: HashMap<char, usize>,
    pub bigram_frequencies: HashMap<String, usize>, 
}


pub fn load_frequency_data() -> FrequencyData {
    let filepath = Path::new("../freq_data.txt");
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
    

    // Want iterator over the lines of character frequencies
    let mut data_itr = file_content.split("Bigrams");
    
    data_itr
        .next().unwrap() // The first chunk contains character frequencies
        .lines().filter(|l| l.ends_with(')')) // Iterate over lines that end with ')'
        .for_each(
            |l| {
                // Parse the character and frequency, adding it to the map.
                let c = l.chars().find(|c| c.is_alphabetic()).unwrap();
                let freq = l.split(&[' ', '('][..]) // Split on ' '  and '%'
                        .nth(3).unwrap() // The fourth item is the frequency
                        .parse::<usize>().unwrap();
                character_frequencies.insert(c, freq);
            });
    
    data_itr
        .next().unwrap() // The second chunk contains bigram frequencies
        .lines().filter(|l| l.ends_with(')')) // Iterate over lines that end with ')'
        .for_each(
            |l| {
                // Get iterator over the tokens of the line
                let mut tokens = l.split(&[' ', ')'][..]);
                
                // Store the second token as the bigram, the fourth as the frequency
                let bigram = tokens.nth(1).unwrap().to_string();
                let freq = tokens.nth(1).unwrap().parse::<usize>().unwrap();
                bigram_frequencies.insert(bigram, freq);
            });
    
    FrequencyData {
        character_frequencies, 
        bigram_frequencies,
    }
}
