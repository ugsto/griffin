use crate::{fuzzer::traits::DomainFuzzer, Domain};
use itertools::Itertools;

#[derive(Debug, Default)]
pub struct BitsquattingFuzzerStrategy;

impl DomainFuzzer for BitsquattingFuzzerStrategy {
    fn fuzz<'a>(&self, domain: &'a Domain) -> Box<dyn Iterator<Item = String> + 'a> {
        let domain_str = domain.domain();
        let tld = domain.top_level_domain();

        Box::new(domain_str.char_indices().flat_map(move |(i, c)| {
            (0..8).filter_map(move |shift| {
                let mask = 1 << shift;
                let new_char = ((c as u8) ^ mask) as char;

                if !('0'..='9')
                    .chain('a'..='z')
                    .chain(std::iter::once('-'))
                    .contains(&new_char)
                {
                    return None;
                }

                Some(format!(
                    "{}{}{}.{}",
                    &domain_str[..i],
                    new_char,
                    &domain_str[i + 1..],
                    tld
                ))
            })
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_bitsquatting_fuzzer_with_simple_domain() {
        let domain = Domain::try_from("example.com").unwrap();

        let fuzz = BitsquattingFuzzerStrategy.fuzz(&domain).collect::<Vec<_>>();
        let expected = [
            "ezample.com",
            "exemple.com",
            "e8ample.com",
            "gxample.com",
            "axample.com",
            "exalple.com",
            "exampne.com",
            "examplm.com",
            "exampme.com",
            "examplu.com",
            "exampla.com",
            "exampld.com",
            "exaeple.com",
            "eximple.com",
            "examplg.com",
            "examqle.com",
            "examtle.com",
            "examrle.com",
            "ehample.com",
            "mxample.com",
            "uxample.com",
            "exaiple.com",
            "dxample.com",
            "exa-ple.com",
            "eyample.com",
            "epample.com",
            "exqmple.com",
            "exaople.com",
            "exam0le.com",
            "examxle.com",
            "exampde.com",
            "excmple.com",
            "examphe.com",
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
    fn test_bitsquatting_fuzzer_with_subdomain() {
        let domain = Domain::try_from("sub.example.com").unwrap();

        let fuzz = BitsquattingFuzzerStrategy.fuzz(&domain).collect::<Vec<_>>();
        let expected = [
            "sub.ezample.com",
            "sub.gxample.com",
            "seb.example.com",
            "swb.example.com",
            "sur.example.com",
            "subnexample.com",
            "sub.exemple.com",
            "sub.eximple.com",
            "sub.exam0le.com",
            "3ub.example.com",
            "cub.example.com",
            "stb.example.com",
            "sub.exqmple.com",
            "sub.dxample.com",
            "sub.exampla.com",
            "sub.uxample.com",
            "s5b.example.com",
            "sub.eyample.com",
            "sub.epample.com",
            "sub.ehample.com",
            "sub.mxample.com",
            "sub.exampld.com",
            "sub.exampde.com",
            "rub.example.com",
            "suf.example.com",
            "sub.examrle.com",
            "sub.examphe.com",
            "suc.example.com",
            "sub.exa-ple.com",
            "sub.exaople.com",
            "wub.example.com",
            "sub.exalple.com",
            "sub.examplm.com",
            "sub.e8ample.com",
            "sub.exaiple.com",
            "sqb.example.com",
            "sub.exampme.com",
            "suj.example.com",
            "sub.axample.com",
            "sub.exaeple.com",
            "qub.example.com",
            "sub.exampne.com",
            "sub.excmple.com",
            "sub.examplu.com",
            "sub.examqle.com",
            "sub.examplg.com",
            "sub.examtle.com",
            "sub.examxle.com",
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
    fn test_bitsquatting_fuzzer_with_single_char() {
        let domain = Domain::try_from("x.com").unwrap();

        let fuzz = BitsquattingFuzzerStrategy.fuzz(&domain).collect::<Vec<_>>();
        let expected = ["p.com", "8.com", "y.com", "z.com", "h.com"]
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
