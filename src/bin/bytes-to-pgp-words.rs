extern crate pgp_words;

use std::io::Read;

fn main() {
    let mut stdin = std::io::stdin();
    let mut s = String::new();
    stdin.read_to_string(&mut s).unwrap();
    let w = tokenize(&s);
    let words = pgp_words::to_words(&w);
    
    if words.len() > 0 {
        print!("{}", words[0]);
        for i in 1..words.len() {
            print!("-{}", words[i]);
        }
    }
    println!("");
}

fn tokenize(s: &String) -> Vec<u8> {
    let l = s.to_lowercase();
    let mut v = Vec::new();
    let mut current_word = String::new();

    for c in l.chars() {
        if current_word.len() == 2 {
            let byte = u8::from_str_radix(&current_word, 16);
            if let Ok(byte) = byte {
                v.push(byte);
            }
            current_word = String::new();
        }

        if (c <= 'z' && c >= 'A') || (c >= '0' && c <= '9') {
            current_word.push(c.clone());
        }
    }
    v
}
