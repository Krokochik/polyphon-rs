## About
It's an effective Rust implementation of phonetic Polyphon algorithm.

Original paper: [«Polyphon: An Algorithm for Phonetic String Matching in Russian Language»](https://www.researchgate.net/publication/307477428_Polyphon_An_Algorithm_for_Phonetic_String_Matching_in_Russian_Language).

Authors: Viacheslav V. Paramonov, Alexey O. Shigarov, Gennagy M. Ruzhnikov, Polina V. Belykh.

> We propose a new phonetic algorithm to string matching in Russian language without transliteration  from  Cyrillic  to  Latin  characters.  It  is  based  on  the  rules of sounds formation in Russian language.

## Usage

Add the dependency:
```toml
[dependencies]
polyphon = "1.0"
```

And then use:
```rust
use polyphon::encode;

let code = encode("литие"); // -> "лата"
```

**Note:** `encode` works on a single word and removes any non-Russian characters (including spaces). If you want to encode multiple words, split them first and encode each separately.

### Python API
There is a python wrapper for the public API. You can find it [here](https://github.com/Krokochik/polyphon-rs/blob/master/python/README.md).

### Code structure

- `src/lib.rs` — public interface;
- `src/normalize.rs` — normalization (incl. `normalize` function);
- `src/rules.rs` — other algorithm steps.