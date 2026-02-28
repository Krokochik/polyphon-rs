// SPDX-License-Identifier: MIT
// Copyright (c) 2026 Pavel Petrov

use once_cell::sync::Lazy;
use phf::{phf_map, phf_set};
use std::cmp::Reverse;

pub fn remove_repeats(s: &str) -> String {
    let mut res = String::with_capacity(s.len());
    let chars: Vec<char> = s.chars().collect();

    for i in 0..chars.len() {
        let cur = chars[i];
        if i + 1 == chars.len() {
            res.push(cur);
            continue;
        }
        let next = chars[i + 1];
        if cur == next {
            continue;
        }
        res.push(cur);
    }
    res
}

static VOWELS: phf::Set<char> = phf_set! {
    'и', 'а', 'о', 'у', 'ы', 'э', 'я', 'ё', 'е', 'ю'
};

pub fn reduce_vowels(s: &str) -> String {
    let mut enough_syllables = false;
    let chars: Vec<char> = s.chars().collect();

    let mut syllables = 0u8;
    let mut consonants = 0u8;
    let mut first_vowel_idx = None;

    for i in 0..chars.len() {
        if VOWELS.contains(&chars[i]) {
            if first_vowel_idx.is_none() {
                first_vowel_idx = Some(i);
            }
            syllables += 1;
        } else {
            consonants += 1;
        }
        if syllables >= 3 || syllables >= 1 && consonants >= 4 {
            enough_syllables = true;
            break;
        }
    }
    if !enough_syllables { return s.to_string(); }

    let mut res = String::with_capacity(s.len());
    let mut one_skipped = false;
    for i in (0..chars.len()).rev() {
        if VOWELS.contains(&chars[i]) {
            if one_skipped && i != first_vowel_idx.unwrap() { continue }
            one_skipped = true;
        }
        res.push(chars[i]);
    }
    res.chars().rev().collect()
}

static LETTERS_MAP: phf::Map<char, char> = phf_map! {
    'е' => 'а',
    'ё' => 'а',
    'и' => 'а',
    'о' => 'а',
    'ы' => 'а',
    'э' => 'а',
    'я' => 'а',
    'б' => 'п',
    'в' => 'ф',
    'г' => 'к',
    'д' => 'т',
    'з' => 'с',
    'щ' => 'ш',
    'ж' => 'ш',
    'м' => 'н',
    'ю' => 'у'
};

pub fn replace_letters(s: &str) -> String {
    s.chars()
        .map(|c| *LETTERS_MAP.get(&c).unwrap_or(&c))
        .collect()
}

static SEQUENCE_RULES: phf::Map<&'static str, &'static str> = phf_map! {
    "ака" => "афа",
    "ан" => "н",
    "зч" => "ш",
    "лнц" => "нц",
    "лфстф" => "лстф",
    "нат" => "н",
    "нтц" => "нц",
    "нт" => "н",
    "нта" => "на",
    "нтк" => "нк",
    "нтс" => "нс",
    "нтск" => "нск",
    "нтш" => "нш",
    "око" => "офо",
    "пал" => "пл",
    "ртч" => "рч",
    "ртц" => "рц",
    "сп" => "сф",
    "тся" => "ц",
    "стл" => "сл",
    "стн" => "сн",
    "сч" => "ш",
    "сш" => "ш",
    "тат" => "т",
    "тса" => "ц",
    "таф" => "тф",
    "тс" => "тц",
    "тц" => "ц",
    "тч" => "ч",
    "фак" => "фк",
    "фстф" => "стф",
    "шч" => "ч"
};

struct Pattern {
    chars: Box<[char]>,
    len: usize,
    repl: &'static str,
}

static PATTERNS: Lazy<Vec<Pattern>> = Lazy::new(|| {
    let mut v: Vec<Pattern> = SEQUENCE_RULES
        .keys()
        .map(|&pat| {
            let chars: Box<[char]> = pat.chars().collect();
            let len = chars.len();
            let repl = SEQUENCE_RULES.get(pat).unwrap();
            Pattern { chars, len, repl }
        })
        .collect();

    v.sort_by_key(|p| Reverse(p.len));
    v
});

pub fn replace_sequences(s: &str) -> String {
    let input: Vec<char> = s.chars().collect();
    let mut res = String::with_capacity(s.len());

    let n = input.len();
    let mut i = 0usize;
    'outer: while i < n {
        for pat in PATTERNS.iter() {
            let pat_len = pat.len;
            if pat_len == 0 || i + pat_len > n {
                continue;
            }
            if &input[i..i + pat_len] == &pat.chars[..] {
                res.push_str(pat.repl);
                i += pat_len;
                continue 'outer;
            }
        }

        res.push(input[i]);
        i += 1;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_removing_repeats() {
        let input = "алла съела змеееда";
        let expected = "ала съела змеда";

        assert_eq!(remove_repeats(input), expected);
    }

    #[test]
    fn test_letter_replacement() {
        let input = "аеёиоыэябвгдзщжмю";
        let expected = "аааааааапфктсшшну";

        assert_eq!(replace_letters(input), expected);
    }

    #[test]
    fn test_sequence_replacement() {
        let input = "факталакачаскай"; // фактологический
        let expected = "фкталафачаскай";

        assert_eq!(replace_sequences(input), expected);
    }

    #[test]
    fn test_vowels_reducing() {
        assert_eq!(reduce_vowels("молоко"), "молко");
        assert_eq!(reduce_vowels("молокозавод"), "молкзвод");
        assert_eq!(reduce_vowels("тиран"), "тиран");
        assert_eq!(reduce_vowels("квартал"), "квартал");
        assert_eq!(reduce_vowels("громофон"), "громфон");
    }
}
