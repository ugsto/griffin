use std::fmt::Display;

use super::models::Domain;

impl Domain {
    pub fn base_domain(&self) -> String {
        let subdomain = self.subdomain.join(".");

        if subdomain.is_empty() {
            return self.domain.to_string();
        }

        format!("{}.{}", subdomain, self.domain)
    }
}

impl From<&Domain> for String {
    fn from(domain: &Domain) -> Self {
        let base_domain = domain.base_domain();

        format!("{}.{}", base_domain, domain.top_level_domain)
    }
}

impl Display for Domain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_to_string_no_subdomains() {
        let domain = Domain {
            top_level_domain: "com".to_string(),
            domain: "example".to_string(),
            subdomain: vec![],
        };

        let domain_str: String = (&domain).into();
        assert_eq!(domain_str, "example.com");
    }

    #[test]
    fn test_domain_to_string_single_subdomain() {
        let domain = Domain {
            top_level_domain: "com".to_string(),
            domain: "example".to_string(),
            subdomain: vec!["sub".to_string()],
        };

        let domain_str: String = (&domain).into();
        assert_eq!(domain_str, "sub.example.com");
    }

    #[test]
    fn test_domain_to_string_multiple_subdomains() {
        let domain = Domain {
            top_level_domain: "com".to_string(),
            domain: "example".to_string(),
            subdomain: vec!["www".to_string(), "blog".to_string()],
        };

        let domain_str: String = (&domain).into();
        assert_eq!(domain_str, "www.blog.example.com");
    }

    #[test]
    fn test_from_string_to_string() {
        let domain_str = "www.blog.example.com";
        let domain = Domain::try_from(domain_str).unwrap();
        let converted_domain_str = String::from(&domain);

        assert_eq!(domain_str, converted_domain_str);
    }
}
