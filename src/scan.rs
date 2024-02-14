use crate::{prelude::*, Domain, DomainResolver, FuzzerStrategy};
use futures::{stream, StreamExt};

pub type GriffinResult = Result<(bool, Domain), String>; // TODO Include proper error

pub async fn scan<'a>(
    domain: &'a Domain,
    fuzzers: &'a [&'a FuzzerStrategy],
    resolver: &'a DomainResolver,
    workers: usize,
) -> impl stream::Stream<Item = GriffinResult> + 'a {
    let domains_iterator = fuzzers
        .iter()
        .flat_map(|fuzzer| fuzzer.fuzz(domain))
        .filter_map(|d| Domain::try_from(d.as_str()).ok());

    let tasks = stream::iter(domains_iterator.map(|domain| {
        let inner_domain_resolver = resolver.clone();
        tokio::spawn(async move {
            (
                inner_domain_resolver.does_domain_resolve(&domain).await,
                domain,
            )
        })
    }))
    .buffer_unordered(workers)
    .map(|result| result.map_err(|err| err.to_string()));

    Box::new(tasks)
}
