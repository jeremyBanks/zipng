#![allow(clippy::unusual_byte_groupings)]

mod data;
mod palettes;
mod to_png;

pub use self::{
    data::{
        BitDepth::{self, *},
        ColorType::{self, *},
        Png,
    },
    to_png::ToPng,
};
