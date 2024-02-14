use crate::{fuzzer::traits::DomainFuzzer, Domain};

#[derive(Debug, Default)]
pub struct HyphenTypoFuzzerStrategy;

impl DomainFuzzer for HyphenTypoFuzzerStrategy {
    fn fuzz<'a>(&self, domain: &'a Domain) -> Box<dyn Iterator<Item = String> + 'a> {
        let domain_str = domain.domain();
        let tld = domain.top_level_domain();

        Box::new(domain_str.char_indices().skip(1).filter_map(move |(i, c)| {
            if domain_str.get(i..=i) == Some("-") {
                return None;
            }

            Some(format!(
                "{}-{}{}.{}",
                &domain_str[..i],
                c,
                &domain_str[i + 1..],
                tld
            ))
        }))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_hyphen_typo_fuzzer_with_simple_domain() {
        let domain = Domain::try_from("example.com").unwrap();

        let fuzz = HyphenTypoFuzzerStrategy::default()
            .fuzz(&domain)
            .collect::<Vec<_>>();
        let expected = [
            "exampl-e.com",
            "examp-le.com",
            "exam-ple.com",
            "exa-mple.com",
            "ex-ample.com",
            "e-xample.com",
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
    fn test_hyphen_typo_fuzzer_with_subdomain() {
        let domain = Domain::try_from("sub.example.com").unwrap();

        let fuzz = HyphenTypoFuzzerStrategy::default()
            .fuzz(&domain)
            .collect::<Vec<_>>();
        let expected = [
            "su-b.example.com",
            "sub.-example.com",
            "sub-.example.com",
            "sub.exampl-e.com",
            "sub.e-xample.com",
            "sub.ex-ample.com",
            "sub.exam-ple.com",
            "s-ub.example.com",
            "sub.exa-mple.com",
            "sub.examp-le.com",
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
    fn test_hyphen_typo_fuzzer_shouldnt_repeat() {
        let domain = Domain::try_from("exampl-e.com").unwrap();

        let fuzz = HyphenTypoFuzzerStrategy::default()
            .fuzz(&domain)
            .collect::<Vec<_>>();
        let expected = [
            "e-xampl-e.com",
            "ex-ampl-e.com",
            "exa-mpl-e.com",
            "exam-pl-e.com",
            "examp-l-e.com",
            "exampl--e.com",
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
    fn test_hyphen_typo_fuzzer_with_single_char() {
        let domain = Domain::try_from("x.com").unwrap();

        let fuzz = HyphenTypoFuzzerStrategy::default()
            .fuzz(&domain)
            .collect::<Vec<_>>();
        let expected = [];

        assert_eq!(
            HashSet::<&String>::from_iter(&fuzz),
            HashSet::<&String>::from_iter(&expected)
        );
        assert_eq!(fuzz.len(), expected.len());
    }
}
