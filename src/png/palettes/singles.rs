//! Palettes that aren't part of a collection.

/// Google's Anton Mikhailov's [_turbo_ rainbow color map](https://ai.googleblog.com/2019/08/turbo-improved-rainbow-colormap-for.html) as an [`EightBit`][crate::EightBit]
/// palette.
pub static TURBO: &[u8] = &[
    0x30, 0x12, 0x3B, 0x31, 0x15, 0x42, 0x32, 0x18, 0x4A, 0x34, 0x1B, 0x51, 0x35, 0x1E, 0x58, 0x36,
    0x21, 0x5F, 0x37, 0x23, 0x65, 0x38, 0x26, 0x6C, 0x39, 0x29, 0x72, 0x3A, 0x2C, 0x79, 0x3B, 0x2F,
    0x7F, 0x3C, 0x32, 0x85, 0x3C, 0x35, 0x8B, 0x3D, 0x37, 0x91, 0x3E, 0x3A, 0x96, 0x3F, 0x3D, 0x9C,
    0x40, 0x40, 0xA1, 0x40, 0x43, 0xA6, 0x41, 0x45, 0xAB, 0x41, 0x48, 0xB0, 0x42, 0x4B, 0xB5, 0x43,
    0x4E, 0xBA, 0x43, 0x50, 0xBE, 0x43, 0x53, 0xC2, 0x44, 0x56, 0xC7, 0x44, 0x58, 0xCB, 0x45, 0x5B,
    0xCE, 0x45, 0x5E, 0xD2, 0x45, 0x60, 0xD6, 0x45, 0x63, 0xD9, 0x46, 0x66, 0xDD, 0x46, 0x68, 0xE0,
    0x46, 0x6B, 0xE3, 0x46, 0x6D, 0xE6, 0x46, 0x70, 0xE8, 0x46, 0x73, 0xEB, 0x46, 0x75, 0xED, 0x46,
    0x78, 0xF0, 0x46, 0x7A, 0xF2, 0x46, 0x7D, 0xF4, 0x46, 0x7F, 0xF6, 0x46, 0x82, 0xF8, 0x45, 0x84,
    0xF9, 0x45, 0x87, 0xFB, 0x45, 0x89, 0xFC, 0x44, 0x8C, 0xFD, 0x43, 0x8E, 0xFD, 0x42, 0x91, 0xFE,
    0x41, 0x93, 0xFE, 0x40, 0x96, 0xFE, 0x3F, 0x98, 0xFE, 0x3E, 0x9B, 0xFE, 0x3C, 0x9D, 0xFD, 0x3B,
    0xA0, 0xFC, 0x39, 0xA2, 0xFC, 0x38, 0xA5, 0xFB, 0x36, 0xA8, 0xF9, 0x34, 0xAA, 0xF8, 0x33, 0xAC,
    0xF6, 0x31, 0xAF, 0xF5, 0x2F, 0xB1, 0xF3, 0x2D, 0xB4, 0xF1, 0x2B, 0xB6, 0xEF, 0x2A, 0xB9, 0xED,
    0x28, 0xBB, 0xEB, 0x26, 0xBD, 0xE9, 0x25, 0xC0, 0xE6, 0x23, 0xC2, 0xE4, 0x21, 0xC4, 0xE1, 0x20,
    0xC6, 0xDF, 0x1E, 0xC9, 0xDC, 0x1D, 0xCB, 0xDA, 0x1C, 0xCD, 0xD7, 0x1B, 0xCF, 0xD4, 0x1A, 0xD1,
    0xD2, 0x19, 0xD3, 0xCF, 0x18, 0xD5, 0xCC, 0x18, 0xD7, 0xCA, 0x17, 0xD9, 0xC7, 0x17, 0xDA, 0xC4,
    0x17, 0xDC, 0xC2, 0x17, 0xDE, 0xBF, 0x18, 0xE0, 0xBD, 0x18, 0xE1, 0xBA, 0x19, 0xE3, 0xB8, 0x1A,
    0xE4, 0xB6, 0x1B, 0xE5, 0xB4, 0x1D, 0xE7, 0xB1, 0x1E, 0xE8, 0xAF, 0x20, 0xE9, 0xAC, 0x22, 0xEB,
    0xA9, 0x24, 0xEC, 0xA6, 0x27, 0xED, 0xA3, 0x29, 0xEE, 0xA0, 0x2C, 0xEF, 0x9D, 0x2F, 0xF0, 0x9A,
    0x32, 0xF1, 0x97, 0x35, 0xF3, 0x94, 0x38, 0xF4, 0x91, 0x3B, 0xF4, 0x8D, 0x3F, 0xF5, 0x8A, 0x42,
    0xF6, 0x87, 0x46, 0xF7, 0x83, 0x4A, 0xF8, 0x80, 0x4D, 0xF9, 0x7C, 0x51, 0xF9, 0x79, 0x55, 0xFA,
    0x76, 0x59, 0xFB, 0x72, 0x5D, 0xFB, 0x6F, 0x61, 0xFC, 0x6C, 0x65, 0xFC, 0x68, 0x69, 0xFD, 0x65,
    0x6D, 0xFD, 0x62, 0x71, 0xFD, 0x5F, 0x74, 0xFE, 0x5C, 0x78, 0xFE, 0x59, 0x7C, 0xFE, 0x56, 0x80,
    0xFE, 0x53, 0x84, 0xFE, 0x50, 0x87, 0xFE, 0x4D, 0x8B, 0xFE, 0x4B, 0x8E, 0xFE, 0x48, 0x92, 0xFE,
    0x46, 0x95, 0xFE, 0x44, 0x98, 0xFE, 0x42, 0x9B, 0xFD, 0x40, 0x9E, 0xFD, 0x3E, 0xA1, 0xFC, 0x3D,
    0xA4, 0xFC, 0x3B, 0xA6, 0xFB, 0x3A, 0xA9, 0xFB, 0x39, 0xAC, 0xFA, 0x37, 0xAE, 0xF9, 0x37, 0xB1,
    0xF8, 0x36, 0xB3, 0xF8, 0x35, 0xB6, 0xF7, 0x35, 0xB9, 0xF5, 0x34, 0xBB, 0xF4, 0x34, 0xBE, 0xF3,
    0x34, 0xC0, 0xF2, 0x33, 0xC3, 0xF1, 0x33, 0xC5, 0xEF, 0x33, 0xC8, 0xEE, 0x33, 0xCA, 0xED, 0x33,
    0xCD, 0xEB, 0x34, 0xCF, 0xEA, 0x34, 0xD1, 0xE8, 0x34, 0xD4, 0xE7, 0x35, 0xD6, 0xE5, 0x35, 0xD8,
    0xE3, 0x35, 0xDA, 0xE2, 0x36, 0xDD, 0xE0, 0x36, 0xDF, 0xDE, 0x36, 0xE1, 0xDC, 0x37, 0xE3, 0xDA,
    0x37, 0xE5, 0xD8, 0x38, 0xE7, 0xD7, 0x38, 0xE8, 0xD5, 0x38, 0xEA, 0xD3, 0x39, 0xEC, 0xD1, 0x39,
    0xED, 0xCF, 0x39, 0xEF, 0xCD, 0x39, 0xF0, 0xCB, 0x3A, 0xF2, 0xC8, 0x3A, 0xF3, 0xC6, 0x3A, 0xF4,
    0xC4, 0x3A, 0xF6, 0xC2, 0x3A, 0xF7, 0xC0, 0x39, 0xF8, 0xBE, 0x39, 0xF9, 0xBC, 0x39, 0xF9, 0xBA,
    0x38, 0xFA, 0xB7, 0x37, 0xFB, 0xB5, 0x37, 0xFB, 0xB3, 0x36, 0xFC, 0xB0, 0x35, 0xFC, 0xAE, 0x34,
    0xFD, 0xAB, 0x33, 0xFD, 0xA9, 0x32, 0xFD, 0xA6, 0x31, 0xFD, 0xA3, 0x30, 0xFE, 0xA1, 0x2F, 0xFE,
    0x9E, 0x2E, 0xFE, 0x9B, 0x2D, 0xFE, 0x98, 0x2C, 0xFD, 0x95, 0x2B, 0xFD, 0x92, 0x29, 0xFD, 0x8F,
    0x28, 0xFD, 0x8C, 0x27, 0xFC, 0x89, 0x26, 0xFC, 0x86, 0x24, 0xFB, 0x83, 0x23, 0xFB, 0x80, 0x22,
    0xFA, 0x7D, 0x20, 0xFA, 0x7A, 0x1F, 0xF9, 0x77, 0x1E, 0xF8, 0x74, 0x1C, 0xF7, 0x71, 0x1B, 0xF7,
    0x6E, 0x1A, 0xF6, 0x6B, 0x18, 0xF5, 0x68, 0x17, 0xF4, 0x65, 0x16, 0xF3, 0x63, 0x15, 0xF2, 0x60,
    0x14, 0xF1, 0x5D, 0x13, 0xEF, 0x5A, 0x11, 0xEE, 0x58, 0x10, 0xED, 0x55, 0x0F, 0xEC, 0x52, 0x0E,
    0xEA, 0x50, 0x0D, 0xE9, 0x4D, 0x0D, 0xE8, 0x4B, 0x0C, 0xE6, 0x49, 0x0B, 0xE5, 0x46, 0x0A, 0xE3,
    0x44, 0x0A, 0xE2, 0x42, 0x09, 0xE0, 0x40, 0x08, 0xDE, 0x3E, 0x08, 0xDD, 0x3C, 0x07, 0xDB, 0x3A,
    0x07, 0xD9, 0x38, 0x06, 0xD7, 0x36, 0x06, 0xD6, 0x34, 0x05, 0xD4, 0x32, 0x05, 0xD2, 0x30, 0x05,
    0xD0, 0x2F, 0x04, 0xCE, 0x2D, 0x04, 0xCB, 0x2B, 0x03, 0xC9, 0x29, 0x03, 0xC7, 0x28, 0x03, 0xC5,
    0x26, 0x02, 0xC3, 0x24, 0x02, 0xC0, 0x23, 0x02, 0xBE, 0x21, 0x02, 0xBB, 0x1F, 0x01, 0xB9, 0x1E,
    0x01, 0xB6, 0x1C, 0x01, 0xB4, 0x1B, 0x01, 0xB1, 0x19, 0x01, 0xAE, 0x18, 0x01, 0xAC, 0x16, 0x01,
    0xA9, 0x15, 0x01, 0xA6, 0x14, 0x01, 0xA3, 0x12, 0x01, 0xA0, 0x11, 0x01, 0x9D, 0x10, 0x01, 0x9A,
    0x0E, 0x01, 0x97, 0x0D, 0x01, 0x94, 0x0C, 0x01, 0x91, 0x0B, 0x01, 0x8E, 0x0A, 0x01, 0x8B, 0x09,
    0x01, 0x87, 0x08, 0x01, 0x84, 0x07, 0x01, 0x81, 0x06, 0x02, 0x7D, 0x05, 0x02, 0x7A, 0x04, 0x02,
];

/// Jamie R. Nuñez et al's [_cividis_ color map](https://doi.org/10.1371/journal.pone.0199239), a modification of
/// [`VIRIDIS`][crate::palettes::viridis::VIRIDIS] optimized for
/// color-vision-deficient viewers, as an [`EightBit`][crate::EightBit]
/// palette.
pub static CIVIDIS: &[u8] = &[
    0x00, 0x20, 0x4C, 0x00, 0x20, 0x4E, 0x00, 0x21, 0x50, 0x00, 0x22, 0x51, 0x00, 0x23, 0x53, 0x00,
    0x23, 0x55, 0x00, 0x24, 0x56, 0x00, 0x25, 0x58, 0x00, 0x26, 0x5A, 0x00, 0x26, 0x5B, 0x00, 0x27,
    0x5D, 0x00, 0x28, 0x5F, 0x00, 0x28, 0x61, 0x00, 0x29, 0x63, 0x00, 0x2A, 0x64, 0x00, 0x2A, 0x66,
    0x00, 0x2B, 0x68, 0x00, 0x2C, 0x6A, 0x00, 0x2D, 0x6C, 0x00, 0x2D, 0x6D, 0x00, 0x2E, 0x6E, 0x00,
    0x2E, 0x6F, 0x00, 0x2F, 0x6F, 0x00, 0x2F, 0x6F, 0x00, 0x30, 0x6F, 0x00, 0x31, 0x6F, 0x00, 0x31,
    0x6F, 0x00, 0x32, 0x6E, 0x00, 0x33, 0x6E, 0x00, 0x34, 0x6E, 0x00, 0x34, 0x6E, 0x01, 0x35, 0x6E,
    0x06, 0x36, 0x6E, 0x0A, 0x37, 0x6D, 0x0E, 0x37, 0x6D, 0x12, 0x38, 0x6D, 0x15, 0x39, 0x6D, 0x17,
    0x39, 0x6D, 0x1A, 0x3A, 0x6C, 0x1C, 0x3B, 0x6C, 0x1E, 0x3C, 0x6C, 0x20, 0x3C, 0x6C, 0x22, 0x3D,
    0x6C, 0x24, 0x3E, 0x6C, 0x26, 0x3E, 0x6C, 0x27, 0x3F, 0x6C, 0x29, 0x40, 0x6B, 0x2B, 0x41, 0x6B,
    0x2C, 0x41, 0x6B, 0x2E, 0x42, 0x6B, 0x2F, 0x43, 0x6B, 0x31, 0x44, 0x6B, 0x32, 0x44, 0x6B, 0x33,
    0x45, 0x6B, 0x35, 0x46, 0x6B, 0x36, 0x46, 0x6B, 0x37, 0x47, 0x6B, 0x38, 0x48, 0x6B, 0x3A, 0x49,
    0x6B, 0x3B, 0x49, 0x6B, 0x3C, 0x4A, 0x6B, 0x3D, 0x4B, 0x6B, 0x3E, 0x4B, 0x6B, 0x40, 0x4C, 0x6B,
    0x41, 0x4D, 0x6B, 0x42, 0x4E, 0x6B, 0x43, 0x4E, 0x6B, 0x44, 0x4F, 0x6B, 0x45, 0x50, 0x6B, 0x46,
    0x50, 0x6B, 0x47, 0x51, 0x6B, 0x48, 0x52, 0x6B, 0x49, 0x53, 0x6B, 0x4A, 0x53, 0x6B, 0x4B, 0x54,
    0x6B, 0x4C, 0x55, 0x6B, 0x4D, 0x55, 0x6B, 0x4E, 0x56, 0x6B, 0x4F, 0x57, 0x6C, 0x50, 0x58, 0x6C,
    0x51, 0x58, 0x6C, 0x52, 0x59, 0x6C, 0x53, 0x5A, 0x6C, 0x54, 0x5A, 0x6C, 0x55, 0x5B, 0x6C, 0x56,
    0x5C, 0x6C, 0x57, 0x5D, 0x6D, 0x58, 0x5D, 0x6D, 0x59, 0x5E, 0x6D, 0x5A, 0x5F, 0x6D, 0x5B, 0x5F,
    0x6D, 0x5C, 0x60, 0x6D, 0x5D, 0x61, 0x6E, 0x5E, 0x62, 0x6E, 0x5F, 0x62, 0x6E, 0x5F, 0x63, 0x6E,
    0x60, 0x64, 0x6E, 0x61, 0x65, 0x6F, 0x62, 0x65, 0x6F, 0x63, 0x66, 0x6F, 0x64, 0x67, 0x6F, 0x65,
    0x67, 0x6F, 0x66, 0x68, 0x70, 0x67, 0x69, 0x70, 0x68, 0x6A, 0x70, 0x68, 0x6A, 0x70, 0x69, 0x6B,
    0x71, 0x6A, 0x6C, 0x71, 0x6B, 0x6D, 0x71, 0x6C, 0x6D, 0x72, 0x6D, 0x6E, 0x72, 0x6E, 0x6F, 0x72,
    0x6F, 0x6F, 0x72, 0x6F, 0x70, 0x73, 0x70, 0x71, 0x73, 0x71, 0x72, 0x73, 0x72, 0x72, 0x74, 0x73,
    0x73, 0x74, 0x74, 0x74, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x75, 0x76, 0x76, 0x76, 0x77, 0x77,
    0x76, 0x78, 0x78, 0x76, 0x79, 0x78, 0x77, 0x7A, 0x79, 0x77, 0x7B, 0x7A, 0x77, 0x7B, 0x7B, 0x78,
    0x7C, 0x7B, 0x78, 0x7D, 0x7C, 0x78, 0x7E, 0x7D, 0x78, 0x7F, 0x7E, 0x78, 0x80, 0x7E, 0x78, 0x81,
    0x7F, 0x78, 0x82, 0x80, 0x78, 0x83, 0x81, 0x78, 0x84, 0x81, 0x78, 0x85, 0x82, 0x78, 0x86, 0x83,
    0x78, 0x87, 0x84, 0x78, 0x88, 0x85, 0x78, 0x89, 0x85, 0x78, 0x8A, 0x86, 0x78, 0x8B, 0x87, 0x78,
    0x8C, 0x88, 0x78, 0x8D, 0x88, 0x78, 0x8E, 0x89, 0x78, 0x8F, 0x8A, 0x78, 0x90, 0x8B, 0x78, 0x91,
    0x8C, 0x78, 0x92, 0x8C, 0x78, 0x93, 0x8D, 0x78, 0x94, 0x8E, 0x78, 0x95, 0x8F, 0x78, 0x96, 0x8F,
    0x77, 0x97, 0x90, 0x77, 0x98, 0x91, 0x77, 0x99, 0x92, 0x77, 0x9A, 0x93, 0x77, 0x9B, 0x93, 0x77,
    0x9C, 0x94, 0x77, 0x9D, 0x95, 0x77, 0x9E, 0x96, 0x76, 0x9F, 0x97, 0x76, 0xA0, 0x98, 0x76, 0xA1,
    0x98, 0x76, 0xA2, 0x99, 0x76, 0xA3, 0x9A, 0x75, 0xA4, 0x9B, 0x75, 0xA5, 0x9C, 0x75, 0xA6, 0x9C,
    0x75, 0xA7, 0x9D, 0x75, 0xA8, 0x9E, 0x74, 0xA9, 0x9F, 0x74, 0xAA, 0xA0, 0x74, 0xAB, 0xA1, 0x74,
    0xAC, 0xA1, 0x73, 0xAD, 0xA2, 0x73, 0xAE, 0xA3, 0x73, 0xAF, 0xA4, 0x73, 0xB0, 0xA5, 0x72, 0xB1,
    0xA6, 0x72, 0xB2, 0xA6, 0x72, 0xB4, 0xA7, 0x71, 0xB5, 0xA8, 0x71, 0xB6, 0xA9, 0x71, 0xB7, 0xAA,
    0x70, 0xB8, 0xAB, 0x70, 0xB9, 0xAB, 0x70, 0xBA, 0xAC, 0x6F, 0xBB, 0xAD, 0x6F, 0xBC, 0xAE, 0x6E,
    0xBD, 0xAF, 0x6E, 0xBE, 0xB0, 0x6E, 0xBF, 0xB1, 0x6D, 0xC0, 0xB1, 0x6D, 0xC1, 0xB2, 0x6C, 0xC2,
    0xB3, 0x6C, 0xC3, 0xB4, 0x6C, 0xC5, 0xB5, 0x6B, 0xC6, 0xB6, 0x6B, 0xC7, 0xB7, 0x6A, 0xC8, 0xB8,
    0x6A, 0xC9, 0xB8, 0x69, 0xCA, 0xB9, 0x69, 0xCB, 0xBA, 0x68, 0xCC, 0xBB, 0x68, 0xCD, 0xBC, 0x67,
    0xCE, 0xBD, 0x67, 0xD0, 0xBE, 0x66, 0xD1, 0xBF, 0x66, 0xD2, 0xC0, 0x65, 0xD3, 0xC0, 0x65, 0xD4,
    0xC1, 0x64, 0xD5, 0xC2, 0x63, 0xD6, 0xC3, 0x63, 0xD7, 0xC4, 0x62, 0xD8, 0xC5, 0x61, 0xD9, 0xC6,
    0x61, 0xDB, 0xC7, 0x60, 0xDC, 0xC8, 0x60, 0xDD, 0xC9, 0x5F, 0xDE, 0xCA, 0x5E, 0xDF, 0xCB, 0x5D,
    0xE0, 0xCB, 0x5D, 0xE1, 0xCC, 0x5C, 0xE3, 0xCD, 0x5B, 0xE4, 0xCE, 0x5B, 0xE5, 0xCF, 0x5A, 0xE6,
    0xD0, 0x59, 0xE7, 0xD1, 0x58, 0xE8, 0xD2, 0x57, 0xE9, 0xD3, 0x56, 0xEB, 0xD4, 0x56, 0xEC, 0xD5,
    0x55, 0xED, 0xD6, 0x54, 0xEE, 0xD7, 0x53, 0xEF, 0xD8, 0x52, 0xF0, 0xD9, 0x51, 0xF1, 0xDA, 0x50,
    0xF3, 0xDB, 0x4F, 0xF4, 0xDC, 0x4E, 0xF5, 0xDD, 0x4D, 0xF6, 0xDE, 0x4C, 0xF7, 0xDF, 0x4B, 0xF9,
    0xE0, 0x49, 0xFA, 0xE0, 0x48, 0xFB, 0xE1, 0x47, 0xFC, 0xE2, 0x46, 0xFD, 0xE3, 0x45, 0xFF, 0xE4,
    0x43, 0xFF, 0xE5, 0x42, 0xFF, 0xE6, 0x42, 0xFF, 0xE7, 0x43, 0xFF, 0xE8, 0x44, 0xFF, 0xE9, 0x45,
];

/// Kate Rose Morley's [_12-bit Rainbow_ 12-color palette](https://iamkate.com/data/12-bit-rainbow/),
/// plus black, white, 25% gray and 75% gray, as a [`FourBit`][crate::FourBit]
/// palette.
pub static FOUR_BIT_RAINBOW: [u8; 48] = [
    0x00, 0x00, 0x00, 0x44, 0x44, 0x44, 0x88, 0x11, 0x77, 0xAA, 0x33, 0x55, 0xCC, 0x66, 0x66, 0xEE,
    0x99, 0x44, 0xEE, 0xDD, 0x00, 0x00, 0xDD, 0x55, 0x44, 0xDD, 0x88, 0x22, 0xCC, 0xBB, 0x00, 0xBB,
    0xCC, 0x00, 0x99, 0xCC, 0x33, 0x66, 0xBB, 0x66, 0x33, 0x99, 0xCC, 0xCC, 0xCC, 0xFF, 0xFF, 0xFF,
];

/// Lexaloffle Games' Joseph White's [_PICO-8 fantasy console_](https://www.lexaloffle.com/pico-8.php)'s
/// default [`FourBit`][crate::FourBit] palette.
pub static FOUR_BIT_PICO: [u8; 48] = [
    0x00, 0x00, 0x00, 0x1D, 0x2B, 0x53, 0x7E, 0x25, 0x53, 0x00, 0x87, 0x51, 0xAB, 0x52, 0x36, 0x5F,
    0x57, 0x4F, 0xC2, 0xC3, 0xC7, 0xFF, 0xF1, 0xE8, 0xFF, 0x00, 0x4D, 0xFF, 0xA3, 0x00, 0xFF, 0xEC,
    0x27, 0x00, 0xE4, 0x36, 0x29, 0xAD, 0xFF, 0x83, 0x76, 0x9C, 0xFF, 0x77, 0xA8, 0xFF, 0xCC, 0xAA,
];