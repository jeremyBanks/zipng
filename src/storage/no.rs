use super::Storage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// No-op dummy storage backend.
pub struct NoStorage;

impl Storage for NoStorage {}
