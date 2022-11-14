use std::sync::Arc;

use super::StorageImpl;

/// Storage backed by a configuration of two or more different [`Storage`]
/// implementations.
#[derive(Debug, Clone)]
pub struct LayeredStorage<Inner: StorageImpl, Next: StorageImpl> {
    inner: Arc<Inner>,
    next: Arc<Next>,
}

// Configurable!
// What are the options?

// For later! Just use sqlite for now.
