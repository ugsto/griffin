use clap::Parser;
use griffin::Domain;

use crate::config::{errors::ConfigError, models::PartialConfig, traits::PartialConfigLoader};

#[derive(Parser, Debug)]
pub struct PartialConfigCliLoader {
    #[arg()]
    domain: Option<String>,

    #[arg(short, long)]
    workers: Option<usize>,

    #[arg(short, long)]
    fuzzers: Vec<String>,
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
        let fuzzers = if args.fuzzers.is_empty() {
            None
        } else {
            Some(args.fuzzers)
        };

        Ok(crate::config::models::PartialConfig {
            domain,
            workers: args.workers,
            fuzzers,
        })
    }
}
