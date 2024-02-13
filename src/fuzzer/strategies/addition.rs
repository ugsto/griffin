use crate::{domain::prelude::*, fuzzer::traits::DomainFuzzer};

pub struct AdditionFuzzerStrategy;

impl DomainFuzzer for AdditionFuzzerStrategy {
    fn fuzz<'a>(domain: &'a Domain) -> Box<dyn Iterator<Item = String> + 'a> {
        Box::new(
            ('0'..='9')
                .chain('a'..='z')
                .map(move |c| format!("{}{}.{}", domain.domain(), c, domain.top_level_domain())),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_addition_fuzzer_with_simple_domain() {
        let domain = Domain::try_from("example.com").unwrap();

        let fuzz = AdditionFuzzerStrategy::fuzz(&domain).collect::<Vec<_>>();
        let expected = [
            "example0.com",
            "example1.com",
            "example2.com",
            "example3.com",
            "example4.com",
            "example5.com",
            "example6.com",
            "example7.com",
            "example8.com",
            "example9.com",
            "examplea.com",
            "exampleb.com",
            "examplec.com",
            "exampled.com",
            "examplee.com",
            "examplef.com",
            "exampleg.com",
            "exampleh.com",
            "examplei.com",
            "examplej.com",
            "examplek.com",
            "examplel.com",
            "examplem.com",
            "examplen.com",
            "exampleo.com",
            "examplep.com",
            "exampleq.com",
            "exampler.com",
            "examples.com",
            "examplet.com",
            "exampleu.com",
            "examplev.com",
            "examplew.com",
            "examplex.com",
            "exampley.com",
            "examplez.com",
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
    fn test_addition_fuzzer_with_subdomain() {
        let domain = Domain::try_from("sub.example.com").unwrap();

        let fuzz = AdditionFuzzerStrategy::fuzz(&domain).collect::<Vec<_>>();
        let expected = [
            "sub.example0.com",
            "sub.example1.com",
            "sub.example2.com",
            "sub.example3.com",
            "sub.example4.com",
            "sub.example5.com",
            "sub.example6.com",
            "sub.example7.com",
            "sub.example8.com",
            "sub.example9.com",
            "sub.examplea.com",
            "sub.exampleb.com",
            "sub.examplec.com",
            "sub.exampled.com",
            "sub.examplee.com",
            "sub.examplef.com",
            "sub.exampleg.com",
            "sub.exampleh.com",
            "sub.examplei.com",
            "sub.examplej.com",
            "sub.examplek.com",
            "sub.examplel.com",
            "sub.examplem.com",
            "sub.examplen.com",
            "sub.exampleo.com",
            "sub.examplep.com",
            "sub.exampleq.com",
            "sub.exampler.com",
            "sub.examples.com",
            "sub.examplet.com",
            "sub.exampleu.com",
            "sub.examplev.com",
            "sub.examplew.com",
            "sub.examplex.com",
            "sub.exampley.com",
            "sub.examplez.com",
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
    fn test_addition_fuzzer_with_single_char() {
        let domain = Domain::try_from("x.com").unwrap();

        let fuzz = AdditionFuzzerStrategy::fuzz(&domain).collect::<Vec<_>>();
        let expected = [
            "x0.com", "x1.com", "x2.com", "x3.com", "x4.com", "x5.com", "x6.com", "x7.com",
            "x8.com", "x9.com", "xa.com", "xb.com", "xc.com", "xd.com", "xe.com", "xf.com",
            "xg.com", "xh.com", "xi.com", "xj.com", "xk.com", "xl.com", "xm.com", "xn.com",
            "xo.com", "xp.com", "xq.com", "xr.com", "xs.com", "xt.com", "xu.com", "xv.com",
            "xw.com", "xx.com", "xy.com", "xz.com",
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
