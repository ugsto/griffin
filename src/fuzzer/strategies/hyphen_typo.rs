use crate::{domain::prelude::*, fuzzer::traits::DomainFuzzer};
pub struct HyphenTypoFuzzerStrategy;

impl DomainFuzzer for HyphenTypoFuzzerStrategy {
    fn fuzz<'a>(domain: &'a Domain) -> Box<dyn Iterator<Item = String> + 'a> {
        Box::new(domain.domain().char_indices().skip(1).map(move |(i, c)| {
            format!(
                "{}-{}{}.{}",
                &domain.domain()[..i],
                c,
                &domain.domain()[i + 1..],
                domain.top_level_domain()
            )
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

        let fuzz = HyphenTypoFuzzerStrategy::fuzz(&domain).collect::<Vec<_>>();
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

        let fuzz = HyphenTypoFuzzerStrategy::fuzz(&domain).collect::<Vec<_>>();
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
    fn test_hyphen_typo_fuzzer_with_single_char() {
        let domain = Domain::try_from("x.com").unwrap();

        let fuzz = HyphenTypoFuzzerStrategy::fuzz(&domain).collect::<Vec<_>>();
        let expected = [];

        assert_eq!(
            HashSet::<&String>::from_iter(&fuzz),
            HashSet::<&String>::from_iter(&expected)
        );
        assert_eq!(fuzz.len(), expected.len());
    }
}
