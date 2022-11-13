use super::Storage;

/// Storage backed by a web server.
#[derive(Debug, Clone)]
pub struct WebStorage {
    pub base_url: String,
}

impl Storage for WebStorage {}
