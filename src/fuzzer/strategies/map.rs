use crate::fuzzer::traits::Fuzzer;
use itertools::Itertools;
use std::collections::HashMap;

pub struct MapFuzzer {
    map: HashMap<char, Vec<&'static str>>,
}

impl MapFuzzer {
    pub fn new(map: HashMap<char, Vec<&'static str>>) -> Self {
        Self { map }
    }
}

impl Fuzzer for MapFuzzer {
    fn fuzz<'a>(&'a self, domain: &'a str) -> impl Iterator<Item = String> + 'a {
        domain
            .chars()
            .map(move |c| {
                static EMPTY: Vec<&'static str> = Vec::new();
                let alternatives = self.map.get(&c).unwrap_or(&EMPTY);
                std::iter::once(c.to_string()).chain(alternatives.iter().map(|s| s.to_string()))
            })
            .multi_cartesian_product()
            .map(|strings| strings.concat())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fuzz_single_character() {
        let fuzzer = MapFuzzer::new(HashMap::from([('a', vec!["q", "w", "e", "r", "t"])]));
        let permutations = fuzzer.fuzz("a").collect::<Vec<String>>();
        let expected = vec!["a", "q", "w", "e", "r", "t"];

        assert_eq!(permutations, expected);
    }

    #[test]
    fn test_fuzz_multiple_characters() {
        let fuzzer = MapFuzzer::new(HashMap::from([('a', vec!["q", "w", "e", "r", "t"])]));
        let permutations = fuzzer.fuzz("abc").collect::<Vec<String>>();
        let expected = vec!["abc", "qbc", "wbc", "ebc", "rbc", "tbc"]; // This expected output assumes only 'a' has mappings.

        assert_eq!(permutations, expected);
    }

    #[test]
    fn test_fuzz_empty_string() {
        let fuzzer = MapFuzzer::new(HashMap::from([('a', vec!["q", "w", "e", "r", "t"])]));
        let permutations = fuzzer.fuzz("").collect::<Vec<String>>();
        let expected = Vec::<String>::new();

        assert_eq!(permutations, expected);
    }

    #[test]
    fn test_fuzz_no_mapping() {
        let fuzzer = MapFuzzer::new(HashMap::new());
        let permutations = fuzzer.fuzz("abc").collect::<Vec<String>>();
        let expected = vec!["abc"];

        assert_eq!(permutations, expected);
    }
}
