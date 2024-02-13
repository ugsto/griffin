use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Missing parameter: {0}")]
    MissingParameter(String),

    #[error("Could not parse {0} into {1}")]
    ParseError(String, String),
}
