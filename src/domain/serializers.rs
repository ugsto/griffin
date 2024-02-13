use std::fmt::Display;

use super::models::Domain;

impl From<&Domain> for String {
    fn from(domain: &Domain) -> Self {
        format!("{}.{}", domain.domain(), domain.top_level_domain())
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
        let domain = Domain::try_from("example.com").unwrap();
        let domain_str: String = (&domain).into();

        assert_eq!(domain_str, "example.com");
    }

    #[test]
    fn test_domain_to_string_single_subdomain() {
        let domain = Domain::try_from("sub.example.com").unwrap();
        let domain_str: String = (&domain).into();

        assert_eq!(domain_str, "sub.example.com");
    }

    #[test]
    fn test_domain_to_string_multiple_subdomains() {
        let domain = Domain::try_from("www.blog.example.com").unwrap();
        let domain_str: String = (&domain).into();

        assert_eq!(domain_str, "www.blog.example.com");
    }
}
