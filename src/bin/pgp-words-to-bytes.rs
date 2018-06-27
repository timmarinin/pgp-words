extern crate pgp_words;

use std::io::Read;

fn main() {
    let mut stdin = std::io::stdin();
    let mut s = String::new();
    stdin.read_to_string(&mut s).unwrap();
    let w = tokenize(&s);
    let bytes = pgp_words::to_bytes(&w);
    if let Some(bytes) = bytes {
        for b in bytes {
            print!("{:02X}", b);
        }
        println!("");
    }
}

fn tokenize(s: &String) -> Vec<String> {
    let l = s.to_lowercase();
    let mut v = Vec::new();
    let mut current_word = String::new();

    for c in l.chars() {
        if c <= 'z' && c >= 'a' {
            current_word.push(c.clone());
        } else {
            let word = format!("{}", current_word);
            v.push(word);
            current_word = String::new();
        }
    }
    v
}
