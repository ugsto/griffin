use crate::{fuzzer::traits::DomainFuzzer, Domain};
use itertools::Itertools;

static SUFFIXES: [&str; 7] = ["s", "x", "z", "ch", "sh", "es", "ies"];

#[derive(Debug, Default)]
pub struct PluralFuzzerStrategy;

impl DomainFuzzer for PluralFuzzerStrategy {
    fn fuzz<'a>(&self, domain: &'a Domain) -> Box<dyn Iterator<Item = String> + 'a> {
        let tld = domain.top_level_domain();

        Box::new(
            domain
                .domain()
                .split('.')
                .enumerate()
                .flat_map(move |(i, part)| {
                    SUFFIXES.iter().map(move |s| {
                        format!(
                            "{}.{}",
                            domain
                                .domain()
                                .split('.')
                                .take(i)
                                .chain(std::iter::once(format!("{}{}", part, s).as_str()))
                                .chain(domain.domain().split('.').skip(i + 1))
                                .join("."),
                            tld
                        )
                    })
                }),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_plural_fuzzer_with_simple_domain() {
        let domain = Domain::try_from("example.com").unwrap();

        let fuzz = PluralFuzzerStrategy.fuzz(&domain).collect::<Vec<_>>();
        let expected = [
            "examples.com",
            "examplex.com",
            "examplez.com",
            "examplech.com",
            "examplesh.com",
            "examplees.com",
            "exampleies.com",
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
    fn test_plural_fuzzer_with_subdomain() {
        let domain = Domain::try_from("sub.example.com").unwrap();

        let fuzz = PluralFuzzerStrategy.fuzz(&domain).collect::<Vec<_>>();
        let expected = [
            "sub.examples.com",
            "sub.examplex.com",
            "sub.examplez.com",
            "sub.examplech.com",
            "sub.examplesh.com",
            "sub.examplees.com",
            "sub.exampleies.com",
            "subs.example.com",
            "subx.example.com",
            "subz.example.com",
            "subch.example.com",
            "subsh.example.com",
            "subes.example.com",
            "subies.example.com",
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
    fn test_plural_fuzzer_with_single_char() {
        let domain = Domain::try_from("x.com").unwrap();

        let fuzz = PluralFuzzerStrategy.fuzz(&domain).collect::<Vec<_>>();
        let expected = [
            "xs.com", "xx.com", "xz.com", "xch.com", "xsh.com", "xes.com", "xies.com",
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
}
