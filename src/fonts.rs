#![allow(clippy::unusual_byte_groupings)]

use std::fmt::Debug;

use static_assertions::assert_obj_safe;

use crate::default;

pub enum OneBitColor {
    Background = 0,
    Foreground = 1,
}

pub enum TwoBitColor {
    Background = 0,
    Light = 1,
    Mid = 2,
    Foreground = 3,
}

pub type Canvas = [bool; 4096];
pub type GlyphBits = u128;

pub struct FontOptions {
    pub top: usize,
    pub left: usize,
    pub width: Option<usize>,
    pub height: Option<usize>,
    pub pack_characters: bool,
    pub pack_lines: bool,
}

impl FontOptions {
    pub const fn new() -> Self {
        Self {
            top: 0,
            left: 0,
            height: None,
            width: Some(64),
            pack_characters: true,
            pack_lines: true,
        }
    }

    pub fn print(&self, text: &str, canvas: &mut Canvas) {}
}

impl Default for FontOptions {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Packing {
    Local,
    Global,
}

assert_obj_safe!(BitmapFont);

pub trait BitmapFont: Debug {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn glyphs(&self) -> &'static [(char, u16)];

    fn x_margin(&self) -> usize {
        1
    }

    fn x_packing(&self) -> Option<Packing> {
        Some(Packing::Local)
    }

    fn y_margin(&self) -> usize {
        1
    }

    fn y_packing(&self) -> Option<Packing> {
        Some(Packing::Global)
    }
}

// inspiration: https://i.imgur.com/4PmVdak.png

#[derive(Debug, Clone, Copy, Default)]
pub struct MicroFont;
impl BitmapFont for MicroFont {
    fn width(&self) -> usize {
        3
    }
    fn height(&self) -> usize {
        5
    }
    fn glyphs(&self) -> &'static [(char, u16)] {
        &[
            (' ', 0b1000_000_010_000_000),
            ('_', 0b_000_000_000_000_111),
            ('-', 0b_000_000_010_001_101),
            (',', 0b_000_000_010_001_100),
            (';', 0b_000_010_000_010_110),
            (':', 0b_000_010_000_010_000),
            ('!', 0b_000_000_010_000_001),
            ('?', 0b_111_001_010_000_010),
            ('.', 0b_000_000_010_001_110),
            ('"', 0b_000_000_010_000_010),
            ('(', 0b_000_000_010_001_000),
            (')', 0b_000_000_010_001_001),
            ('[', 0b_111_100_100_100_111),
            (']', 0b_111_001_001_001_111),
            ('{', 0b_011_010_100_010_011),
            ('}', 0b_110_010_001_010_110),
            ('*', 0b_000_000_010_001_010),
            ('/', 0b_001_001_010_100_100),
            ('\'', 0b_000_000_010_000_111),
            ('\\', 0b_100_100_010_001_001),
            ('\x09', 0b1000_000_111_000_000),
            ('%', 0b_101_001_010_100_101),
            ('`', 0b_001_010_000_000_000),
            ('^', 0b_010_101_000_000_000),
            ('+', 0b_000_000_010_001_011),
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

#[derive(Debug, Clone, Copy, Default)]
pub struct NanoFont;
impl BitmapFont for NanoFont {
    fn width(&self) -> usize {
        3
    }
    fn height(&self) -> usize {
        3
    }
    fn glyphs(&self) -> &'static [(char, u16)] {
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

/// encodes ASCII characters as their corresponding bit pattern in
/// a 2x4 grid.
#[derive(Debug, Clone, Copy, Default)]
pub struct AsciiBits;
impl BitmapFont for AsciiBits {
    fn width(&self) -> usize {
        2
    }
    fn height(&self) -> usize {
        4
    }

    fn x_packing(&self) -> Option<Packing> {
        None
    }

    fn x_margin(&self) -> usize {
        0
    }

    fn y_packing(&self) -> Option<Packing> {
        None
    }

    fn y_margin(&self) -> usize {
        0
    }

    fn glyphs(&self) -> &'static [(char, u16)] {
        &[
            // for 0 through 0x7F
            (0 as char, 0),
            (1 as char, 1),
            (2 as char, 2),
            (3 as char, 3),
            (4 as char, 4),
            (5 as char, 5),
            (6 as char, 6),
            (7 as char, 7),
            (8 as char, 8),
            (9 as char, 9),
            (10 as char, 10),
            (11 as char, 11),
            (12 as char, 12),
            (13 as char, 13),
            (14 as char, 14),
            (15 as char, 15),
            (16 as char, 16),
            (17 as char, 17),
            (18 as char, 18),
            (19 as char, 19),
            (20 as char, 20),
            (21 as char, 21),
            (22 as char, 22),
            (23 as char, 23),
            (24 as char, 24),
            (25 as char, 25),
            (26 as char, 26),
            (27 as char, 27),
            (28 as char, 28),
            (29 as char, 29),
            (30 as char, 30),
            (31 as char, 31),
            (32 as char, 32),
            (33 as char, 33),
            (34 as char, 34),
            (35 as char, 35),
            (36 as char, 36),
            (37 as char, 37),
            (38 as char, 38),
            (39 as char, 39),
            (40 as char, 40),
            (41 as char, 41),
            (42 as char, 42),
            (43 as char, 43),
            (44 as char, 44),
            (45 as char, 45),
            (46 as char, 46),
            (47 as char, 47),
            (48 as char, 48),
            (49 as char, 49),
            (50 as char, 50),
            (51 as char, 51),
            (52 as char, 52),
            (53 as char, 53),
            (54 as char, 54),
            (55 as char, 55),
            (56 as char, 56),
            (57 as char, 57),
            (58 as char, 58),
            (59 as char, 59),
            (60 as char, 60),
            (61 as char, 61),
            (62 as char, 62),
            (63 as char, 63),
            (64 as char, 64),
            (65 as char, 65),
            (66 as char, 66),
            (67 as char, 67),
            (68 as char, 68),
            (69 as char, 69),
            (70 as char, 70),
            (71 as char, 71),
            (72 as char, 72),
            (73 as char, 73),
            (74 as char, 74),
            (75 as char, 75),
            (76 as char, 76),
            (77 as char, 77),
            (78 as char, 78),
            (79 as char, 79),
            (80 as char, 80),
            (81 as char, 81),
            (82 as char, 82),
            (83 as char, 83),
            (84 as char, 84),
            (85 as char, 85),
            (86 as char, 86),
            (87 as char, 87),
            (88 as char, 88),
            (89 as char, 89),
            (90 as char, 90),
            (91 as char, 91),
            (92 as char, 92),
            (93 as char, 93),
            (94 as char, 94),
            (95 as char, 95),
            (96 as char, 96),
            (97 as char, 97),
            (98 as char, 98),
            (99 as char, 99),
            (100 as char, 100),
            (101 as char, 101),
            (102 as char, 102),
            (103 as char, 103),
            (104 as char, 104),
            (105 as char, 105),
            (106 as char, 106),
            (107 as char, 107),
            (108 as char, 108),
            (109 as char, 109),
            (110 as char, 110),
            (111 as char, 111),
            (112 as char, 112),
            (113 as char, 113),
            (114 as char, 114),
            (115 as char, 115),
            (116 as char, 116),
            (117 as char, 117),
            (118 as char, 118),
            (119 as char, 119),
            (120 as char, 120),
            (121 as char, 121),
            (122 as char, 122),
            (123 as char, 123),
            (124 as char, 124),
            (125 as char, 125),
            (126 as char, 126),
            (127 as char, 127),
            ('�', 153),
        ]
    }
}
