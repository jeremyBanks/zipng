
use super::Storage;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// A dummy storage backend that stores nothing. Writes are no-ops,
/// reads will fail.
pub struct NoStorage;

impl Storage for NoStorage {}
