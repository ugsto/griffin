use super::errors::ConfigError;
use crate::domain::prelude::*;

#[derive(Debug)]
pub struct Config {
    pub domain: Domain,
    pub workers: usize,
}

#[derive(Debug)]
pub struct PartialConfig {
    pub domain: Option<Domain>,
    pub workers: Option<usize>,
}

impl TryFrom<PartialConfig> for Config {
    type Error = ConfigError;

    fn try_from(config: PartialConfig) -> Result<Self, Self::Error> {
        let domain = config
            .domain
            .ok_or_else(|| ConfigError::MissingParameter("domain".to_string()))?;
        let workers = config.workers.unwrap_or(1);

        Ok(Self { domain, workers })
    }
}

impl PartialConfig {
    pub fn merge(mut self, other: PartialConfig) -> Self {
        self.domain = other.domain.or(self.domain.clone());
        self.workers = other.workers.or(self.workers);

        self
    }
}
