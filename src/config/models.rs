use super::errors::ConfigError;

#[derive(Debug)]
pub struct Config {
    pub domain: String,
}

#[derive(Debug)]
pub struct PartialConfig {
    pub domain: Option<String>,
}

impl TryFrom<PartialConfig> for Config {
    type Error = ConfigError;

    fn try_from(config: PartialConfig) -> Result<Self, Self::Error> {
        let domain = config
            .domain
            .ok_or_else(|| ConfigError::MissingParameter("domain".to_string()))?;

        Ok(Self { domain })
    }
}

impl PartialConfig {
    pub fn merge(mut self, other: PartialConfig) -> Self {
        self.domain = other.domain.or(self.domain.clone());

        self
    }
}
