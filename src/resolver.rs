use trust_dns_resolver::{
    name_server::{GenericConnector, TokioRuntimeProvider},
    AsyncResolver,
};

#[derive(Debug, Clone)]
pub struct DomainResolver {
    resolver: AsyncResolver<GenericConnector<TokioRuntimeProvider>>,
}

impl DomainResolver {
    pub fn try_new() -> Result<Self, anyhow::Error> {
        let domain_resolver = Self {
            resolver: AsyncResolver::tokio_from_system_conf()?,
        };

        Ok(domain_resolver)
    }

    pub async fn does_domain_resolve(&self, domain: &str) -> bool {
        let resolver = self.resolver.clone();

        resolver.lookup_ip(domain).await.is_ok()
    }
}
