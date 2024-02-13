use crate::domain::prelude::*;

pub trait Fuzzer {
    fn fuzz<'a>(&'a self, domain: &'a Domain) -> Box<dyn Iterator<Item = String> + 'a>;
}
