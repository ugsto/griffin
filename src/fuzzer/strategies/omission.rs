use crate::{domain::prelude::*, fuzzer::traits::DomainFuzzer};

pub struct OmissionFuzzerStrategy;

impl DomainFuzzer for OmissionFuzzerStrategy {
    fn fuzz<'a>(domain: &'a Domain) -> Box<dyn Iterator<Item = String> + 'a> {
        Box::new(domain.domain().char_indices().map(move |(i, _)| {
            format!(
                "{}{}.{}",
                &domain.domain()[..i],
                &domain.domain()[i + 1..],
                domain.top_level_domain()
            )
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_omission_fuzzer_with_simple_domain() {
        let domain = Domain::try_from("example.com").unwrap();

        let fuzz = OmissionFuzzerStrategy::fuzz(&domain).collect::<Vec<_>>();
        let expected = [
            "exaple.com",
            "exampe.com",
            "eample.com",
            "exampl.com",
            "exmple.com",
            "examle.com",
            "xample.com",
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
    fn test_omission_fuzzer_with_subdomain() {
        let domain = Domain::try_from("sub.example.com").unwrap();

        let fuzz = OmissionFuzzerStrategy::fuzz(&domain).collect::<Vec<_>>();
        let expected = [
            "ub.example.com",
            "subexample.com",
            "sub.exaple.com",
            "sub.xample.com",
            "sub.exampl.com",
            "sub.eample.com",
            "sub.examle.com",
            "sub.exmple.com",
            "sub.exampe.com",
            "sb.example.com",
            "su.example.com",
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
    fn test_omission_fuzzer_with_single_char() {
        let domain = Domain::try_from("x.com").unwrap();

        let fuzz = OmissionFuzzerStrategy::fuzz(&domain).collect::<Vec<_>>();
        let expected = [".com"].iter().map(|s| s.to_string()).collect::<Vec<_>>();

        assert_eq!(
            HashSet::<&String>::from_iter(&fuzz),
            HashSet::<&String>::from_iter(&expected)
        );
        assert_eq!(fuzz.len(), expected.len());
    }
}
