use std::io::Read;
use std::fs::File;
use std::collections::HashMap;
fn main() {
    let mut buffer: Vec<u8> = Vec::new();
    let mut x = File::open("/usr/share/dict/cracklib-small").unwrap();
    let _ = x.read_to_end(&mut buffer);
    let s = String::from_utf8(buffer).unwrap();
    
    let mut words: Vec<String> = Vec::new();

    let mut word_buffer = String::new();
    for c in s.chars() {
        if c == '\n' {
            words.push(word_buffer);
            word_buffer = String::new();
        } else {
            word_buffer.push(c);
        }
    }

    let mut nodup_words: Vec<String> = Vec::new();
    for word in words {
        let mut assume_word = true;
        let mut w_entry: HashMap<char, bool> = HashMap::new();
        for c in word.chars() {
            if w_entry.get(&c) == Some(&true) {
                assume_word = false;
            }
            w_entry.insert(c, true);
        }
        if assume_word {
            nodup_words.push(word);
        }
    }
    let mut max_len = 0;
    let mut longest_word = String::new();
    for word in nodup_words.clone() {
        if word.len() > max_len {
            max_len = word.len();
            longest_word = word.clone();
        }
    }
    println!("The longest word in English with no repeated letters is {} at {} characters long", longest_word, max_len);
}

