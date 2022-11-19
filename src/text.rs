use bitvec::prelude::Lsb0;
use bitvec::slice::BitSlice;
use bitvec::vec::BitVec;

use crate::font;
use crate::font::Font;

pub struct Canvas {
    pub pixels_per_line: usize,
    pub pixels_per_row: usize,
    pub bits_per_pixel: usize,
    pub data: BitVec<usize, Lsb0>,
}

impl Canvas {
    pub fn new(width: usize, height: usize, bit_depth: usize) -> Self {
        let bits = width * height * bit_depth;

        Self {
            pixels_per_line: width,
            pixels_per_row: height,
            bits_per_pixel: bit_depth,
            data: BitVec::repeat(false, bits),
        }
    }

    pub fn background(&self) -> BitVec<usize, Lsb0> {
        BitVec::repeat(false, self.bits_per_pixel)
    }

    pub fn foreground(&self) -> BitVec<usize, Lsb0> {
        BitVec::repeat(true, self.bits_per_pixel)
    }
}

pub fn print(canvas: &mut BitVec, text: &str, font: &dyn Font) {}
