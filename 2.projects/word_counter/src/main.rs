/*
Word Counter — Read a text file and print word frequencies. Learn: std::fs, iterators,
string slices, HashMap.
*/

use std::collections::HashMap;
use std::fs;

fn main() {
    let data = match fs::read_to_string("file.txt") {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Failed to read file: {}", e);
            return;
        }
    };

    println!("File:\n {}", data);

    let mut word_count: HashMap<String, i32> = HashMap::new();

    for word in data.split_ascii_whitespace() {
        let word = word.to_lowercase();
        *word_count.entry(word).or_insert(0) += 1;
    }

    for (word, count) in &word_count {
        println!("The word '{}' occurs {} times", word, count);
    }
}
