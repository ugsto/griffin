use super::{errors::ConfigError, models::PartialConfig};

pub trait PartialConfigLoader {
    fn load() -> Result<PartialConfig, ConfigError>;
}
