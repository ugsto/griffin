use griffin::{Domain, FuzzerStrategy};

use super::{builder::ConfigBuilder, errors::ConfigError};

#[derive(Debug)]
pub struct Config {
    pub domain: Domain,
    pub workers: usize,
    pub fuzzers: Vec<FuzzerStrategy>,
}

#[derive(Debug, Default)]
pub struct PartialConfig {
    pub domain: Option<Domain>,
    pub workers: Option<usize>,
    pub fuzzers: Option<Vec<String>>,
}

impl TryFrom<PartialConfig> for Config {
    type Error = ConfigError;

    fn try_from(config: PartialConfig) -> Result<Self, Self::Error> {
        let mut builder = ConfigBuilder::new();

        if let Some(domain) = config.domain {
            builder = builder.domain(domain);
        }
        if let Some(workers) = config.workers {
            builder = builder.workers(workers);
        }
        if let Some(fuzzers) = config.fuzzers {
            builder = builder.fuzzers(fuzzers);
        }

        builder.build()
    }
}

impl PartialConfig {
    pub fn merge(mut self, other: PartialConfig) -> Self {
        self.domain = other.domain.or(self.domain.clone());
        self.workers = other.workers.or(self.workers);
        self.fuzzers = other.fuzzers.or(self.fuzzers);

        self
    }
}
