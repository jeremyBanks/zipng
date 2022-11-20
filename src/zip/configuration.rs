use {
    crate::{default, generic::never, ToPng, Zip},
    std::{cmp::Ordering, fmt::Debug},
};

#[non_exhaustive]
pub struct FileEntry<'name, 'body> {
    pub name: &'name [u8],
    pub body: &'body [u8],
}

type FilePredicate = fn(&FileEntry) -> bool;
type FileComparison = fn(&FileEntry, &FileEntry) -> Ordering;

pub static SORT_BY_NAME: FileComparison = |a, b| (a.name, a.body).cmp(&(b.name, b.body));
pub static SORT_BY_BODY: FileComparison = |a, b| (a.body, a.name).cmp(&(b.body, b.name));

/// Picks the `mimetype` magic header file used by EPUB and OpenDocument.
pub static PICK_MIMETYPE: FilePredicate =
    |file| file.name == b"mimetype" && file.body.is_ascii() && file.body.len() <= 0xFF;

#[derive(Clone)]
#[non_exhaustive]
pub struct ZipConfiguration {
    /// Whether all body data must be stored contiguously.
    pub body_contiguous: bool,
    /// Align and pad all body data into blocks of this many bytes.
    pub body_alignment: usize,
    /// Comparison function used to determine file body ordering.
    pub sort_body_by: Option<FileComparison>,
    /// Comparison function used to determine file metadata ordering.
    pub sort_meta_by: Option<FileComparison>,
    /// Predicate function used to determine files that should be pinned to the
    /// top of the archive without alignment or padding.
    pub pick_header_with: Option<FilePredicate>,
}

impl ZipConfiguration {
    pub fn new(f: fn(&mut Self)) -> Self {
        let mut zip_configuration = default();
        f(&mut zip_configuration);

        zip_configuration
    }
}

impl Debug for ZipConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZipConfiguration")
            .field("body_contiguous", &self.body_contiguous)
            .field("body_alignment", &self.body_alignment)
            .field(
                "sort_body_by",
                self.sort_body_by.map(|_| &Some(())).unwrap_or(&None),
            )
            .field(
                "sort_meta_by",
                self.sort_meta_by.map(|_| &Some(())).unwrap_or(&None),
            )
            .finish()
    }
}

impl Default for ZipConfiguration {
    fn default() -> Self {
        Self {
            body_contiguous: true,
            body_alignment: 1024,
            sort_body_by: SORT_BY_BODY.into(),
            sort_meta_by: SORT_BY_NAME.into(),
            pick_header_with: PICK_MIMETYPE.into(),
        }
    }
}

impl Zip {
    /// Configure this archive to avoid including anything extraneous.
    pub fn minimal(&mut self) {
        todo!()
    }

    /// Configures this archive to store file contents contiguously, aligned
    /// and padded to to 1024-byte blocks.
    pub fn blocks(&mut self) {
        todo!()
    }

    /// Configures this archive to create a polyglot PNG/ZIP file which can also
    /// be interpreted as a PNG image image file displaying the content of the
    /// first PNG image file that the archive contains.
    pub fn covered(&mut self) {
        todo!()
    }

    /// Configures this archive to create a polyglot PNG/ZIP file which can also
    /// be interpreted as a PNG image with all of the file contents displayed in
    /// the image data.
    pub fn uncovered(&mut self) {
        todo!()
    }
}
