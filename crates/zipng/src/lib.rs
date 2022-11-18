#![feature(doc_cfg)]
//! A limited but fun encoder for ZIPs and PNGs.
//!
//! This crate doesn't implement any compression. Both for simplicity, and
//! because in most cases it will be most efficient to compress the resulting
//! ZIP as a whole, rather than compressing the constituent parts individually.
//! We provide some wrapper functions doing so with `brotli` for convenience.
//!
//! Internal modules are exposed under the `unstable` feature flag for
//! experimentation, but aren't meant for general use.

use derive_more::From;
use derive_more::Into;
use indexmap::IndexMap;
use png::BitDepth;
use tap::Tap;

#[cfg(not(any(doc, feature = "unstable")))]
macro_rules! mods { ($(mod $i:ident;)*) => ($(
    pub(in crate) mod $i;
)*) }
#[cfg(any(doc, feature = "unstable"))]
macro_rules! mods { ($(mod $i:ident;)*) => ($(
    #[doc(cfg(feature = "unstable"))]
    pub mod $i;
)*) }

mods! {
    mod brotli;
    mod checksums;
    mod deflate;
    mod generic;
    mod padding;
    mod png;
    mod zip;
    mod zlib;
}

fn noopt<T>(_: &mut T) {}
type Opts<Options> = fn(&mut Options);
fn get_opts<Options: Default>(f: Opts<Options>) -> Options {
    Options::default().tap_mut(f)
}

pub fn zipng(files: &Files) -> Vec<u8> {
    zipng_with(files, noopt)
}

pub fn zipng_with(files: &Files, opts: Opts<ZipngOptions>) -> Vec<u8> {
    let opts = get_opts(opts);
    todo!()
}

pub fn zip(files: &Files) -> Vec<u8> {
    zip_with(files, noopt)
}

pub fn zip_with(files: &Files, opts: Opts<ZipOptions>) -> Vec<u8> {
    let opts = get_opts(opts);
    todo!()
}

pub fn png(body: &[u8]) -> Vec<u8> {
    png_with(body, noopt)
}

pub fn png_with(body: &[u8], opts: Opts<PngOptions>) -> Vec<u8> {
    let opts = get_opts(opts);
    todo!()
}

pub fn zipngbr(files: &Files) -> Vec<u8> {
    zipngbr_with(files, noopt)
}

pub fn zipngbr_with(files: &Files, opts: Opts<ZipngBrOptions>) -> Vec<u8> {
    brotli::compress(zipng(files).as_slice()).to_vec()
}

/// Files to be included in a zip archive.
#[derive(Debug, Default, Clone, PartialEq, Eq, From, Into)]
pub struct Files {
    files: IndexMap<Vec<u8>, Vec<u8>>,
}

/// Zip archive options.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub struct ZipOptions {}

/// PNG encoding options.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub struct PngOptions {
    pub width: usize,
    pub height: usize,
    pub bit_depth: BitDepth,
    pub color_mode: ColorMode,
}

/// Brotli compression options.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[non_exhaustive]
pub struct BrotliOptions {}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Into, From)]
#[non_exhaustive]
pub struct ZipngOptions {
    pub png: PngOptions,
    pub zip: ZipOptions,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Into, From)]
#[non_exhaustive]
pub struct ZipngBrOptions {
    pub png: PngOptions,
    pub zip: ZipOptions,
    pub br: BrotliOptions,
}
