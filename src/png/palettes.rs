//! Built-in color palettes

pub mod crameri;
pub mod viridis;

/// Kate Rose Morley's _12-bit Rainbow_ 12-color palette, plus black, white, 25%
/// gray and 75% gray.
pub static FOUR_BIT_RAINBOW: [u8; 48] = [
    0x00, 0x00, 0x00, 0x44, 0x44, 0x44, 0x88, 0x11, 0x77, 0xAA, 0x33, 0x55, 0xCC, 0x66, 0x66, 0xEE,
    0x99, 0x44, 0xEE, 0xDD, 0x00, 0x00, 0xDD, 0x55, 0x44, 0xDD, 0x88, 0x22, 0xCC, 0xBB, 0x00, 0xBB,
    0xCC, 0x00, 0x99, 0xCC, 0x33, 0x66, 0xBB, 0x66, 0x33, 0x99, 0xCC, 0xCC, 0xCC, 0xFF, 0xFF, 0xFF,
];

/// A `u8` to `u8` mapping that was randomly generated.
pub static MAP_SHUFFLED: [u8; 256] = [
    0x51, 0xB5, 0x09, 0x89, 0xCF, 0x32, 0x40, 0x79, 0x0B, 0xE9, 0x63, 0x4F, 0xEF, 0x6B, 0xEB, 0x46,
    0x10, 0x1F, 0x1A, 0x7A, 0x88, 0xC6, 0x19, 0xDE, 0xA3, 0xC2, 0x3F, 0x01, 0x05, 0x34, 0x49, 0xFF,
    0xED, 0x94, 0x43, 0xBC, 0xB9, 0xF5, 0x04, 0x27, 0x1E, 0xAA, 0x0C, 0xB8, 0x02, 0x7C, 0xCC, 0xCB,
    0x87, 0xBB, 0x96, 0x59, 0xFD, 0xA1, 0x74, 0x12, 0x8B, 0x0A, 0x1C, 0x95, 0xE0, 0xFB, 0xB2, 0xB1,
    0xC1, 0xF6, 0x44, 0xA0, 0xA4, 0xBF, 0xE5, 0xEA, 0x06, 0xD0, 0xCD, 0x66, 0xE4, 0x07, 0x9A, 0xD6,
    0x86, 0xC9, 0x60, 0x90, 0x2A, 0x20, 0xD2, 0x0F, 0x5C, 0x9F, 0xC4, 0x3A, 0x58, 0x38, 0x55, 0xA7,
    0xDA, 0xBA, 0xD8, 0x13, 0x25, 0x2E, 0x15, 0xB3, 0x8F, 0x3B, 0xF2, 0xF7, 0x7E, 0x26, 0x80, 0x5F,
    0x8C, 0xC8, 0xF9, 0x6E, 0xBD, 0xA2, 0xF4, 0x91, 0x6A, 0x97, 0x77, 0xF1, 0xD4, 0x78, 0x99, 0x1B,
    0x0E, 0xEC, 0xEE, 0x1D, 0x67, 0xB0, 0x23, 0x62, 0x5B, 0x2F, 0xF3, 0xAC, 0x35, 0x64, 0x5A, 0xDF,
    0x31, 0x45, 0x93, 0x03, 0x2C, 0x81, 0x3D, 0xC0, 0x08, 0xF0, 0xE6, 0xD7, 0x5E, 0x9B, 0x0D, 0x17,
    0xCE, 0xA9, 0x9E, 0x41, 0x11, 0x39, 0x53, 0xFC, 0x7D, 0xAF, 0x28, 0xE3, 0x68, 0xF8, 0x22, 0x3E,
    0x52, 0x18, 0x21, 0x42, 0x4D, 0xD3, 0x75, 0xB6, 0x8A, 0x6D, 0x69, 0x2D, 0x8D, 0xAD, 0x57, 0x24,
    0x56, 0xD5, 0x73, 0x70, 0x29, 0x61, 0x98, 0x83, 0xD1, 0x92, 0xB4, 0x6F, 0x00, 0xDD, 0x7F, 0xAB,
    0x50, 0x82, 0xDB, 0xFE, 0x37, 0x3C, 0x65, 0xC3, 0x6C, 0xE2, 0x48, 0x72, 0x76, 0xE8, 0xC7, 0xD9,
    0x36, 0xAE, 0x9D, 0x47, 0xA6, 0x14, 0xDC, 0xA8, 0xC5, 0x7B, 0x9C, 0x4E, 0x4A, 0x71, 0xBE, 0x8E,
    0x4B, 0x54, 0x16, 0xB7, 0x5D, 0x4C, 0x2B, 0xFA, 0xE7, 0x85, 0x84, 0x30, 0xCA, 0xA5, 0xE1, 0x33,
];

/// A `u8` to `u8` mapping with values sorted by number of bits set.
pub static MAP_BIT_COUNT: [u8; 256] = [
    0x00, 0x07, 0x01, 0x15, 0x04, 0x19, 0x11, 0x42, 0x08, 0x0C, 0x21, 0x32, 0x1E, 0x4D, 0x25, 0x7C,
    0x06, 0x13, 0x1A, 0x37, 0x16, 0x3F, 0x56, 0x76, 0x10, 0x58, 0x49, 0x80, 0x50, 0x7A, 0x7E, 0xCE,
    0x05, 0x0F, 0x1F, 0x47, 0x22, 0x4A, 0x28, 0x87, 0x14, 0x5C, 0x2D, 0x66, 0x30, 0x60, 0x9F, 0xC7,
    0x0B, 0x3C, 0x44, 0x93, 0x2B, 0x78, 0x91, 0xD6, 0x41, 0x62, 0x82, 0xC4, 0x6D, 0xC9, 0xBF, 0xEC,
    0x03, 0x1D, 0x0D, 0x3A, 0x0A, 0x45, 0x3D, 0x72, 0x1B, 0x55, 0x35, 0x6C, 0x38, 0x90, 0xA1, 0xAB,
    0x23, 0x29, 0x4E, 0x70, 0x2E, 0x74, 0x5D, 0xCB, 0x4B, 0x8E, 0x86, 0xB9, 0x8C, 0xB6, 0xAE, 0xF3,
    0x17, 0x5A, 0x51, 0x6A, 0x33, 0x83, 0x9B, 0xB0, 0x53, 0x64, 0x68, 0xA8, 0x9D, 0xB3, 0xA3, 0xF1,
    0x26, 0x97, 0x95, 0xA5, 0x99, 0xD3, 0xDA, 0xE5, 0x8A, 0xC1, 0xBC, 0xDD, 0xD1, 0xE8, 0xE2, 0xF8,
    0x02, 0x24, 0x09, 0x2F, 0x0E, 0x52, 0x59, 0x89, 0x18, 0x27, 0x2A, 0x9A, 0x3B, 0x96, 0x98, 0xD4,
    0x20, 0x34, 0x54, 0x9E, 0x5B, 0x67, 0x63, 0xBD, 0x46, 0x9C, 0x84, 0xD9, 0x69, 0xA4, 0xA9, 0xDC,
    0x1C, 0x4F, 0x4C, 0x8B, 0x36, 0x85, 0x8D, 0xB5, 0x3E, 0x5E, 0x73, 0xD2, 0x6F, 0xCC, 0xC2, 0xDF,
    0x39, 0xA2, 0x8F, 0xB2, 0x6B, 0xA7, 0xBA, 0xE7, 0x71, 0xAF, 0xB7, 0xF5, 0xAC, 0xE3, 0xF2, 0xFC,
    0x12, 0x2C, 0x40, 0x6E, 0x48, 0x81, 0x61, 0xC0, 0x43, 0x92, 0x77, 0xD7, 0x94, 0xBB, 0xA6, 0xE4,
    0x31, 0xA0, 0x5F, 0xD0, 0x65, 0xC3, 0xC8, 0xF0, 0x88, 0xC6, 0xB4, 0xED, 0xB1, 0xE0, 0xEA, 0xF7,
    0x57, 0x7D, 0x79, 0xAD, 0x7F, 0xB8, 0xC5, 0xF4, 0x75, 0xBE, 0xCA, 0xE9, 0xCF, 0xDB, 0xEE, 0xFA,
    0x7B, 0xAA, 0xCD, 0xE1, 0xD8, 0xEB, 0xDE, 0xF9, 0xD5, 0xEF, 0xE6, 0xFB, 0xF6, 0xFD, 0xFE, 0xFF,
];
