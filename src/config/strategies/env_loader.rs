use crate::config::traits::PartialConfigLoader;

pub struct PartialConfigEnvLoader;

impl PartialConfigLoader for PartialConfigEnvLoader {
    fn load() -> Result<crate::config::models::PartialConfig, crate::config::errors::ConfigError> {
        Ok(crate::config::models::PartialConfig { domain: None })
    }
}
