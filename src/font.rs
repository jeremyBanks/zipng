#![allow(clippy::unusual_byte_groupings)]

use {
    crate::{panic, Png},
    bitvec::{
        order::{Lsb0, Msb0},
        view::AsBits,
    },
    std::fmt::Debug,
};

/// A bitmap font that can be used to render text onto a [`Png`] image.
pub struct Font;
