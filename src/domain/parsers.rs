use super::{errors::DomainParseError, models::Domain};

impl TryFrom<&str> for Domain {
    type Error = DomainParseError;

    fn try_from(domain: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = domain.split('.').collect();
        if parts.len() < 2 {
            return Err(DomainParseError::MissingTopLevelDomain);
        }

        let top_level_domain = parts.last().unwrap();
        if top_level_domain.is_empty() {
            return Err(DomainParseError::MissingTopLevelDomain);
        }

        let domain = parts.get(parts.len() - 2).unwrap();
        if domain.is_empty() {
            return Err(DomainParseError::MissingDomain);
        }

        let subdomain = if parts.len() > 2 {
            parts[..parts.len() - 2]
                .iter()
                .map(|s| s.to_string())
                .collect()
        } else {
            Vec::new()
        };

        Ok(Domain {
            top_level_domain: top_level_domain.to_string(),
            domain: domain.to_string(),
            subdomain,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_basic_domain() {
        let domain_str = "example.com";
        let domain = Domain::try_from(domain_str).unwrap();
        assert_eq!(domain.top_level_domain, "com");
        assert_eq!(domain.domain, "example");
        assert!(domain.subdomain.is_empty());
    }

    #[test]
    fn test_valid_subdomain() {
        let domain_str = "sub.example.com";
        let domain = Domain::try_from(domain_str).unwrap();
        assert_eq!(domain.top_level_domain, "com");
        assert_eq!(domain.domain, "example");
        assert_eq!(domain.subdomain, vec!["sub"]);
    }

    #[test]
    fn test_valid_multiple_subdomains() {
        let domain_str = "a.b.c.example.com";
        let domain = Domain::try_from(domain_str).unwrap();
        assert_eq!(domain.top_level_domain, "com");
        assert_eq!(domain.domain, "example");
        assert_eq!(domain.subdomain, vec!["a", "b", "c"]);
    }

    #[test]
    fn test_missing_tld() {
        let domain_str = "example";
        let result = Domain::try_from(domain_str);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DomainParseError::MissingTopLevelDomain);
    }

    #[test]
    fn test_missing_domain() {
        let domain_str = ".com";
        let result = Domain::try_from(domain_str);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DomainParseError::MissingDomain);
    }

    #[test]
    fn test_empty_input() {
        let domain_str = "";
        let result = Domain::try_from(domain_str);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), DomainParseError::MissingTopLevelDomain);
    }
}
