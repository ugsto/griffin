use std::process::exit;

use config::prelude::*;
use futures::{stream, StreamExt};
use fuzzer::traits::Fuzzer;
use resolver::DomainResolver;

mod config;
mod domain;
mod fuzzer;
mod resolver;

fn load_config() -> Result<Config, ConfigError> {
    let partial_config = PartialConfigEnvLoader::load()?.merge(PartialConfigCliLoader::load()?);
    let config = Config::try_from(partial_config)?;

    Ok(config)
}

fn initialize_domains_iterators() -> Vec<Box<dyn Fuzzer>> {
    vec![]
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

    let iterators = initialize_domains_iterators();
    let domains_iterator = iterators
        .iter()
        .flat_map(|fuzzer| fuzzer.fuzz(&config.domain));

    let mut tasks = stream::iter(domains_iterator.map(move |domain| {
        let inner_domain_resolver = domain_resolver.clone();
        tokio::spawn(async move {
            (
                inner_domain_resolver.does_domain_resolve(&domain).await,
                domain,
            )
        })
    }))
    .buffer_unordered(config.workers);

    while let Some(result) = tasks.next().await {
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
