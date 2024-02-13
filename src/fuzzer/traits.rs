use crate::domain::prelude::*;

pub trait DomainFuzzer {
    fn fuzz<'a>(domain: &'a Domain) -> Box<dyn Iterator<Item = String> + 'a>;
}
