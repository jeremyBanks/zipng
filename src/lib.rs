#![feature(doc_cfg, doc_auto_cfg)]
#![doc = include_str!("../README.md")]

use {
    crate::generic::*,
    derive_more::{From, Into},
};

mod checksums;
mod deflate;
mod generic;
mod padding;
mod png;
mod poly;
mod text;
mod zip;
mod zlib;

#[doc(inline)]
pub use crate::{checksums::*, deflate::*, generic::*, png::*, poly::*, text::*, zip::*, zlib::*};

/// Creates a ZIP archive.
pub fn zip_to_vec(zip_contents: &impl ToZip) -> Vec<u8> {
    Zip::new(zip_contents).write_vec().unwrap()
}

/// Reads the archive contents of a ZIP file.
pub fn zip_from_slice(zip_file: &[u8]) -> Result<Zip, panic> {
    Ok(Zip::read_slice(zip_file)?)
}

/// Creates a PNG file with the given image data.
pub fn png_to_vec(png_contents: &impl ToPng) -> Vec<u8> {
    Png::new(png_contents).write_vec().unwrap()
}

/// Reads the image data from a PNG file.
pub fn png_from_slice(png_file: &[u8]) -> Result<Png, panic> {
    Ok(Png::read_slice(png_file)?)
}
