use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum DomainParseError {
    #[error("Missing top level domain")]
    MissingTopLevelDomain,

    #[error("Missing domain")]
    MissingDomain,
}
