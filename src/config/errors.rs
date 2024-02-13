use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Missing parameter: {0}")]
    MissingParameter(String),

    #[error("Could not parse {0} into {1}. Reason: {2}")]
    ParseError(String, String, String),
}
