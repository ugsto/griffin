use crate::{domain::prelude::*, fuzzer::traits::DomainFuzzer};

pub struct RepetitionFuzzerStrategy;

impl DomainFuzzer for RepetitionFuzzerStrategy {
    fn fuzz<'a>(domain: &'a Domain) -> Box<dyn Iterator<Item = String> + 'a> {
        let domain_str = domain.domain();
        let tld = domain.top_level_domain();

        Box::new(domain_str.char_indices().filter_map(move |(i, c)| {
            if c == '.' || domain_str.get(i + 1..=i + 1) == Some(&c.to_string()) {
                return None;
            }

            Some(format!(
                "{}{}{}.{}",
                &domain_str[..i],
                c,
                &domain_str[i..],
                tld
            ))
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_repetition_fuzzer_with_simple_domain() {
        let domain = Domain::try_from("example.com").unwrap();

        let fuzz = RepetitionFuzzerStrategy::fuzz(&domain).collect::<Vec<_>>();
        let expected = [
            "eexample.com",
            "exxample.com",
            "exaample.com",
            "exammple.com",
            "exampple.com",
            "examplle.com",
            "examplee.com",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

        assert_eq!(
            HashSet::<&String>::from_iter(&fuzz),
            HashSet::<&String>::from_iter(&expected)
        );
        assert_eq!(fuzz.len(), expected.len());
    }

    #[test]
    fn test_repetition_fuzzer_with_subdomain() {
        let domain = Domain::try_from("sub.example.com").unwrap();

        let fuzz = RepetitionFuzzerStrategy::fuzz(&domain).collect::<Vec<_>>();
        let expected = [
            "ssub.example.com",
            "suub.example.com",
            "subb.example.com",
            "sub.eexample.com",
            "sub.exxample.com",
            "sub.exaample.com",
            "sub.exammple.com",
            "sub.exampple.com",
            "sub.examplle.com",
            "sub.examplee.com",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

        assert_eq!(
            HashSet::<&String>::from_iter(&fuzz),
            HashSet::<&String>::from_iter(&expected)
        );
        assert_eq!(fuzz.len(), expected.len());
    }

    #[test]
    fn test_repetition_fuzzer_shouldnt_repeat() {
        let domain = Domain::try_from("eexample.com").unwrap();

        let fuzz = RepetitionFuzzerStrategy::fuzz(&domain).collect::<Vec<_>>();
        let expected = [
            "eeexample.com",
            "eexxample.com",
            "eexaample.com",
            "eexammple.com",
            "eexampple.com",
            "eexamplle.com",
            "eexamplee.com",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

        assert_eq!(
            HashSet::<&String>::from_iter(&fuzz),
            HashSet::<&String>::from_iter(&expected)
        );
        assert_eq!(fuzz.len(), expected.len());
    }

    #[test]
    fn test_repetition_fuzzer_with_single_char() {
        let domain = Domain::try_from("x.com").unwrap();

        let fuzz = RepetitionFuzzerStrategy::fuzz(&domain).collect::<Vec<_>>();
        let expected = ["xx.com"].iter().map(|s| s.to_string()).collect::<Vec<_>>();

        assert_eq!(
            HashSet::<&String>::from_iter(&fuzz),
            HashSet::<&String>::from_iter(&expected)
        );
        assert_eq!(fuzz.len(), expected.len());
    }
}
