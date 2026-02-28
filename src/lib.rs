// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Pavel Petrov

use crate::normalize::normalize;
use crate::rules::*;

pub mod normalize;
pub mod rules;

pub fn encode(s: &str) -> String {
    let s = normalize(s);
    let s = remove_repeats(&s);
    let s = reduce_vowels(&s);
    let s = replace_letters(&s);
    let s = replace_sequences(&s);
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encoding() {
        assert_eq!(encode("Литие"), "лата");
        assert_eq!(encode("ладо"), "лата");
        assert_eq!(encode("литье"), "лата");
        assert_eq!(encode("летие"), "лата");
        assert_eq!(encode("лeто"), "лата");
        assert_eq!(encode("леди"), "лата");
    }
}
