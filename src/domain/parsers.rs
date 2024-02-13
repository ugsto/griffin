use super::{errors::DomainParseError, models::Domain};

impl TryFrom<&str> for Domain {
    type Error = DomainParseError;

    fn try_from(domain: &str) -> Result<Self, Self::Error> {
        let mut parts = domain.split('.').rev().map(|part| part.to_string());

        let top_level_domain = parts
            .next()
            .ok_or(DomainParseError::MissingTopLevelDomain)?;
        let domain = parts.next().ok_or(DomainParseError::MissingDomain)?;
        let subdomain = parts.map(|part| part.to_string()).collect::<Vec<_>>();

        let parsed_domain = Domain {
            top_level_domain,
            domain,
            subdomain,
        };

        Ok(parsed_domain)
    }
}
