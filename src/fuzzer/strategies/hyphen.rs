use crate::fuzzer::traits::Fuzzer;

pub struct HyphenFuzzer;

impl HyphenFuzzer {
    pub fn new() -> Self {
        Self
    }
}

impl Fuzzer for HyphenFuzzer {
    fn fuzz<'a>(&'a self, domain: &'a str) -> impl Iterator<Item = String> + 'a {
        domain
            .char_indices()
            .skip(1)
            .map(|(i, c)| format!("{}-{}{}", &domain[..i], c, &domain[i + 1..]))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_fuzz_single_character() {
        let fuzzer = HyphenFuzzer::new();
        let permutations = fuzzer.fuzz("a").collect::<Vec<String>>();
        let expected = [];

        assert_eq!(
            HashSet::<_>::from_iter(&permutations),
            HashSet::<_>::from_iter(&expected)
        );
        assert_eq!(permutations.len(), expected.len());
    }

    #[test]
    fn test_fuzz_multiple_characters() {
        let fuzzer = HyphenFuzzer::new();
        let permutations = fuzzer.fuzz("ab").collect::<Vec<String>>();
        let expected = ["a-b"]
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
    fn test_fuzz_three_characters() {
        let fuzzer = HyphenFuzzer::new();
        let permutations = fuzzer.fuzz("abc").collect::<Vec<String>>();
        let expected = ["ab-c", "a-bc"]
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
    fn test_fuzz_four_characters() {
        let fuzzer = HyphenFuzzer::new();
        let permutations = fuzzer.fuzz("abcd").collect::<Vec<String>>();
        let expected = ["abc-d", "ab-cd", "a-bcd"]
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
