use {
    crate::{never, palettes::colormaps::ROMA, panic, ToPng},
    bitvec::slice::BitSlice,
    serde::{Deserialize, Serialize},
    std::io::{Read, Write},
};

#[doc(hidden)]
pub use self::{BitDepth::*, ColorType::*};

/// In-memory representation of a PNG file's essential image contents.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
#[non_exhaustive]
pub struct Png {
    pub pixel_data: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub bit_depth: BitDepth,
    pub color_type: ColorType,
    pub palette_data: Option<Vec<u8>>,
    pub transparency_data: Option<Vec<u8>>,
}

impl Png {
    /// Creates a new [`Png`] from the given data.
    pub fn new(data: &impl ToPng) -> Self {
        data.to_png().into_owned()
    }

    /// Create a new indexed-color [`Png`] with the given dimensions, bit depth,
    /// and color palette.
    pub fn new_indexed(width: usize, height: usize, bit_depth: BitDepth, palette: &[u8]) -> Self {
        let palette_data = Some(palette.to_vec());
        let pixel_count = width * height;
        let pixel_data =
            vec![
                0;
                pixel_count * bit_depth.bits_per_sample() * ColorType::Indexed.samples_per_pixel()
                    / 8
            ];
        Png {
            width,
            height,
            bit_depth,
            color_type: ColorType::Indexed,
            palette_data,
            transparency_data: None,
            pixel_data,
        }
    }

    /// Create a new greyscale [`Png`] with the given dimensions and bit depth.
    pub fn new_grayscale(width: usize, height: usize, bit_depth: BitDepth) -> Self {
        let color_type = Luminance;
        let pixel_data = vec![0u8; width * height * color_type.samples_per_pixel()];
        Png {
            pixel_data,
            width,
            height,
            bit_depth,
            color_type,
            palette_data: None,
            transparency_data: None,
        }
    }

    /// Creates a new 24-bit RGB [`Png`] with the given dimensions.
    pub fn new_rgb(width: usize, height: usize) -> Self {
        let bit_depth = EightBit;
        let color_type = RedGreenBlue;
        let pixel_data = vec![0u8; width * height * color_type.samples_per_pixel()];
        Png {
            pixel_data,
            width,
            height,
            bit_depth,
            color_type,
            palette_data: None,
            transparency_data: None,
        }
    }

    /// Creates a new 32-bit RGBA [`Png`] with the given dimensions.
    pub fn new_rgba(width: usize, height: usize) -> Self {
        let bit_depth = EightBit;
        let color_type = RedGreenBlueAlpha;
        let pixel_data = vec![0u8; width * height * color_type.samples_per_pixel()];
        Png {
            pixel_data,
            width,
            height,
            bit_depth,
            color_type,
            palette_data: None,
            transparency_data: None,
        }
    }

    /// Create a [`Png`] from unstructured image data bytes.
    ///
    /// Data beyond the first 32MiB may be ignored.
    pub fn from_unstructured_bytes(bytes: &[u8]) -> Self {
        let mut bit_depth = BitDepth::EightBit;
        let mut color_type = ColorType::Indexed;
        let mut palette_data = Some(ROMA.to_vec());
        let transparency_data = None;
        let width;
        match bytes.len() {
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
            bytes.len() / (bit_depth.bits_per_sample() * color_type.samples_per_pixel());

        let width = 1024.min(width);
        let height = 8192.min((pixel_count + width - 1) / width);

        Png {
            width,
            height,
            bit_depth,
            color_type,
            palette_data,
            transparency_data,
            pixel_data: bytes.to_vec(),
        }
    }

    /// Returns the number of bits per pixel in the image data of this [`Png`].
    ///
    /// This does not relate to the color palette (if one is present).
    pub fn bits_per_pixel(&self) -> usize {
        self.bit_depth.bits_per_sample() * self.color_type.samples_per_pixel()
    }

    /// Returns the number of bytes per row of pixels in the image data of this
    /// [`Png`].
    pub fn image_bytes_per_row(&self) -> usize {
        (self.width * self.bits_per_pixel() + 7) / 8
    }

    /// Returns the total number of image bytes that we expect the image to
    /// contain based on its metadata. **This may not be the actual size of
    /// the image data in this [`Png`]**, but in most cases it should be.
    pub fn image_bytes_expected(&self) -> usize {
        self.image_bytes_per_row() * self.height
    }

    /// Serializes this [`Png`] as a PNG image file.
    pub fn write(&self, output: &mut impl Write) -> Result<(), panic> {
        let mut buffer = Vec::new();
        crate::png::writer::write_png(
            &mut buffer,
            self.pixel_data.as_slice(),
            self.width.try_into()?,
            self.height.try_into()?,
            self.bit_depth,
            self.color_type,
            self.palette_data.as_deref(),
        );
        Ok(output.write_all(&buffer)?)
    }

    /// Deserializes a PNG image file into a [`Png`].
    pub fn read(_input: &impl Read) -> Result<Self, panic> {
        unimplemented!()
    }

    /// Serializes this [`Png`] into a byte vector as a PNG image file.
    pub fn write_vec(&self) -> Result<Vec<u8>, never> {
        let mut output = Vec::new();
        self.write(&mut output)?;
        Ok(output)
    }

    /// Deserialize a PNG image file into a [`Png`] from a byte vector.
    pub fn read_slice(input: &[u8]) -> Result<Self, never> {
        Ok(Self::read(&input)?)
    }

    pub fn mut_pixel(&mut self, x: usize, y: usize) -> &mut [u8] {
        if !matches!(self.bit_depth, EightBit | SixteenBit) {
            unimplemented!(
                "mut_pixel only supports 8- and 16-bit images, use mut_pixel_bits instead"
            );
        }
        let bits_per_pixel = self.bits_per_pixel();
        let row = y * self.image_bytes_per_row();
        let col = x * bits_per_pixel / 8;
        &mut self.pixel_data[row + col..row + col + bits_per_pixel / 8]
    }

    pub fn mut_pixel_bits(&mut self, _x: usize, _y: usize) -> &mut BitSlice<u8> {
        todo!()
    }

    /// Sets the pixel at the given coordinates to the given color.
    /// The required length of the color data may vary depending on the
    /// color type and bit depth of the image.
    ///
    /// This is provided for convenience, but it's not fast. If you need speed,
    /// modify the image data directly or use a faster library.
    pub fn set_pixel(&mut self, x: usize, y: usize, color: &[u8]) -> Result<(), never> {
        self.mut_pixel(x, y).copy_from_slice(color);
        Ok(())
    }

    /// Sets the pixel at the given coordinates to the given color.
    /// The required length of the color data may vary depending on the
    /// color type and bit depth of the image.
    ///
    /// This is provided for convenience, but it's not fast. If you need speed,
    /// modify the image data directly or use a faster library.
    pub fn set_pixel_bits(
        &mut self,
        _x: usize,
        _y: usize,
        _color: &BitSlice<u8>,
    ) -> Result<(), never> {
        unimplemented!()
    }
}

impl Serialize for Png {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        serializer.serialize_bytes(
            &self
                .write_vec()
                .expect("serializing Png to bytes should not fail"),
        )
    }
}

impl<'de> Deserialize<'de> for Png {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de> {
        let bytes: &[u8] = serde_bytes::deserialize(deserializer)?;
        Self::read_slice(bytes).map_err(serde::de::Error::custom)
    }
}

/// The bit depth of an image, as defined in the PNG specification.
///
/// > **bit depth**: for indexed-colour images, the number of bits per palette
/// > index. For other images, the number of bits per sample in the image. This
/// > is the value that appears in the `IHDR` Image header chunk.
///
/// > **sample**: intersection of a channel and a pixel in an image.
///
/// > **channel**: array of all per-pixel information of a particular kind
/// > within a reference image. There are five kinds of information: red, green,
/// > blue, greyscale, and alpha.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(u8)]
pub enum BitDepth {
    OneBit = 1,
    TwoBit = 2,
    FourBit = 4,
    #[default]
    EightBit = 8,
    SixteenBit = 16,
}

/// The color type of an image, as defined in the PNG specification.
///
/// > There are five types of PNG image. Corresponding to each type is a
/// > **colour type**, which is the sum of the following values: 1 (palette
/// > used), 2 (truecolour used) and 4 (alpha used). Greyscale and truecolour
/// > images may have an explicit alpha channel. The PNG image types and
/// > corresponding colour types are listed in Table 8.
///
/// > **greyscale**: image representation in which each pixel is defined by a
/// > single sample of colour information, representing overall luminance (on a
/// > scale from black to white)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(u8)]
pub enum ColorType {
    #[default]
    Luminance = 0,
    RedGreenBlue = 2,
    Indexed = 3,
    LuminanceAlpha = 4,
    RedGreenBlueAlpha = 6,
}

impl BitDepth {
    pub fn bits_per_sample(&self) -> usize {
        u8::from(*self).into()
    }
}

impl From<BitDepth> for u8 {
    fn from(depth: BitDepth) -> Self {
        depth as u8
    }
}

impl ColorType {
    pub fn samples_per_pixel(&self) -> usize {
        match self {
            Luminance => 1,
            RedGreenBlue => 3,
            Indexed => 1,
            LuminanceAlpha => 2,
            RedGreenBlueAlpha => 4,
        }
    }
}

impl From<ColorType> for u8 {
    fn from(val: ColorType) -> Self {
        val as u8
    }
}