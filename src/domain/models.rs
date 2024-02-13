#[derive(Debug, Clone)]
pub struct Domain {
    pub top_level_domain: String,
    pub domain: String,
    pub subdomain: Vec<String>,
}
