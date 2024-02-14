use crate::config::{errors::ConfigError, models::PartialConfig, traits::PartialConfigLoader};

pub struct PartialConfigEnvLoader;

impl PartialConfigLoader for PartialConfigEnvLoader {
    fn load() -> Result<PartialConfig, ConfigError> {
        let workers = std::env::var("GRIFFIN_WORKERS")
            .ok()
            .map(|v| {
                v.parse().map_err(|_| {
                    ConfigError::ParseError(
                        "Environment variable 'GRIFFIN_WORKERS'".to_string(),
                        "usize".to_string(),
                        "Value is not a number".to_string(),
                    )
                })
            })
            .transpose()?;
        let fuzzers = std::env::var("GRIFFIN_FUZZERS").ok().map(|v| {
            v.split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect::<Vec<_>>()
        });

        Ok(crate::config::models::PartialConfig {
            domain: None,
            workers,
            fuzzers,
        })
    }
}
