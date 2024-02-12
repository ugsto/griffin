pub trait Fuzzer {
    fn fuzz<'a>(&'a self, domain: &'a str) -> Box<dyn Iterator<Item = String> + 'a>;
}
