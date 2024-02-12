use clap::Parser;

use crate::config::traits::PartialConfigLoader;

#[derive(Parser, Debug)]
pub struct PartialConfigCliLoader {
    #[arg()]
    domain: Option<String>,
}

impl PartialConfigLoader for PartialConfigCliLoader {
    fn load() -> Result<crate::config::models::PartialConfig, crate::config::errors::ConfigError> {
        let args = Self::parse();

        Ok(crate::config::models::PartialConfig {
            domain: args.domain,
        })
    }
}
