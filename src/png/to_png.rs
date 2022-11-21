use {
    super::palettes::EIGHT_BIT_DATA,
    crate::{
        default, palettes, BitDepth, ColorType, Luminance, OneBit, Png, RedGreenBlue,
        RedGreenBlueAlpha, TwoBit,
    },
    bytemuck::{bytes_of, cast},
    std::borrow::Cow::{self, Borrowed, Owned},
    tracing::warn,
};

/// A [`Png`] or input that can be converted to one.
pub trait ToPng {
    /// Create or borrow a [`Png`] from this input.
    fn to_png(&self) -> Cow<Png>;
}

impl ToPng for Png {
    /// Returns a reference to this [`Png`].
    fn to_png(&self) -> Cow<Png> {
        Borrowed(self)
    }
}

impl ToPng for [u8] {
    /// Create a [`Png`] from unstructured image bytes.
    ///
    /// Data beyond the first 32MiB may be ignored.
    fn to_png(&self) -> Cow<Png> {
        let mut bit_depth = BitDepth::EightBit;
        let mut color_type = ColorType::Indexed;
        let mut palette_data = Some(bytes_of(&EIGHT_BIT_DATA).to_vec());
        let transparency_data = None;
        let width;
        match self.len() {
            len @ 0x0..=0x20 => {
                palette_data = None;
                bit_depth = OneBit;
                color_type = Luminance;
                width = 16.min(len * 8);
            },
            0x21..=0x100 => {
                palette_data = None;
                bit_depth = TwoBit;
                color_type = Luminance;
                width = 16;
            },
            0x101..=0x200 => {
                width = 16;
            },
            0x201..=0x800 => {
                width = 32;
            },
            0x801..=0x2000 => {
                width = 64;
            },
            0x2001..=0x8000 => {
                width = 128;
            },
            0x8001..=0x20000 => {
                width = 256;
            },
            0x20001..=0x80000 => {
                width = 512;
            },
            0x80001..=0x200000 => {
                width = 1024;
            },
            0x200001..=0x800000 => {
                width = 1024;
                palette_data = None;
                color_type = RedGreenBlue;
            },
            _ => {
                width = 1024;
                palette_data = None;
                color_type = RedGreenBlueAlpha;
            },
        }
        let pixel_count =
            self.len() / (bit_depth.bits_per_sample() * color_type.samples_per_pixel());

        let width = 1024.min(width as u32);
        let height = 8192.min((pixel_count as u32 + width - 1) / width);

        Owned(Png {
            width,
            height,
            bit_depth,
            color_type,
            palette_data,
            transparency_data,
            pixel_data: self.to_vec(),
        })
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> ToPng for [[u8; WIDTH]; HEIGHT] {
    /// Create a [`Png`] from a 2D array of `u8` luminance pixels.
    fn to_png(&self) -> Cow<Png> {
        let pixel_data = bytes_of(self).to_vec();
        Owned(Png {
            width: WIDTH as u32,
            height: HEIGHT as u32,
            bit_depth: BitDepth::EightBit,
            color_type: RedGreenBlue,
            palette_data: None,
            transparency_data: None,
            pixel_data,
        })
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> ToPng for [[[u8; 3]; WIDTH]; HEIGHT] {
    /// Create a [`Png`] from a 2D array of RGB pixel arrays.
    fn to_png(&self) -> Cow<Png> {
        unimplemented!()
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> ToPng for [[[u8; 4]; WIDTH]; HEIGHT] {
    /// Create a [`Png`] from a 2D array of RGBA pixel arrays.
    fn to_png(&self) -> Cow<Png> {
        unimplemented!()
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> ToPng for [[u32; WIDTH]; HEIGHT] {
    /// Create a [`Png`] from a 2D array of `u32` RGBA pixels.
    fn to_png(&self) -> Cow<Png> {
        unimplemented!()
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> ToPng for [[(u8, u8, u8); WIDTH]; HEIGHT] {
    /// Create a [`Png`] from a 2D array of RGB pixel tuples.
    fn to_png(&self) -> Cow<Png> {
        Owned(
            self.map(|row| row.map(|(r, g, b)| [r, g, b]))
                .to_png()
                .into_owned(),
        )
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> ToPng for [[(u8, u8, u8, u8); WIDTH]; HEIGHT] {
    /// Create a [`Png`] from a 2D array of RGBA pixel tuples.
    fn to_png(&self) -> Cow<Png> {
        Owned(
            self.map(|row| row.map(|(r, g, b, a)| [r, g, b, a]))
                .to_png()
                .into_owned(),
        )
    }
}

impl ToPng for str {
    /// Create a [`Png`] from text, by rendering it with a bitmap font.
    fn to_png(&self) -> Cow<Png> {
        unimplemented!()
    }
}

impl ToPng for (usize, usize) {
    /// Create a new 8-bit RGB [`Png`] from a `(width, height)` usize tuple.
    fn to_png(&self) -> Cow<Png> {
        unimplemented!()
    }
}

impl ToPng for ((usize, usize), (BitDepth, ColorType)) {
    /// Create a new [`Png`] from a `((width, height), (BitDepth, ColorType))`
    /// usize tuple.
    fn to_png(&self) -> Cow<Png> {
        unimplemented!()
    }
}

impl ToPng for ((usize, usize), (BitDepth, &[u8])) {
    /// Create a new [`Png`] from a `((width, height), (BitDepth, palette))`
    /// usize tuple.
    fn to_png(&self) -> Cow<Png> {
        let ((width, height), (bit_depth, palette)) = self;
        let palette_data = Some(palette.to_vec());
        let pixel_count = width * height;
        let pixel_data =
            vec![
                0;
                pixel_count * bit_depth.bits_per_sample() * ColorType::Indexed.samples_per_pixel()
                    / 8
            ];
        Owned(Png {
            width: *width as u32,
            height: *height as u32,
            bit_depth: *bit_depth,
            color_type: ColorType::Indexed,
            palette_data,
            transparency_data: None,
            pixel_data,
        })
    }
}

impl ToPng for ((usize, usize), &[u8]) {
    /// Create a new [`Png`] from a `((width, height), pixel data)`
    /// usize tuple.
    fn to_png(&self) -> Cow<Png> {
        unimplemented!()
    }
}

impl ToPng for fn(&mut Png) {
    /// Create a [`Png`] from a function that mutates a [`Png::default`].
    fn to_png(&self) -> Cow<Png> {
        let mut png = default();
        self(&mut png);
        Owned(png)
    }
}

impl ToPng for fn(Png) -> Png {
    /// Create a [`Png`] from a function that mutates a [`Png::default`].
    fn to_png(&self) -> Cow<Png> {
        Owned(self(default()))
    }
}
