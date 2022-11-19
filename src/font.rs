#![allow(clippy::unusual_byte_groupings)]

use {
    crate::{panic, Png},
    bitvec::{
        order::{Lsb0, Msb0},
        view::AsBits,
    },
    std::fmt::Debug,
};

type GlyphBits = u16;

/// A bitmap font that can be used to render text onto a PNG image.
pub trait Font: Send + Sync {
    fn name(&self) -> &'static str;

    /// Width of the pixel grid this font is drawn on.
    /// Typically also be the width of the widest glyph.
    fn width(&self) -> usize;

    /// Height of the pixel grid this font is drawn on.
    fn height(&self) -> usize;

    /// Minimum horizontal margin between glyphs.
    fn x_margin(&self) -> usize {
        1
    }

    /// whether glyphs should be
    /// - None: monospace based on the entire font
    /// - Global: monospace based on the glyphs in used
    /// - Local: packed/kerned in as much as possible while respecting x_margin
    fn x_packing(&self) -> Option<Packing> {
        Some(Packing::Local)
    }

    /// recommended minimum vertical margin between glyphs
    fn y_margin(&self) -> usize {
        1
    }

    /// whether line heights should be
    /// - None: fixed based on the entire font
    /// - Global: fixed based on the glyphs in use
    /// - Local: packed/kerned in as much as possible while respecting y_margin
    fn y_packing(&self) -> Option<Packing> {
        Some(Packing::Global)
    }

    /// map of characters to glyphs, represented as bits
    fn glyphs(&self) -> &'static [(char, GlyphBits)];

    /// the good stuff?
    fn render(&self, png: &mut Png, text: &str) -> Result<(), panic> {
        let width = self.width();
        let height = self.height();
        let x_margin = self.x_margin();
        let y_margin = self.y_margin();
        let x_packing = self.x_packing();
        let y_packing = self.y_packing();
        let glyphs = self.glyphs();

        let mut x = 0;
        let mut y = 0;
        let mut max_y = 0;
        let mut last_char = None;
        for character in text.chars() {
            let (glyph, glyph_width) = match glyphs.iter().find(|(c, _)| *c == character) {
                Some((_, glyph)) => (glyph, width),
                None => {
                    eprintln!("Unknown character: {}", character);
                    continue;
                },
            };

            let glyph_height = height;
            let glyph_x = x;
            let glyph_y = y;

            let mut glyph_bits = glyph.to_le_bytes().as_bits::<Lsb0>();
            let mut glyph_bits = glyph_bits.iter().take(glyph_width * glyph_height + 1);
            let invisible = glyph_bits.next().unwrap().into();

            if invisible {
                glyph_bits = std::iter::repeat(false);
            }

            for _ in 0..glyph_height {
                for _ in 0..glyph_width {
                    let bit = glyph_bits.next().unwrap().into();
                    png.set_pixel(glyph_x, glyph_y, bit);
                }
            }

            let x_advance = match x_packing {
                Some(Packing::Local) => glyph_width,
                Some(Packing::Global) => width,
                None => glyph_width,
            };
            let y_advance = match y_packing {
                Some(Packing::Local) => glyph_height,
                Some(Packing::Global) => height,
                None => glyph_height,
            };

            x += x_advance + x_margin;
            max_y = max_y.max(y + y_advance);
            if x + x_advance + x_margin > png.width as _ {
                x = 0;
                y = max_y + y_margin;
            }

            last_char = Some(character);
        }

        Ok(())
    }
}

/// How aggressively to pack text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Packing {
    Local,
    Global,
}

/// Built-in bitmap fonts.
pub const FONTS: &[&dyn Font] = &[
    Mini5pt::DYN,
    Slab9pt::DYN,
    Sans9pt::DYN,
    Mono9pt::DYN,
    Serif9pt::DYN,
    Micro3pt::DYN,
];

/// 3-point _Micro_
///
/// Inspired by /u/Udzu's Unicase Microfont.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Micro3pt;
impl Micro3pt {
    pub const DYN: &dyn Font = &Self;
}
impl Font for Micro3pt {
    fn name(&self) -> &'static str {
        "3-point Micro"
    }

    fn width(&self) -> usize {
        3
    }

    fn height(&self) -> usize {
        3
    }

    fn glyphs(&self) -> &'static [(char, GlyphBits)] {
        &[
            (' ', 0b1000_010_000),
            ('!', 0b_001_001_001),
            ('"', 0b_111_111_111),
            ('(', 0b_111_111_111),
            (')', 0b_111_111_111),
            ('+', 0b_111_111_111),
            ('-', 0b_111_100_000),
            ('.', 0b_111_111_111),
            ('/', 0b_111_111_111),
            ('0', 0b_111_101_111),
            ('1', 0b_001_001_001),
            ('2', 0b_111_001_111),
            ('3', 0b_111_001_111),
            ('4', 0b_101_101_111),
            ('5', 0b_111_100_111),
            ('6', 0b_111_100_111),
            ('7', 0b_111_001_001),
            ('8', 0b_111_111_111),
            ('9', 0b_111_111_111),
            (':', 0b_111_111_111),
            ('=', 0b_111_100_111),
            ('?', 0b_110_001_010),
            ('A', 0b_011_111_111),
            ('B', 0b_100_111_111),
            ('C', 0b_111_100_111),
            ('D', 0b_001_111_111),
            ('E', 0b_111_110_111),
            ('F', 0b_111_110_100),
            ('G', 0b_110_111_111),
            ('H', 0b_100_111_101),
            ('I', 0b_111_010_111),
            ('J', 0b_111_010_110),
            ('K', 0b_101_110_101),
            ('L', 0b_100_100_111),
            ('M', 0b_111_111_101),
            ('N', 0b_111_111_101),
            ('O', 0b_111_111_111),
            ('P', 0b_111_111_110),
            ('Q', 0b_111_111_011),
            ('R', 0b_111_111_101),
            ('S', 0b_110_111_111),
            ('T', 0b_100_100_111),
            ('U', 0b_011_111_111),
            ('V', 0b_011_111_001),
            ('W', 0b_011_111_111),
            ('X', 0b_011_001_011),
            ('Y', 0b_011_011_111),
            ('Z', 0b_001_110_111),
            ('[', 0b_111_100_111),
            ('\'', 0b_001_001_000),
            ('\\', 0b_111_111_111),
            (']', 0b_111_111_111),
            ('^', 0b_111_111_111),
            ('_', 0b_111_111_111),
            ('`', 0b_111_111_111),
            ('a', 0b_011_111_111),
            ('b', 0b_100_111_111),
            ('c', 0b_111_100_111),
            ('d', 0b_001_111_111),
            ('e', 0b_111_110_111),
            ('f', 0b_111_110_100),
            ('g', 0b_110_111_111),
            ('h', 0b_100_111_101),
            ('i', 0b_111_010_111),
            ('j', 0b_111_010_110),
            ('k', 0b_101_110_101),
            ('l', 0b_100_100_111),
            ('m', 0b_111_111_101),
            ('n', 0b_111_111_101),
            ('o', 0b_111_111_111),
            ('p', 0b_111_111_110),
            ('q', 0b_111_111_011),
            ('r', 0b_111_111_101),
            ('s', 0b_110_111_111),
            ('t', 0b_100_100_111),
            ('u', 0b_011_111_111),
            ('v', 0b_011_111_001),
            ('w', 0b_011_111_111),
            ('x', 0b_011_001_011),
            ('y', 0b_011_011_111),
            ('z', 0b_001_110_111),
            ('{', 0b_111_111_111),
            ('|', 0b_111_111_111),
            ('}', 0b_111_111_111),
            ('�', 0b_110_001_010),
        ]
    }
}

/// 5-point _Mini_
///
/// Inspired by /u/Udzu's Minifont.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mini5pt;
impl Mini5pt {
    pub const DYN: &dyn Font = &Self;
}
impl Font for Mini5pt {
    fn name(&self) -> &'static str {
        "5-point Mini"
    }

    fn width(&self) -> usize {
        3
    }

    fn height(&self) -> usize {
        5
    }

    fn glyphs(&self) -> &'static [(char, GlyphBits)] {
        &[
            (' ', 0b1000_000_010_000_000),
            ('_', 0b_000_000_000_000_111),
            ('-', 0b_000_000_110_000_000),
            (',', 0b_000_000_000_010_110),
            (';', 0b_000_010_000_010_110),
            (':', 0b_000_010_000_010_000),
            ('!', 0b_010_010_000_010_000),
            ('?', 0b_111_001_000_010_000),
            ('.', 0b_000_000_000_010_000),
            ('"', 0b_101_101_000_000_000),
            ('\'', 0b_010_010_000_000_000),
            ('(', 0b_001_010_010_001_000),
            (')', 0b_100_010_010_100_000),
            ('[', 0b_011_010_010_011_000),
            (']', 0b_110_010_010_110_000),
            ('{', 0b_011_110_010_011_000),
            ('}', 0b_110_011_010_110_000),
            ('*', 0b_101_010_101_000_000),
            ('/', 0b_001_010_010_100_000),
            ('\\', 0b_001_010_010_100_000),
            ('\x09', 0b1000_000_111_000_000),
            ('%', 0b_101_001_010_100_101),
            ('`', 0b_001_010_000_000_000),
            ('^', 0b_010_101_000_000_000),
            ('+', 0b_010_111_010_000_000),
            ('<', 0b_001_010_100_010_001),
            ('=', 0b_000_111_000_111_000),
            ('>', 0b_100_010_001_010_100),
            ('|', 0b_010_010_010_010_010),
            ('0', 0b_111_101_101_111_000),
            ('1', 0b_010_110_010_111_000),
            ('2', 0b_111_001_010_111_000),
            ('3', 0b_111_001_001_111_000),
            ('4', 0b_101_101_111_001_000),
            ('5', 0b_111_100_001_111_000),
            ('6', 0b_111_100_101_111_000),
            ('7', 0b_111_001_010_010_000),
            ('8', 0b_111_101_101_111_000),
            ('9', 0b_111_101_111_001_000),
            ('A', 0b_111_101_111_101_000),
            ('a', 0b_111_101_111_101_000),
            ('B', 0b_110_101_110_101_110),
            ('b', 0b_110_111_101_111_000),
            ('C', 0b_111_100_100_100_111),
            ('c', 0b_111_100_100_111_000),
            ('D', 0b_110_101_101_101_110),
            ('d', 0b_110_101_101_110_000),
            ('E', 0b_111_100_110_100_111),
            ('e', 0b_111_110_100_111_000),
            ('F', 0b_111_100_110_100_100),
            ('f', 0b_111_110_100_100_000),
            ('G', 0b_111_100_101_101_111),
            ('g', 0b_111_100_101_111_000),
            ('H', 0b_101_101_111_101_101),
            ('h', 0b_101_111_101_101_000),
            ('I', 0b_111_010_010_010_111),
            ('i', 0b_111_010_010_111_000),
            ('J', 0b_001_001_001_101_111),
            ('j', 0b_111_001_001_110_000),
            ('K', 0b_101_110_100_110_101),
            ('k', 0b_101_110_110_101_000),
            ('L', 0b_100_100_100_100_111),
            ('l', 0b_100_100_100_111_000),
            ('m', 0b_101_111_111_101_000),
            ('M', 0b_101_111_111_101_101),
            ('n', 0b_101_111_111_101_000),
            ('N', 0b_101_111_111_111_101),
            ('O', 0b_111_101_101_101_111),
            ('o', 0b_111_101_101_111_000),
            ('p', 0b_111_101_111_100_000),
            ('P', 0b_111_101_111_100_100),
            ('Q', 0b_111_101_101_111_001),
            ('q', 0b_111_101_111_011_000),
            ('r', 0b_111_101_111_101_000),
            ('R', 0b_111_101_111_101_101),
            ('s', 0b_111_100_001_111_000),
            ('S', 0b_111_100_111_001_111),
            ('t', 0b_111_010_010_010_000),
            ('T', 0b_111_010_010_010_010),
            ('U', 0b_101_101_101_101_111),
            ('u', 0b_101_101_101_111_000),
            ('v', 0b_101_101_101_010_000),
            ('V', 0b_101_101_101_101_010),
            ('W', 0b_101_101_101_111_111),
            ('w', 0b_101_101_111_111_000),
            ('x', 0b_101_010_010_101_000),
            ('X', 0b_101_101_010_101_101),
            ('y', 0b_101_010_010_010_000),
            ('Y', 0b_101_101_010_010_010),
            ('Z', 0b_111_001_010_100_111),
            ('z', 0b_111_001_010_111_000),
            ('�', 0b_010_111_101_111_010),
        ]
    }
}

/// 9-point _Kareless Slab_
///
/// Inspired by Apple's Susan Kare's _Toronto_.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Slab9pt;
impl Slab9pt {
    pub const DYN: &dyn Font = &Self;
}
impl Font for Slab9pt {
    fn name(&self) -> &'static str {
        "9-point Kareless Slab"
    }

    fn width(&self) -> usize {
        9
    }

    fn height(&self) -> usize {
        12
    }

    fn glyphs(&self) -> &'static [(char, GlyphBits)] {
        &[]
    }
}

/// 9-point _Kareless Sans_
///
/// Inspired by Apple's Susan Kare's _Chicago_.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sans9pt;
impl Sans9pt {
    pub const DYN: &dyn Font = &Self;
}
impl Font for Sans9pt {
    fn name(&self) -> &'static str {
        "9-point Kareless Sans"
    }

    fn width(&self) -> usize {
        9
    }

    fn height(&self) -> usize {
        12
    }

    fn glyphs(&self) -> &'static [(char, GlyphBits)] {
        &[]
    }
}

/// 9-point _Kareless Mono_
///
/// Inspired by Apple's Susan Kare's _Monaco_.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Mono9pt;
impl Mono9pt {
    pub const DYN: &dyn Font = &Self;
}
impl Font for Mono9pt {
    fn name(&self) -> &'static str {
        "9-point Kareless Mono"
    }

    fn width(&self) -> usize {
        9
    }

    fn height(&self) -> usize {
        12
    }

    fn glyphs(&self) -> &'static [(char, GlyphBits)] {
        &[]
    }
}

/// 9-point _Kareless Serif_
///
/// Inspired by Apple's Susan Kare's _New York_.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Serif9pt;
impl Serif9pt {
    pub const DYN: &dyn Font = &Self;
}
impl Font for Serif9pt {
    fn name(&self) -> &'static str {
        "9-point Kareless Serif"
    }

    fn width(&self) -> usize {
        9
    }

    fn height(&self) -> usize {
        12
    }

    fn glyphs(&self) -> &'static [(char, GlyphBits)] {
        &[]
    }
}
