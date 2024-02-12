use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Missing parameter: {0}")]
    MissingParameter(String),
}
