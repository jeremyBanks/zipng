use super::Storage;
use std::sync::Arc;


#[derive(Debug, Clone)]
pub struct LayeredStorage<Inner: Storage, Next: Storage> {
    inner: Arc<Inner>,
    next: Arc<Next>,
}

// Configurable!
// What are the options?

// For later! Just use sqlite for now.
