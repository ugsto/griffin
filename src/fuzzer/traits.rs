use crate::Domain;

pub trait DomainFuzzer {
    fn fuzz<'a>(&self, domain: &'a Domain) -> Box<dyn Iterator<Item = String> + 'a>;
}
