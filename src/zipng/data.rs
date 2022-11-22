use crate::{Png, ToZipng, Zip};

/// In-memory representation of a ZIP file's archive contents and a PNG file's
/// image contents.
#[derive(Debug, Clone, Default)]
pub struct Zipng {
    pub zip: Zip,
    pub png: Png,
}

impl Zipng {
    /// Creates a new [`Zipng`] from the given data.
    pub fn new(data: &impl ToZipng) -> Self {
        data.to_zipng().into_owned()
    }
}
