use crate::{fuzzer::traits::DomainFuzzer, Domain};

#[derive(Debug, Default)]
pub struct OmissionFuzzerStrategy;

impl DomainFuzzer for OmissionFuzzerStrategy {
    fn fuzz<'a>(&self, domain: &'a Domain) -> Box<dyn Iterator<Item = String> + 'a> {
        let domain_str = domain.domain();
        let tld = domain.top_level_domain();

        Box::new(domain_str.char_indices().filter_map(move |(i, c)| {
            if domain_str.get(i + 1..=i + 1) == Some(&c.to_string()) {
                return None;
            }

            Some(format!(
                "{}{}.{}",
                &domain_str[..i],
                &domain_str[i + 1..],
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
    fn test_omission_fuzzer_with_simple_domain() {
        let domain = Domain::try_from("example.com").unwrap();

        let fuzz = OmissionFuzzerStrategy::default()
            .fuzz(&domain)
            .collect::<Vec<_>>();
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

        let fuzz = OmissionFuzzerStrategy::default()
            .fuzz(&domain)
            .collect::<Vec<_>>();
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
    fn test_omission_fuzzer_shouldnt_repeat() {
        let domain = Domain::try_from("eexample.com").unwrap();

        let fuzz = OmissionFuzzerStrategy::default()
            .fuzz(&domain)
            .collect::<Vec<_>>();
        let expected = [
            "example.com",
            "eeample.com",
            "eexmple.com",
            "eexaple.com",
            "eexamle.com",
            "eexampe.com",
            "eexampl.com",
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

        let fuzz = OmissionFuzzerStrategy::default()
            .fuzz(&domain)
            .collect::<Vec<_>>();
        let expected = [".com"].iter().map(|s| s.to_string()).collect::<Vec<_>>();

        assert_eq!(
            HashSet::<&String>::from_iter(&fuzz),
            HashSet::<&String>::from_iter(&expected)
        );
        assert_eq!(fuzz.len(), expected.len());
    }
}
