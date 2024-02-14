use std::process::exit;

use config::{
    errors::ConfigError,
    models::Config,
    prelude::*,
    strategies::{cli_loader::PartialConfigCliLoader, env_loader::PartialConfigEnvLoader},
};
use griffin::{prelude::*, scan, DomainResolver};

mod config;

fn load_config() -> Result<Config, ConfigError> {
    let partial_config = PartialConfigEnvLoader::load()?.merge(PartialConfigCliLoader::load()?);
    let config = Config::try_from(partial_config)?;

    Ok(config)
}

#[tokio::main]
async fn main() {
    let config = load_config().unwrap_or_else(|err| {
        eprintln!("{}", err);
        exit(1)
    });

    let domain_resolver = DomainResolver::try_new().unwrap_or_else(|err| {
        eprintln!("{}", err);
        exit(1)
    });

    let fuzzer_refs = config.fuzzers.iter().collect::<Vec<_>>();

    let mut results = scan(
        &config.domain,
        fuzzer_refs.as_slice(),
        &domain_resolver,
        config.workers,
    )
    .await;

    while let Some(result) = results.next().await {
        match result {
            Ok((does_resolve, domain)) if does_resolve => {
                println!("{}", domain);
            }
            Ok((_, domain)) => {
                eprintln!("Could not resolve domain: {}", domain);
            }
            Err(err) => {
                eprintln!("{}", err);
            }
        };
    }
}
