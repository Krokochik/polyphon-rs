// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Pavel Petrov

use unicode_normalization::UnicodeNormalization;
use phf::phf_map;

static LATIN_MAP: phf::Map<char, char> = phf_map! {
    'a' => 'а',
    'e' => 'е',
    'o' => 'о',
    'c' => 'с',
    'x' => 'х',
    'B' => 'В',
    'M' => 'М',
    'H' => 'Н',
    'b' => 'в',
    'm' => 'м',
    'h' => 'н'
};

pub fn replace_latin(s: &str) -> String {
    s.chars()
        .map(|c| *LATIN_MAP.get(&c).unwrap_or(&c))
        .collect::<String>()
}

fn remove_noise_lowercase(s: &str) -> String {
    let mut res = String::with_capacity(s.len());
    let chars = s.chars();
    for c in chars {
        let cyrillic = c >= 'а' && c <= 'я' || c == 'ё';
        if cyrillic && c != 'ь' && c != 'ъ' {
            res.push(c);
        }
    }
    res
}

fn repair_cyrillic_diacritic_lowercase(s: &str) -> String {
    let mut res = String::with_capacity(s.len());
    let chars: Vec<char> = s.chars().collect();

    let mut iter = 0..chars.len();
    while let Some(i) = iter.next() {
        let cur = chars[i];
        if i + 1 == chars.len() {
            res.push(cur);
            continue;
        }
        let next = chars[i+1];
        if cur == 'е' && next == '\u{308}' {
            res.push('ё');
            iter.next();
        } else if cur == 'и' && next == '\u{306}' {
            res.push('й');
            iter.next();
        } else {
            res.push(cur);
        }
    }
    res
}

pub fn normalize(s: &str) -> String {
    let s = s.nfkd().collect::<String>();
    let s = s.to_lowercase();
    let s = replace_latin(&s);
    let s = repair_cyrillic_diacritic_lowercase(&s);
    let s = remove_noise_lowercase(&s);
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_latin_to_cyrillic_replacement() {
        let input = "a e o c x B M H";
        let expected = "а е о с х В М Н";

        assert_eq!(replace_latin(input), expected);
    }

    #[test]
    fn test_no_replacement_for_normal_cyrillic() {
        let input = "привет";
        assert_eq!(replace_latin(input), input);
    }

    #[test]
    fn test_noise_removing() {
        let input = "hello ,  прохорёнокъ!";
        let expected = "прохорёнок";
        assert_eq!(remove_noise_lowercase(input), expected);
    }

    #[test]
    fn test_diacritic_repairment() {
        let input = "йо́г на пе́рекрё́стке".nfkd().collect::<String>();
        let expected = "йо\u{301}г на пе\u{301}рекрё\u{301}стке";

        assert_eq!(repair_cyrillic_diacritic_lowercase(&input), expected);
    }

    #[test]
    fn test_normalization() {
        let input = "Ивановъ ВаСилий HиканорÓвич";
        let expected = "ивановвасилийниканорович";

        assert_eq!(normalize(input), expected);
    }
}
