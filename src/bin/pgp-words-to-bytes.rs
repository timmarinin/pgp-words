extern crate pgp_words;

use std::io::Read;

fn main() {
    let mut stdin = std::io::stdin();
    let mut s = String::new();
    stdin.read_to_string(&mut s);
    let w = tokenize(&s);
    let wslice: Vec<&str> = w.iter().map(String::as_ref).collect();
    let bytes = pgp_words::to_bytes(&wslice);
    if let Some(bytes) = bytes {
        for b in bytes {
            print!("{:x}", b);
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
