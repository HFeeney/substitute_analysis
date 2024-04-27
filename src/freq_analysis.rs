use dashmap::DashMap;
use rayon::prelude::ParallelString;
use rayon::iter::ParallelIterator;

/**
 * Counts the frequency of all letters in the alphabet.
 * Returns a HashMap with entries indicating the frequency of each letter in
 * the alphabet within c_text.
 */
pub fn count_char_freq(c_text: &String) -> DashMap<char, usize> {
    let mut ret: DashMap<char, usize> = DashMap::with_capacity(26);

    // Add keys for all letters of the alphabet.
    for c in 'a'..='z' {
        ret.insert(c, 0);
    }

    // Use a parallel iterator over the chars.
    c_text.to_lowercase()
          .par_chars()
          .for_each(|x| { 
              ret.entry(x).and_modify(|v| *v += 1); 
          });

   ret 
}

pub fn count_bigram_freq(c_text: &String) -> DashMap<String, usize> {
    let mut ret: DashMap<String, usize> = DashMap::with_capacity(26);

    // Add keys for all letters of the alphabet.
    for c1 in 'a'..='z' {
        for c2 in 'a'..='z' {
            let mut key = String::from(c1);
            key.push(c2);
            ret.insert(key, 0);
        }
    }

    /*
     * This method is hard. I have to iterate over all pairs within the string, but only of
     * adjacent alphabetic characters. ex: "hello it's me." -> [he el ll lo it me]
     * 
     * I can definitely split on whitespace. I can also split on non-alphabetic characters.
     * Then, from what tokens remain, I need every 2 character window within those tokens.
     */
// https://stackoverflow.com/questions/51257304/creating-a-sliding-window-iterator-of-slices-of-chars-from-a-string
//    c_text.to_lowercase()
//          .for_each(|x| { 
//              ret.entry(x).and_modify(|v| *v += 1); 
//          });

   ret 
}
