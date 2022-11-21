use crate::{Png, Zip};

/// In-memory representation of a ZIP file's archive contents and a PNG file's
/// image contents.
#[derive(Debug, Clone, Default)]
pub struct Zipng {
    pub zip: Zip,
    pub png: Png,
}
