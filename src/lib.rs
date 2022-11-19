#![feature(doc_cfg, doc_auto_cfg)]
#![doc = include_str!("../README.md")]

use {
    crate::{generic::*, r#impl::*},
    derive_more::{From, Into},
};

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
    pub mod zip;
    pub mod zlib;
}

#[doc(inline)]
pub use crate::{
    font::{Font, FONTS},
    png::{BitDepth, ColorType, Png},
    zip::Zip,
};

/// Creates a ZIP archive with this crate's suggested/canonical settings.
pub fn zip(files: &Files) -> Vec<u8> {
    todo!()
}

/// Reads the archive contents of a ZIP file.
pub fn unzip(data: &[u8]) -> Result<Zip, panic> {
    Zip::from_bytes(data)
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

/// Creates a PNG file with the given image data.
pub fn png(body: &[u8], _meta: ()) -> Vec<u8> {
    todo!()
}

/// Reads the image data from a PNG file.
pub fn unpng(data: &[u8]) -> Result<Png, panic> {
    Png::from_bytes(data)
}

/// Creates a PNG file with a render of the given text.
pub fn text_png(text: &str) -> Vec<u8> {
    let mut canvas = Png {
        width: 0,
        height: 0,
        color_type: ColorType::RedGreenBlue,
        bit_depth: BitDepth::Eight,
        ..default()
    };

    let font = FONTS[0];

    font.render(text, &mut canvas);

    canvas.to_bytes()
}

#[doc(hidden)]
/// Files to be included in a zip archive.
#[derive(Debug, Default, Clone, PartialEq, Eq, From, Into)]
pub struct Files {
    files: Vec<(Vec<u8>, Vec<u8>)>,
}
