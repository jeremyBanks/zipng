#![feature(doc_cfg, doc_auto_cfg)]
#![doc = include_str!("../README.md")]

use derive_more::From;
use derive_more::Into;
use tracing::warn;

#[doc(hidden)]
use crate as zipng;

macro_rules! splatt {
    ($({$tt:tt}),* $(,)?) => {
        $($tt)*
    };
}


macro_rules! if_cfg_unstable {
    ( $then:item $(else $else:item )?) => {
        splatt!{{#}, {[cfg(feature = env!("CARGO_PKG_VERSION"))]}, { $then } }
    }
}

if_cfg_unstable!{
    pub fn main() {
        println!("hello, world!")
    }
    else
    pub fn main() {
        println!("oh no!")
    }
}

pub mod r#impl {
    //! Unstable internal implementation details.
    #![path = "."]
    #![allow(missing_docs)]
    pub mod brotli;
    pub mod checksums;
    pub mod deflate;
    pub mod font;
    pub mod generic;
    pub mod padding;
    pub mod png;
    pub mod text;
    pub mod zip;
    pub mod zlib;
}

#[doc(inline)]
pub use crate::png::BitDepth;
#[doc(inline)]
pub use crate::png::ColorType;
#[doc(inline)]
pub(crate) use crate::r#impl::*;

/// Creates a ZIP archive with this crate's suggested/canonical settings.
pub fn zip(files: &Files) -> Vec<u8> {
    todo!()
}

/// Creates a normal ZIP archive with nothing extraneous included.
pub fn zip_minimal(files: &Files) -> Vec<u8> {
    todo!()
}

/// Creates a ZIP archive with file contents stored contiguously, aligned
/// and padded to to 1024-byte blocks.
pub fn zip_blocks(files: &Files) -> Vec<u8> {
    todo!()
}

/// Creates a ZIP archive which can also be interpreted as a PNG image file
/// displaying the first PNG image file that the archive itself contains.
pub fn zipng_covered(files: &Files) -> Vec<u8> {
    todo!()
}

/// Creates a "transparent" ZIP archive which can also be interpreted as a PNG
/// with all of the file contents embedded in the image data.
pub fn zipng_uncovered(files: &Files) -> Vec<u8> {
    todo!()
}

/// Creates a PNG file with the given image data and metadata.
pub fn png(body: &[u8], _meta: ()) -> Vec<u8> {
    todo!()
}

#[doc(hidden)]
/// Files to be included in a zip archive.
#[derive(Debug, Default, Clone, PartialEq, Eq, From, Into)]
pub struct Files {
    files: Vec<(Vec<u8>, Vec<u8>)>,
}
