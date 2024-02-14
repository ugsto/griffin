use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainFuzzerError {
    #[error("Domain fuzzer \"{0}\" does not exist")]
    DomainFuzzerDoesNotExist(String),
}
