#![feature(doc_cfg, doc_auto_cfg)]
#![warn(unused_crate_dependencies, missing_docs)]
#![cfg_attr(
    all(debug_assertions, any(not(test), feature = "EDITOR")),
    allow(
        dead_code,
        unreachable_code,
        unused_variables,
        unused_crate_dependencies,
        unused_imports,
        missing_docs,
    )
)]
#![deny(unsafe_code)]
#![doc = include_str!("../README.md")]

use {
    crate::generic::*,
    std::io::{BufRead, Seek, Write},
};

mod checksums;
mod deflate;
mod generic;
mod padding;
mod png;
mod text;
mod zip;
mod zipng;
mod zlib;

#[cfg(feature = "dev-dependencies")]
pub mod dev;



#[doc(inline)]
pub use crate::{
    checksums::*, deflate::*, generic::*, png::*, text::*, zip::*, zipng::*, zipng::*, zlib::*,
};

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

pub trait BufReadAndSeek: BufRead + Seek {}
impl<T> BufReadAndSeek for T where T: BufRead + Seek {}

pub trait WriteAndSeek: Write + Seek {}
impl<T> WriteAndSeek for T where T: Write + Seek {}
