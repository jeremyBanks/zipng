use crate::{panic, ToPng};

/// In-memory representation of a PNG file's essential image contents.
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
#[non_exhaustive]
pub struct Png {
    pub width: u32,
    pub height: u32,
    pub bit_depth: BitDepth,
    pub color_type: ColorType,
    pub pixel_data: Vec<u8>,
    pub palette_data: Option<Vec<u8>>,
    pub transparency_data: Option<Vec<u8>>,
}

impl Png {
    pub fn new(data: impl ToPng) -> Self {
        data.to_png().into_owned()
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, panic> {
        unimplemented!()
    }

    /// Sets the pixel at the given coordinates to the given color.
    /// The required length of the color data may vary depending on the
    /// color type and bit depth of the image.
    pub fn set_pixel(&mut self, x: u32, y: u32, color: &[u8]) -> Result<(), ()> {
        todo!()
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
    One = 1,
    Two = 2,
    Four = 4,
    #[default]
    Eight = 8,
    Sixteen = 16,
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
