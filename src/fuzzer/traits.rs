pub trait Fuzzer {
    fn fuzz<'a>(&'a self, domain: &'a str) -> impl Iterator<Item = String> + 'a;
}
