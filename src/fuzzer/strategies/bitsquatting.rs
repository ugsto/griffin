use crate::fuzzer::traits::Fuzzer;

pub struct BitsquattingFuzzer;

impl BitsquattingFuzzer {
    pub fn new() -> Self {
        Self
    }
}

fn is_valid_character(c: char) -> bool {
    let is_lowercase = c.is_ascii_lowercase();
    let is_number = c.is_ascii_digit();
    let is_hyphen = c == '-';

    is_lowercase || is_number || is_hyphen
}

impl Fuzzer for BitsquattingFuzzer {
    fn fuzz<'a>(&'a self, domain: &'a str) -> Box<dyn Iterator<Item = String> + 'a> {
        let iterator = domain.char_indices().flat_map(move |(i, c)| {
            (0..8).filter_map(move |shift| {
                let mask = 1 << shift;
                let b = ((c as u8) ^ mask) as char;

                if !is_valid_character(b) {
                    return None;
                }

                Some(format!("{}{}{}", &domain[..i], b, &domain[i + 1..]))
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
    fn test_bitsquatting_with_simple_domain() {
        let fuzzer = BitsquattingFuzzer;
        let domain = "ab.com";
        let permutations = fuzzer.fuzz(domain).collect::<Vec<String>>();

        let expected = vec![
            "cb.com", "eb.com", "ib.com", "qb.com", "ac.com", "af.com", "aj.com", "ar.com",
            "abncom", "ab.bom", "ab.aom", "ab.gom", "ab.kom", "ab.som", "ab.cnm", "ab.cmm",
            "ab.ckm", "ab.cgm", "ab.col", "ab.coo", "ab.coi", "ab.coe", "ab.co-",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

        assert_eq!(permutations.len(), expected.len());
        assert_eq!(
            HashSet::<String>::from_iter(permutations),
            HashSet::<String>::from_iter(expected)
        );
    }

    #[test]
    fn test_bitsquatting_with_numeric_domain() {
        let fuzzer = BitsquattingFuzzer;
        let domain = "1a.com";
        let permutations = fuzzer.fuzz(domain).collect::<Vec<String>>();

        let expected = vec![
            "0a.com", "3a.com", "5a.com", "9a.com", "qa.com", "1c.com", "1e.com", "1i.com",
            "1q.com", "1ancom", "1a.bom", "1a.aom", "1a.gom", "1a.kom", "1a.som", "1a.cnm",
            "1a.cmm", "1a.ckm", "1a.cgm", "1a.col", "1a.coo", "1a.coi", "1a.coe", "1a.co-",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

        assert_eq!(permutations.len(), expected.len());
        assert_eq!(
            HashSet::<String>::from_iter(permutations),
            HashSet::<String>::from_iter(expected)
        );
    }

    #[test]
    fn test_bitsquatting_with_edge_case_domain() {
        let fuzzer = BitsquattingFuzzer;
        let domain = "-a.com";
        let permutations = fuzzer.fuzz(domain).collect::<Vec<String>>();

        let expected = vec![
            "ma.com", "-c.com", "-e.com", "-i.com", "-q.com", "-ancom", "-a.bom", "-a.aom",
            "-a.gom", "-a.kom", "-a.som", "-a.cnm", "-a.cmm", "-a.ckm", "-a.cgm", "-a.col",
            "-a.coo", "-a.coi", "-a.coe", "-a.co-",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

        assert_eq!(permutations.len(), expected.len());
        assert_eq!(
            HashSet::<String>::from_iter(permutations),
            HashSet::<String>::from_iter(expected)
        );
    }

    #[test]
    fn test_bitsquatting_with_invalid_characters() {
        let fuzzer = BitsquattingFuzzer;
        let domain = "a$.com";
        let permutations = fuzzer.fuzz(domain).collect::<Vec<String>>();

        let expected = vec![
            "c$.com", "e$.com", "i$.com", "q$.com", "a4.com", "ad.com", "a$ncom", "a$.bom",
            "a$.aom", "a$.gom", "a$.kom", "a$.som", "a$.cnm", "a$.cmm", "a$.ckm", "a$.cgm",
            "a$.col", "a$.coo", "a$.coi", "a$.coe", "a$.co-",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

        assert_eq!(permutations.len(), expected.len());
        assert_eq!(
            HashSet::<String>::from_iter(permutations),
            HashSet::<String>::from_iter(expected)
        );
    }
}
