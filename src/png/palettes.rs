//! Built-in color palettes

pub mod crameric;
pub mod oceanographic;
pub mod viridic;

/// Kate Rose Morley's _12-bit Rainbow_ 12-color palette, plus black, white, 25%
/// gray and 75% gray.
pub static FOUR_BIT_RAINBOW: [u8; 48] = [
    0x00, 0x00, 0x00, 0x44, 0x44, 0x44, 0x88, 0x11, 0x77, 0xAA, 0x33, 0x55, 0xCC, 0x66, 0x66, 0xEE,
    0x99, 0x44, 0xEE, 0xDD, 0x00, 0x00, 0xDD, 0x55, 0x44, 0xDD, 0x88, 0x22, 0xCC, 0xBB, 0x00, 0xBB,
    0xCC, 0x00, 0x99, 0xCC, 0x33, 0x66, 0xBB, 0x66, 0x33, 0x99, 0xCC, 0xCC, 0xCC, 0xFF, 0xFF, 0xFF,
];

/// A `u8` to `u8` mapping that compresses all inputs into the upper half of
/// possible values.
///
/// This mapping is lossy/not bijective.
pub static MAP_HALF_UPPER: [u8; 256] = [
    0x80, 0x80, 0x81, 0x81, 0x82, 0x82, 0x83, 0x83, 0x84, 0x84, 0x85, 0x85, 0x86, 0x86, 0x87, 0x87,
    0x88, 0x88, 0x89, 0x89, 0x8A, 0x8A, 0x8B, 0x8B, 0x8C, 0x8C, 0x8D, 0x8D, 0x8E, 0x8E, 0x8F, 0x8F,
    0x90, 0x90, 0x91, 0x91, 0x92, 0x92, 0x93, 0x93, 0x94, 0x94, 0x95, 0x95, 0x96, 0x96, 0x97, 0x97,
    0x98, 0x98, 0x99, 0x99, 0x9A, 0x9A, 0x9B, 0x9B, 0x9C, 0x9C, 0x9D, 0x9D, 0x9E, 0x9E, 0x9F, 0x9F,
    0xA0, 0xA0, 0xA1, 0xA1, 0xA2, 0xA2, 0xA3, 0xA3, 0xA4, 0xA4, 0xA5, 0xA5, 0xA6, 0xA6, 0xA7, 0xA7,
    0xA8, 0xA8, 0xA9, 0xA9, 0xAA, 0xAA, 0xAB, 0xAB, 0xAC, 0xAC, 0xAD, 0xAD, 0xAE, 0xAE, 0xAF, 0xAF,
    0xB0, 0xB0, 0xB1, 0xB1, 0xB2, 0xB2, 0xB3, 0xB3, 0xB4, 0xB4, 0xB5, 0xB5, 0xB6, 0xB6, 0xB7, 0xB7,
    0xB8, 0xB8, 0xB9, 0xB9, 0xBA, 0xBA, 0xBB, 0xBB, 0xBC, 0xBC, 0xBD, 0xBD, 0xBE, 0xBE, 0xBF, 0xBF,
    0xC0, 0xC0, 0xC1, 0xC1, 0xC2, 0xC2, 0xC3, 0xC3, 0xC4, 0xC4, 0xC5, 0xC5, 0xC6, 0xC6, 0xC7, 0xC7,
    0xC8, 0xC8, 0xC9, 0xC9, 0xCA, 0xCA, 0xCB, 0xCB, 0xCC, 0xCC, 0xCD, 0xCD, 0xCE, 0xCE, 0xCF, 0xCF,
    0xD0, 0xD0, 0xD1, 0xD1, 0xD2, 0xD2, 0xD3, 0xD3, 0xD4, 0xD4, 0xD5, 0xD5, 0xD6, 0xD6, 0xD7, 0xD7,
    0xD8, 0xD8, 0xD9, 0xD9, 0xDA, 0xDA, 0xDB, 0xDB, 0xDC, 0xDC, 0xDD, 0xDD, 0xDE, 0xDE, 0xDF, 0xDF,
    0xE0, 0xE0, 0xE1, 0xE1, 0xE2, 0xE2, 0xE3, 0xE3, 0xE4, 0xE4, 0xE5, 0xE5, 0xE6, 0xE6, 0xE7, 0xE7,
    0xE8, 0xE8, 0xE9, 0xE9, 0xEA, 0xEA, 0xEB, 0xEB, 0xEC, 0xEC, 0xED, 0xED, 0xEE, 0xEE, 0xEF, 0xEF,
    0xF0, 0xF0, 0xF1, 0xF1, 0xF2, 0xF2, 0xF3, 0xF3, 0xF4, 0xF4, 0xF5, 0xF5, 0xF6, 0xF6, 0xF7, 0xF7,
    0xF8, 0xF8, 0xF9, 0xF9, 0xFA, 0xFA, 0xFB, 0xFB, 0xFC, 0xFC, 0xFD, 0xFD, 0xFE, 0xFE, 0xFF, 0xFF,
];

/// A `u8` to `u8` mapping that compresses all inputs into the lower half of
/// possible values.
///
/// This mapping is lossy/not bijective.
pub static MAP_HALF_LOWER: [u8; 256] = [
    0x00, 0x00, 0x01, 0x01, 0x02, 0x02, 0x03, 0x03, 0x04, 0x04, 0x05, 0x05, 0x06, 0x06, 0x07, 0x07,
    0x08, 0x08, 0x09, 0x09, 0x0A, 0x0A, 0x0B, 0x0B, 0x0C, 0x0C, 0x0D, 0x0D, 0x0E, 0x0E, 0x0F, 0x0F,
    0x10, 0x10, 0x11, 0x11, 0x12, 0x12, 0x13, 0x13, 0x14, 0x14, 0x15, 0x15, 0x16, 0x16, 0x17, 0x17,
    0x18, 0x18, 0x19, 0x19, 0x1A, 0x1A, 0x1B, 0x1B, 0x1C, 0x1C, 0x1D, 0x1D, 0x1E, 0x1E, 0x1F, 0x1F,
    0x20, 0x20, 0x21, 0x21, 0x22, 0x22, 0x23, 0x23, 0x24, 0x24, 0x25, 0x25, 0x26, 0x26, 0x27, 0x27,
    0x28, 0x28, 0x29, 0x29, 0x2A, 0x2A, 0x2B, 0x2B, 0x2C, 0x2C, 0x2D, 0x2D, 0x2E, 0x2E, 0x2F, 0x2F,
    0x30, 0x30, 0x31, 0x31, 0x32, 0x32, 0x33, 0x33, 0x34, 0x34, 0x35, 0x35, 0x36, 0x36, 0x37, 0x37,
    0x38, 0x38, 0x39, 0x39, 0x3A, 0x3A, 0x3B, 0x3B, 0x3C, 0x3C, 0x3D, 0x3D, 0x3E, 0x3E, 0x3F, 0x3F,
    0x40, 0x40, 0x41, 0x41, 0x42, 0x42, 0x43, 0x43, 0x44, 0x44, 0x45, 0x45, 0x46, 0x46, 0x47, 0x47,
    0x48, 0x48, 0x49, 0x49, 0x4A, 0x4A, 0x4B, 0x4B, 0x4C, 0x4C, 0x4D, 0x4D, 0x4E, 0x4E, 0x4F, 0x4F,
    0x50, 0x50, 0x51, 0x51, 0x52, 0x52, 0x53, 0x53, 0x54, 0x54, 0x55, 0x55, 0x56, 0x56, 0x57, 0x57,
    0x58, 0x58, 0x59, 0x59, 0x5A, 0x5A, 0x5B, 0x5B, 0x5C, 0x5C, 0x5D, 0x5D, 0x5E, 0x5E, 0x5F, 0x5F,
    0x60, 0x60, 0x61, 0x61, 0x62, 0x62, 0x63, 0x63, 0x64, 0x64, 0x65, 0x65, 0x66, 0x66, 0x67, 0x67,
    0x68, 0x68, 0x69, 0x69, 0x6A, 0x6A, 0x6B, 0x6B, 0x6C, 0x6C, 0x6D, 0x6D, 0x6E, 0x6E, 0x6F, 0x6F,
    0x70, 0x70, 0x71, 0x71, 0x72, 0x72, 0x73, 0x73, 0x74, 0x74, 0x75, 0x75, 0x76, 0x76, 0x77, 0x77,
    0x78, 0x78, 0x79, 0x79, 0x7A, 0x7A, 0x7B, 0x7B, 0x7C, 0x7C, 0x7D, 0x7D, 0x7E, 0x7E, 0x7F, 0x7F,
];

/// A `u8` to `u8` mapping that reverses the order of all values.
pub static MAP_REVERSED: [u8; 256] = [
    0xFF, 0xFE, 0xFD, 0xFC, 0xFB, 0xFA, 0xF9, 0xF8, 0xF7, 0xF6, 0xF5, 0xF4, 0xF3, 0xF2, 0xF1, 0xF0,
    0xEF, 0xEE, 0xED, 0xEC, 0xEB, 0xEA, 0xE9, 0xE8, 0xE7, 0xE6, 0xE5, 0xE4, 0xE3, 0xE2, 0xE1, 0xE0,
    0xDF, 0xDE, 0xDD, 0xDC, 0xDB, 0xDA, 0xD9, 0xD8, 0xD7, 0xD6, 0xD5, 0xD4, 0xD3, 0xD2, 0xD1, 0xD0,
    0xCF, 0xCE, 0xCD, 0xCC, 0xCB, 0xCA, 0xC9, 0xC8, 0xC7, 0xC6, 0xC5, 0xC4, 0xC3, 0xC2, 0xC1, 0xC0,
    0xBF, 0xBE, 0xBD, 0xBC, 0xBB, 0xBA, 0xB9, 0xB8, 0xB7, 0xB6, 0xB5, 0xB4, 0xB3, 0xB2, 0xB1, 0xB0,
    0xAF, 0xAE, 0xAD, 0xAC, 0xAB, 0xAA, 0xA9, 0xA8, 0xA7, 0xA6, 0xA5, 0xA4, 0xA3, 0xA2, 0xA1, 0xA0,
    0x9F, 0x9E, 0x9D, 0x9C, 0x9B, 0x9A, 0x99, 0x98, 0x97, 0x96, 0x95, 0x94, 0x93, 0x92, 0x91, 0x90,
    0x8F, 0x8E, 0x8D, 0x8C, 0x8B, 0x8A, 0x89, 0x88, 0x87, 0x86, 0x85, 0x84, 0x83, 0x82, 0x81, 0x80,
    0x7F, 0x7E, 0x7D, 0x7C, 0x7B, 0x7A, 0x79, 0x78, 0x77, 0x76, 0x75, 0x74, 0x73, 0x72, 0x71, 0x70,
    0x6F, 0x6E, 0x6D, 0x6C, 0x6B, 0x6A, 0x69, 0x68, 0x67, 0x66, 0x65, 0x64, 0x63, 0x62, 0x61, 0x60,
    0x5F, 0x5E, 0x5D, 0x5C, 0x5B, 0x5A, 0x59, 0x58, 0x57, 0x56, 0x55, 0x54, 0x53, 0x52, 0x51, 0x50,
    0x4F, 0x4E, 0x4D, 0x4C, 0x4B, 0x4A, 0x49, 0x48, 0x47, 0x46, 0x45, 0x44, 0x43, 0x42, 0x41, 0x40,
    0x3F, 0x3E, 0x3D, 0x3C, 0x3B, 0x3A, 0x39, 0x38, 0x37, 0x36, 0x35, 0x34, 0x33, 0x32, 0x31, 0x30,
    0x2F, 0x2E, 0x2D, 0x2C, 0x2B, 0x2A, 0x29, 0x28, 0x27, 0x26, 0x25, 0x24, 0x23, 0x22, 0x21, 0x20,
    0x1F, 0x1E, 0x1D, 0x1C, 0x1B, 0x1A, 0x19, 0x18, 0x17, 0x16, 0x15, 0x14, 0x13, 0x12, 0x11, 0x10,
    0x0F, 0x0E, 0x0D, 0x0C, 0x0B, 0x0A, 0x09, 0x08, 0x07, 0x06, 0x05, 0x04, 0x03, 0x02, 0x01, 0x00,
];

/// A `u8` to `u8` mapping that evenly splits the input space into two halves.
pub static MAP_STRATA_2: [u8; 256] = [
    0x00, 0x02, 0x04, 0x06, 0x08, 0x0A, 0x0C, 0x0E, 0x10, 0x12, 0x14, 0x16, 0x18, 0x1A, 0x1C, 0x1E,
    0x20, 0x22, 0x24, 0x26, 0x28, 0x2A, 0x2C, 0x2E, 0x30, 0x32, 0x34, 0x36, 0x38, 0x3A, 0x3C, 0x3E,
    0x40, 0x42, 0x44, 0x46, 0x48, 0x4A, 0x4C, 0x4E, 0x50, 0x52, 0x54, 0x56, 0x58, 0x5A, 0x5C, 0x5E,
    0x60, 0x62, 0x64, 0x66, 0x68, 0x6A, 0x6C, 0x6E, 0x70, 0x72, 0x74, 0x76, 0x78, 0x7A, 0x7C, 0x7E,
    0x80, 0x82, 0x84, 0x86, 0x88, 0x8A, 0x8C, 0x8E, 0x90, 0x92, 0x94, 0x96, 0x98, 0x9A, 0x9C, 0x9E,
    0xA0, 0xA2, 0xA4, 0xA6, 0xA8, 0xAA, 0xAC, 0xAE, 0xB0, 0xB2, 0xB4, 0xB6, 0xB8, 0xBA, 0xBC, 0xBE,
    0xC0, 0xC2, 0xC4, 0xC6, 0xC8, 0xCA, 0xCC, 0xCE, 0xD0, 0xD2, 0xD4, 0xD6, 0xD8, 0xDA, 0xDC, 0xDE,
    0xE0, 0xE2, 0xE4, 0xE6, 0xE8, 0xEA, 0xEC, 0xEE, 0xF0, 0xF2, 0xF4, 0xF6, 0xF8, 0xFA, 0xFC, 0xFE,
    0x01, 0x03, 0x05, 0x07, 0x09, 0x0B, 0x0D, 0x0F, 0x11, 0x13, 0x15, 0x17, 0x19, 0x1B, 0x1D, 0x1F,
    0x21, 0x23, 0x25, 0x27, 0x29, 0x2B, 0x2D, 0x2F, 0x31, 0x33, 0x35, 0x37, 0x39, 0x3B, 0x3D, 0x3F,
    0x41, 0x43, 0x45, 0x47, 0x49, 0x4B, 0x4D, 0x4F, 0x51, 0x53, 0x55, 0x57, 0x59, 0x5B, 0x5D, 0x5F,
    0x61, 0x63, 0x65, 0x67, 0x69, 0x6B, 0x6D, 0x6F, 0x71, 0x73, 0x75, 0x77, 0x79, 0x7B, 0x7D, 0x7F,
    0x81, 0x83, 0x85, 0x87, 0x89, 0x8B, 0x8D, 0x8F, 0x91, 0x93, 0x95, 0x97, 0x99, 0x9B, 0x9D, 0x9F,
    0xA1, 0xA3, 0xA5, 0xA7, 0xA9, 0xAB, 0xAD, 0xAF, 0xB1, 0xB3, 0xB5, 0xB7, 0xB9, 0xBB, 0xBD, 0xBF,
    0xC1, 0xC3, 0xC5, 0xC7, 0xC9, 0xCB, 0xCD, 0xCF, 0xD1, 0xD3, 0xD5, 0xD7, 0xD9, 0xDB, 0xDD, 0xDF,
    0xE1, 0xE3, 0xE5, 0xE7, 0xE9, 0xEB, 0xED, 0xEF, 0xF1, 0xF3, 0xF5, 0xF7, 0xF9, 0xFB, 0xFD, 0xFF,
];

/// A `u8` to `u8` mapping that evenly splits the input space into four
/// quarters.
pub static MAP_STRATA_4: [u8; 256] = [
    0x00, 0x04, 0x08, 0x0C, 0x10, 0x14, 0x18, 0x1C, 0x20, 0x24, 0x28, 0x2C, 0x30, 0x34, 0x38, 0x3C,
    0x40, 0x44, 0x48, 0x4C, 0x50, 0x54, 0x58, 0x5C, 0x60, 0x64, 0x68, 0x6C, 0x70, 0x74, 0x78, 0x7C,
    0x80, 0x84, 0x88, 0x8C, 0x90, 0x94, 0x98, 0x9C, 0xA0, 0xA4, 0xA8, 0xAC, 0xB0, 0xB4, 0xB8, 0xBC,
    0xC0, 0xC4, 0xC8, 0xCC, 0xD0, 0xD4, 0xD8, 0xDC, 0xE0, 0xE4, 0xE8, 0xEC, 0xF0, 0xF4, 0xF8, 0xFC,
    0x01, 0x05, 0x09, 0x0D, 0x11, 0x15, 0x19, 0x1D, 0x21, 0x25, 0x29, 0x2D, 0x31, 0x35, 0x39, 0x3D,
    0x41, 0x45, 0x49, 0x4D, 0x51, 0x55, 0x59, 0x5D, 0x61, 0x65, 0x69, 0x6D, 0x71, 0x75, 0x79, 0x7D,
    0x81, 0x85, 0x89, 0x8D, 0x91, 0x95, 0x99, 0x9D, 0xA1, 0xA5, 0xA9, 0xAD, 0xB1, 0xB5, 0xB9, 0xBD,
    0xC1, 0xC5, 0xC9, 0xCD, 0xD1, 0xD5, 0xD9, 0xDD, 0xE1, 0xE5, 0xE9, 0xED, 0xF1, 0xF5, 0xF9, 0xFD,
    0x02, 0x06, 0x0A, 0x0E, 0x12, 0x16, 0x1A, 0x1E, 0x22, 0x26, 0x2A, 0x2E, 0x32, 0x36, 0x3A, 0x3E,
    0x42, 0x46, 0x4A, 0x4E, 0x52, 0x56, 0x5A, 0x5E, 0x62, 0x66, 0x6A, 0x6E, 0x72, 0x76, 0x7A, 0x7E,
    0x82, 0x86, 0x8A, 0x8E, 0x92, 0x96, 0x9A, 0x9E, 0xA2, 0xA6, 0xAA, 0xAE, 0xB2, 0xB6, 0xBA, 0xBE,
    0xC2, 0xC6, 0xCA, 0xCE, 0xD2, 0xD6, 0xDA, 0xDE, 0xE2, 0xE6, 0xEA, 0xEE, 0xF2, 0xF6, 0xFA, 0xFE,
    0x03, 0x07, 0x0B, 0x0F, 0x13, 0x17, 0x1B, 0x1F, 0x23, 0x27, 0x2B, 0x2F, 0x33, 0x37, 0x3B, 0x3F,
    0x43, 0x47, 0x4B, 0x4F, 0x53, 0x57, 0x5B, 0x5F, 0x63, 0x67, 0x6B, 0x6F, 0x73, 0x77, 0x7B, 0x7F,
    0x83, 0x87, 0x8B, 0x8F, 0x93, 0x97, 0x9B, 0x9F, 0xA3, 0xA7, 0xAB, 0xAF, 0xB3, 0xB7, 0xBB, 0xBF,
    0xC3, 0xC7, 0xCB, 0xCF, 0xD3, 0xD7, 0xDB, 0xDF, 0xE3, 0xE7, 0xEB, 0xEF, 0xF3, 0xF7, 0xFB, 0xFF,
];

/// A `u8` to `u8` mapping that evenly splits the input space into eight
/// eighths.
pub static MAP_STRATA_8: [u8; 256] = [
    0x00, 0x08, 0x10, 0x18, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x58, 0x60, 0x68, 0x70, 0x78,
    0x80, 0x88, 0x90, 0x98, 0xA0, 0xA8, 0xB0, 0xB8, 0xC0, 0xC8, 0xD0, 0xD8, 0xE0, 0xE8, 0xF0, 0xF8,
    0x01, 0x09, 0x11, 0x19, 0x21, 0x29, 0x31, 0x39, 0x41, 0x49, 0x51, 0x59, 0x61, 0x69, 0x71, 0x79,
    0x81, 0x89, 0x91, 0x99, 0xA1, 0xA9, 0xB1, 0xB9, 0xC1, 0xC9, 0xD1, 0xD9, 0xE1, 0xE9, 0xF1, 0xF9,
    0x02, 0x0A, 0x12, 0x1A, 0x22, 0x2A, 0x32, 0x3A, 0x42, 0x4A, 0x52, 0x5A, 0x62, 0x6A, 0x72, 0x7A,
    0x82, 0x8A, 0x92, 0x9A, 0xA2, 0xAA, 0xB2, 0xBA, 0xC2, 0xCA, 0xD2, 0xDA, 0xE2, 0xEA, 0xF2, 0xFA,
    0x03, 0x0B, 0x13, 0x1B, 0x23, 0x2B, 0x33, 0x3B, 0x43, 0x4B, 0x53, 0x5B, 0x63, 0x6B, 0x73, 0x7B,
    0x83, 0x8B, 0x93, 0x9B, 0xA3, 0xAB, 0xB3, 0xBB, 0xC3, 0xCB, 0xD3, 0xDB, 0xE3, 0xEB, 0xF3, 0xFB,
    0x04, 0x0C, 0x14, 0x1C, 0x24, 0x2C, 0x34, 0x3C, 0x44, 0x4C, 0x54, 0x5C, 0x64, 0x6C, 0x74, 0x7C,
    0x84, 0x8C, 0x94, 0x9C, 0xA4, 0xAC, 0xB4, 0xBC, 0xC4, 0xCC, 0xD4, 0xDC, 0xE4, 0xEC, 0xF4, 0xFC,
    0x05, 0x0D, 0x15, 0x1D, 0x25, 0x2D, 0x35, 0x3D, 0x45, 0x4D, 0x55, 0x5D, 0x65, 0x6D, 0x75, 0x7D,
    0x85, 0x8D, 0x95, 0x9D, 0xA5, 0xAD, 0xB5, 0xBD, 0xC5, 0xCD, 0xD5, 0xDD, 0xE5, 0xED, 0xF5, 0xFD,
    0x06, 0x0E, 0x16, 0x1E, 0x26, 0x2E, 0x36, 0x3E, 0x46, 0x4E, 0x56, 0x5E, 0x66, 0x6E, 0x76, 0x7E,
    0x86, 0x8E, 0x96, 0x9E, 0xA6, 0xAE, 0xB6, 0xBE, 0xC6, 0xCE, 0xD6, 0xDE, 0xE6, 0xEE, 0xF6, 0xFE,
    0x07, 0x0F, 0x17, 0x1F, 0x27, 0x2F, 0x37, 0x3F, 0x47, 0x4F, 0x57, 0x5F, 0x67, 0x6F, 0x77, 0x7F,
    0x87, 0x8F, 0x97, 0x9F, 0xA7, 0xAF, 0xB7, 0xBF, 0xC7, 0xCF, 0xD7, 0xDF, 0xE7, 0xEF, 0xF7, 0xFF,
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

/// A `u8` to `u8` mapping that was randomly generated, but preserves the high
/// bit.
pub static MAP_SHUFFLED_IN_2: [u8; 256] = [0; 256];

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
