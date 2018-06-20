# pgp-words

Crate for transforming bytes (`u8`) into [PGP words](https://en.wikipedia.org/wiki/PGP_word_list) and back.

Status: functions `to_bytes` and `to_words` are working, but need documentation.

```rust
extern crate pgp_words;

fn main() {
  let msg: [u8; 4] = [0x2D, 0x6D, 0xED, 0x27];
  let words = pgp_words::to_words(&msg);
  assert_eq!(words[0], "button");
  assert_eq!(words[1], "hazardous");
  assert_eq!(words[2], "tunnel");
  assert_eq!(words[3], "celebrate");
}
```

`to_bytes` returns `Option<Vec<u8>>`, because you can pass words that are not part of the list or that have the wrong evenness, there are no attempts to recover and `pgp_words` would return `None`.

## License

`pgp-words` is MIT licensed. 2018, Tim Marinin <mt@marinintim.com>

Words were taken from the Wikipedia entry on 2018-06-20.
