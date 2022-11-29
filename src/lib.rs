#![feature(doc_cfg, doc_auto_cfg)]
#![allow(non_upper_case_globals)]
#![warn(unused_crate_dependencies, missing_docs, clippy::redundant_pub_crate)]
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
    std::io::{Read, Write},
};

mod checksums;
mod deflate;
mod generic;
mod io;
mod png;
mod text;
mod zip;
mod zipng;
mod zlib;

#[cfg(feature = "dev-dependencies")]
pub mod dev;
mod opstructs;

use std::io::SeekFrom;

#[doc(hidden)]
pub use crate::text::*;
#[doc(inline)]
pub use crate::{
    checksums::*, deflate::*, generic::*, io::*, opstructs::*, png::*, zip::*, zipng::*, zipng::*,
    zlib::*,
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

#[doc(hidden)]
pub trait Offset {
    fn offset(&mut self) -> usize;

    fn len(&mut self) -> usize;
}
impl<T> Offset for T
where T: std::io::Seek
{
    fn offset(&mut self) -> usize {
        let Ok(position) = self.stream_position() else {
            return usize::MAX;
        };
        let Ok(position) = usize::try_from(position) else {
            return usize::MAX;
        };
        position
    }

    fn len(&mut self) -> usize {
        let old_pos = self.stream_position().unwrap_or(u64::MAX);
        let len = self.seek(SeekFrom::End(0)).unwrap_or(u64::MAX);
        if old_pos != len {
            self.seek(SeekFrom::Start(old_pos)).unwrap();
        }
        len.try_into().unwrap_or(usize::MAX)
    }
}

#[doc(hidden)]
pub trait ReadAndSeek: Read + Offset {}
impl<T> ReadAndSeek for T where T: Read + Offset {}

#[doc(hidden)]
pub trait WriteAndSeek: Write + Offset {}
impl<T> WriteAndSeek for T where T: Write + Offset {}
