use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Domain {
    pub top_level_domain: String,
    pub domain: String,
    pub subdomain: Vec<String>,
}

impl From<&Domain> for String {
    fn from(domain: &Domain) -> Self {
        let subdomain = domain.subdomain.join(".");

        if subdomain.is_empty() {
            return format!("{}.{}", domain.domain, domain.top_level_domain);
        }

        format!(
            "{}.{}.{}",
            subdomain, domain.domain, domain.top_level_domain
        )
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
