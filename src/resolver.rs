use trust_dns_resolver::{
    name_server::{GenericConnector, TokioRuntimeProvider},
    AsyncResolver,
};

use crate::domain::prelude::Domain;

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

    pub async fn does_domain_resolve(&self, domain: &Domain) -> bool {
        self.resolver
            .lookup_ip(String::from(domain).as_str())
            .await
            .is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_does_domain_resolve() {
        let domain = Domain::try_from("google.com").unwrap();
        let resolver = DomainResolver::try_new().unwrap();

        assert!(resolver.does_domain_resolve(&domain).await);
    }

    #[tokio::test]
    async fn test_not_does_domain_resolve() {
        let domain = Domain {
            top_level_domain: "invalidy".to_string(),
            domain: "idontexist".to_string(),
            subdomain: vec![],
        };
        let resolver = DomainResolver::try_new().unwrap();

        assert!(!resolver.does_domain_resolve(&domain).await);
    }
}
