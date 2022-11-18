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
use generic::default;
use indexmap::IndexMap;
use png::BitDepth;
use png::ColorMode;
use png::EightBit;
use png::FourBit;
use png::Indexed;
use png::Lightness;
use png::OneBit;
use png::RedGreenBlue;
use png::RedGreenBlueAlpha;
use png::TwoBit;
use png::PALLETTE_8_BIT_DATA;
use tap::Tap;
use tracing::warn;

#[cfg(not(any(doc, feature = "unstable")))]
macro_rules! mods { ($(mod $i:ident;)*) => ($(
    mod $i;
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

pub fn zipng(files: &Files) -> Vec<u8> {
    zipng_with(files, noopt)
}

pub fn zipng_with(files: &Files, opts: Opts<ZipngOptions>) -> Vec<u8> {
    let opts = ZipngOptions::default_for_data(&[]).tap_mut(opts);
    todo!()
}

pub fn zip(files: &Files) -> Vec<u8> {
    zip_with(files, noopt)
}

pub fn zip_with(files: &Files, opts: Opts<ZipOptions>) -> Vec<u8> {
    let opts = ZipOptions::default().tap_mut(opts);
    todo!()
}

pub fn png(body: &[u8]) -> Vec<u8> {
    png_with(body, noopt)
}

pub fn png_with(body: &[u8], opts: Opts<PngOptions>) -> Vec<u8> {
    let opts = PngOptions::default().tap_mut(opts);
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
    pub max_height: usize,
    pub bit_depth: BitDepth,
    pub color_mode: ColorMode,
    pub color_palette: Option<Vec<u8>>,
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

impl ZipngOptions {
    pub fn default_for_data(data: &[u8]) -> Self {
        let mut opts = Self::default();

        opts.png.bit_depth = EightBit;
        opts.png.color_mode = Indexed;
        opts.png.color_palette = Some(PALLETTE_8_BIT_DATA.to_vec());
        opts.png.max_height = 8192;

        match data.len() {
            len @ 0x0..=0x20 => {
                warn!("zip data size is weirdly low ({len} bytes)");
                opts.png.color_palette = None;
                opts.png.bit_depth = OneBit;
                opts.png.color_mode = Lightness;
                opts.png.width = 16;
            },
            0x21..=0x100 => {
                opts.png.color_palette = None;
                opts.png.bit_depth = TwoBit;
                opts.png.color_mode = Lightness;
                opts.png.width = 16;
            },
            0x101..=0x200 => {
                opts.png.width = 16;
            },
            0x201..=0x800 => {
                opts.png.width = 32;
            },
            0x801..=0x2000 => {
                opts.png.width = 64;
            },
            0x2001..=0x8000 => {
                opts.png.width = 128;
            },
            0x8001..=0x20000 => {
                opts.png.width = 256;
            },
            0x20001..=0x80000 => {
                opts.png.width = 512;
            },
            0x80001..=0x200000 => {
                opts.png.width = 1024;
            },
            0x200001..=0x800000 => {
                opts.png.width = 1024;
                opts.png.color_palette = None;
                opts.png.color_mode = RedGreenBlue;
            },
            len => {
                opts.png.width = 1024;
                opts.png.color_palette = None;
                opts.png.color_mode = RedGreenBlueAlpha;
                warn!("zip data size is too damn high ({len} bytes)");
            },
        }

        opts
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Into, From)]
#[non_exhaustive]
pub struct ZipngBrOptions {
    pub png: PngOptions,
    pub zip: ZipOptions,
    pub br: BrotliOptions,
}