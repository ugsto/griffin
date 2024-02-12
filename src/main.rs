use std::process::exit;

use config::{
    errors::ConfigError,
    models::Config,
    strategies::{cli_loader::PartialConfigCliLoader, env_loader::PartialConfigEnvLoader},
    traits::PartialConfigLoader,
};
use fuzzer::strategies::cyrillic::CyrillicFuzzer;

use crate::fuzzer::traits::Fuzzer;

mod config;
mod fuzzer;

fn load_config() -> Result<Config, ConfigError> {
    let partial_config = PartialConfigEnvLoader::load()?.merge(PartialConfigCliLoader::load()?);
    let config = Config::try_from(partial_config)?;

    Ok(config)
}

fn main() {
    let config = load_config().unwrap_or_else(|err| {
        eprintln!("{}", err);
        exit(1)
    });

    let fuzzer = CyrillicFuzzer::new();
    for domain in fuzzer.fuzz(config.domain.as_str()) {
        println!("{}", domain);
    }
}
