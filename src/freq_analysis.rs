use dashmap::DashMap;
use rayon::prelude::*;

/**
 * Counts the frequency of all letters in the alphabet.
 * Returns a HashMap with entries indicating the frequency of each letter in
 * the alphabet within c_text.
 */
pub fn count_char_freq(c_text: &Vec<char>) -> DashMap<char, usize> {
    let ret: DashMap<char, usize> = DashMap::with_capacity(26);

    // Add keys for all letters of the alphabet.
    for c in 'a'..='z' {
        ret.insert(c, 0);
    }

    // Iterate over the contents in parallel, updating the map.
    c_text.par_iter().for_each(|&x| {
        ret.entry(x).and_modify(|v| *v += 1);
    });

    ret
}

// TODO: comment
pub fn count_bigram_freq(c_text: &Vec<char>) -> DashMap<String, usize> {
    let ret: DashMap<String, usize> = DashMap::with_capacity(26);

    /*
     * This method is hard. I have to iterate over all pairs within the string, but only of
     * adjacent alphabetic characters. ex: "hello it's me." -> [he el ll lo it me]
     *
     * I can definitely split on whitespace. I can also split on non-alphabetic characters.
     * Then, from what tokens remain, I need every 2 character window within those tokens.
     */
    c_text.par_windows(2)
        .for_each(|slice| {
            if !slice.iter().any(|c| c.is_whitespace()) {
                *ret.entry(slice.iter().collect::<String>()).or_insert(0) += 1;
            }
        });

    ret
}


pub fn count_trigram_freq(c_text: &Vec<char>) -> DashMap<String, usize> {
    let ret: DashMap<String, usize> = DashMap::with_capacity(26);

    c_text.par_windows(3)
        .for_each(|slice| {
            if !slice.iter().any(|c| c.is_whitespace()) {
                *ret.entry(slice.iter().collect::<String>()).or_insert(0) += 1;
            }
        });

    ret
}
