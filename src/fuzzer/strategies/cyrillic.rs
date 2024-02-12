use std::collections::HashMap;

use itertools::Itertools;

use crate::fuzzer::traits::Fuzzer;

pub struct CyrillicFuzzer {
    cyrillic_map: HashMap<char, &'static str>,
}

impl CyrillicFuzzer {
    pub fn new() -> Self {
        let cyrillic_map = HashMap::from([
            ('a', "аӓӑ"),
            ('b', "ьъ"),
            ('c', "сҫ"),
            ('d', "ԁд"),
            ('e', "еёэ"),
            ('g', "ԍг"),
            ('h', "һн"),
            ('i', "і"),
            ('j', "ј"),
            ('k', "кқ"),
            ('l', "ӏ"),
            ('m', "м"),
            ('n', "и"),
            ('o', "оӧө"),
            ('p', "рҏ"),
            ('q', "ԛ"),
            ('r', "л"),
            ('s', "ѕ"),
            ('t', "т"),
            ('v', "ѵ"),
            ('w', "ԝш"),
            ('x', "хҳ"),
            ('y', "уүұ"),
        ]);

        Self { cyrillic_map }
    }
}

impl Fuzzer for CyrillicFuzzer {
    fn fuzz<'a>(&'a self, domain: &'a str) -> impl Iterator<Item = String> + 'a {
        let domains = domain
            .chars()
            .map(|c| std::iter::once(c).chain(self.cyrillic_map.get(&c).unwrap_or(&"").chars()));

        let multi_prod = domains.multi_cartesian_product();

        multi_prod.map(|chars| chars.into_iter().collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzz_single_character() {
        let fuzzer = CyrillicFuzzer::new();
        let fuzzed_chars: Vec<String> = fuzzer.fuzz("a").collect();
        let expected = vec!["a", "а", "ӓ", "ӑ"];

        assert_eq!(fuzzed_chars, expected);
    }

    #[test]
    fn test_fuzz_multiple_characters() {
        let fuzzer = CyrillicFuzzer::new();
        let fuzzed_chars: Vec<String> = fuzzer.fuzz("abc").collect();
        let expected = vec![
            "abc", "abс", "abҫ", "aьc", "aьс", "aьҫ", "aъc", "aъс", "aъҫ", "аbc", "аbс", "аbҫ",
            "аьc", "аьс", "аьҫ", "аъc", "аъс", "аъҫ", "ӓbc", "ӓbс", "ӓbҫ", "ӓьc", "ӓьс", "ӓьҫ",
            "ӓъc", "ӓъс", "ӓъҫ", "ӑbc", "ӑbс", "ӑbҫ", "ӑьc", "ӑьс", "ӑьҫ", "ӑъc", "ӑъс", "ӑъҫ",
        ];

        assert_eq!(fuzzed_chars, expected);
    }

    #[test]
    fn test_fuzz_empty_string() {
        let fuzzer = CyrillicFuzzer::new();
        let fuzzed_chars: Vec<String> = fuzzer.fuzz("").collect();
        let expected = Vec::<String>::new();

        assert_eq!(fuzzed_chars, expected);
    }

    #[test]
    fn test_fuzz_non_cyrillic_character() {
        let fuzzer = CyrillicFuzzer::new();
        let fuzzed_chars: Vec<String> = fuzzer.fuzz("123").collect();
        let expected = vec!["123"];

        assert_eq!(fuzzed_chars, expected);
    }
}
