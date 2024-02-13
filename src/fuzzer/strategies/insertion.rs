use std::collections::HashMap;

use crate::fuzzer::traits::Fuzzer;

pub struct InsertionFuzzer {
    map: HashMap<char, Vec<char>>,
}

impl InsertionFuzzer {
    pub fn new(map: HashMap<char, Vec<char>>) -> Self {
        Self { map }
    }
}

impl Fuzzer for InsertionFuzzer {
    fn fuzz<'a>(&'a self, domain: &'a str) -> Box<dyn Iterator<Item = String> + 'a> {
        let iterator = domain.char_indices().flat_map(move |(i, c)| {
            static EMPTY: &Vec<char> = &Vec::new();
            let insertions = self.map.get(&c).unwrap_or(EMPTY);

            insertions.iter().flat_map(move |insertion| {
                [
                    format!("{}{}{}", &domain[..i], insertion, &domain[i..]),
                    format!(
                        "{}{}{}{}",
                        &domain[..i],
                        &domain[i..i + 1],
                        insertion,
                        &domain[i + 1..]
                    ),
                ]
            })
        });

        Box::new(iterator)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_insertion_single_character() {
        let fuzzer = InsertionFuzzer::new(HashMap::from([('a', vec!['1', '2'])]));
        let permutations = fuzzer.fuzz("a").collect::<Vec<String>>();
        let expected = ["a1", "1a", "a2", "2a"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        assert_eq!(
            HashSet::<_>::from_iter(&permutations),
            HashSet::<_>::from_iter(&expected)
        );
        assert_eq!(permutations.len(), expected.len());
    }

    #[test]
    fn test_insertion_multiple_characters() {
        let fuzzer = InsertionFuzzer::new(HashMap::from([('a', vec!['1']), ('b', vec!['2'])]));
        let permutations = fuzzer.fuzz("ab").collect::<Vec<String>>();
        let expected = ["1ab", "a1b", "a2b", "ab2"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        assert_eq!(
            HashSet::<_>::from_iter(&permutations),
            HashSet::<_>::from_iter(&expected)
        );
        assert_eq!(permutations.len(), expected.len());
    }

    #[test]
    fn test_insertion_no_match() {
        let fuzzer = InsertionFuzzer::new(HashMap::new());
        let permutations = fuzzer.fuzz("abc").collect::<Vec<String>>();
        let expected: Vec<String> = Vec::new(); // No permutations expected

        assert_eq!(permutations, expected);
    }

    #[test]
    fn test_insertion_empty_string() {
        let fuzzer = InsertionFuzzer::new(HashMap::from([('a', vec!['1', '2'])]));
        let permutations = fuzzer.fuzz("").collect::<Vec<String>>();
        let expected: Vec<String> = Vec::new(); // No permutations expected for empty string

        assert_eq!(permutations, expected);
    }

    #[test]
    fn test_insertion_with_multiple_options() {
        let fuzzer = InsertionFuzzer::new(HashMap::from([
            ('a', vec!['1', '2']),
            ('b', vec!['3', '4']),
        ]));
        let permutations = fuzzer.fuzz("ab").collect::<Vec<String>>();
        let expected = ["ab3", "2ab", "a3b", "a1b", "1ab", "a4b", "ab4", "a2b"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        assert_eq!(
            HashSet::<_>::from_iter(&permutations),
            HashSet::<_>::from_iter(&expected)
        );
        assert_eq!(permutations.len(), expected.len());
    }
}
