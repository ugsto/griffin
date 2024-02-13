use clap::Parser;

use crate::{config::traits::PartialConfigLoader, domain::prelude::*, ConfigError, PartialConfig};

#[derive(Parser, Debug)]
pub struct PartialConfigCliLoader {
    #[arg()]
    domain: Option<String>,

    #[arg(short, long)]
    workers: Option<usize>,
}

impl PartialConfigLoader for PartialConfigCliLoader {
    fn load() -> Result<PartialConfig, ConfigError> {
        let args = Self::parse();
        let domain = args
            .domain
            .map(|d| Domain::try_from(d.as_str()))
            .transpose()
            .map_err(|e| {
                ConfigError::ParseError("domain".to_string(), "Domain".to_string(), e.to_string())
            })?;

        Ok(crate::config::models::PartialConfig {
            domain,
            workers: args.workers,
        })
    }
}
