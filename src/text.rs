#![allow(clippy::unusual_byte_groupings)]

use std::io::{Read, Write};

pub mod fonts;

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

impl Font {
    /// Serialize this [`Font`] as a classic Macintosh Bitmapped Font Resource.
    ///
    /// These would traditionally have been stored in a Macintosh filesystem
    /// resource fork of type `FONT` or `NFNT`.
    pub fn write(&self, output: &mut impl Write) -> Result<(), panic> {
        unimplemented!()
    }

    /// Deserialize a classic Macintosh Bitmapped Font Resource into a [`Font`].
    ///
    /// These would traditionally have been stored in a Macintosh filesystem
    /// resource fork of type `FONT` or `NFNT`.
    pub fn read(input: &impl Read) -> Result<Self, panic> {
        unimplemented!()
    }
}
