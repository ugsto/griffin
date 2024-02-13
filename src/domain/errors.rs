use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainParseError {
    #[error("Missing top level domain")]
    MissingTopLevelDomain,

    #[error("Missing domain")]
    MissingDomain,
}
