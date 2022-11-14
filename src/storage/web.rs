use super::StorageImpl;

/// Read-only storage backed by a web server.
#[derive(Debug, Clone)]
pub struct WebStorage {
    pub base_url: String,
}

impl StorageImpl for WebStorage {}
