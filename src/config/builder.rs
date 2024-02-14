use griffin::{Domain, FuzzerStrategy};
use std::collections::HashSet;

use super::{
    defaults::{DEFAULT_FUZZERS, DEFAULT_WORKERS},
    errors::ConfigError,
    models::{Config, PartialConfig},
};

pub struct ConfigBuilder {
    partial_config: PartialConfig,
}

impl ConfigBuilder {
    pub fn new() -> Self {
        Self {
            partial_config: PartialConfig::default(),
        }
    }
}

impl ConfigBuilder {
    pub fn domain(mut self, domain: Domain) -> Self {
        self.partial_config.domain = Some(domain);
        self
    }

    pub fn workers(mut self, workers: usize) -> Self {
        self.partial_config.workers = Some(workers);
        self
    }

    pub fn fuzzers(mut self, fuzzers: Vec<String>) -> Self {
        self.partial_config.fuzzers = Some(fuzzers);
        self
    }

    pub fn build(self) -> Result<Config, ConfigError> {
        let domain = self
            .partial_config
            .domain
            .ok_or_else(|| ConfigError::MissingParameter("domain".to_string()))?;
        let workers = self.partial_config.workers.unwrap_or(DEFAULT_WORKERS);
        let fuzzers = self
            .partial_config
            .fuzzers
            .unwrap_or(DEFAULT_FUZZERS.iter().map(|s| s.to_string()).collect())
            .iter()
            .map(|s| s.as_str())
            .collect::<HashSet<_>>()
            .into_iter()
            .map(FuzzerStrategy::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| {
                ConfigError::ParseError(
                    "fuzzers".to_string(),
                    "array of fuzzers".to_string(),
                    e.to_string(),
                )
            })?;

        Ok(Config {
            domain,
            workers,
            fuzzers,
        })
    }
}
