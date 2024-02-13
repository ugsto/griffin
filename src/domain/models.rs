#[derive(Debug, Clone)]
pub struct Domain {
    top_level_domain: String,
    domain: String,
}

impl Domain {
    pub fn new(domain: &[&str], top_level_domain: String) -> Self {
        let domain = domain.join(".");

        Self {
            top_level_domain,
            domain,
        }
    }

    pub fn domain(&self) -> &str {
        &self.domain
    }

    pub fn top_level_domain(&self) -> &str {
        &self.top_level_domain
    }
}
