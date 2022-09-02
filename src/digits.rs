use std::fmt::Debug;
use std::fmt::Display;
use std::io::Write;
use std::marker::PhantomData;
use std::str::FromStr;

use miette::Diagnostic;
use thiserror::Error;

trait N: Into<u64> + From<u32> + TryFrom<u64> {
    const MAX: u64;
}

impl N for u32 {
    const MAX: u64 = u32::MAX.into();
}

impl N for u64 {
    const MAX: u64 = u64::MAX;
}

#[derive(Default, Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Digitizer<const DIGITS: u8, N: self::N> {
    _n: PhantomData<N>,
}

static ALPHABET_B64: [char; 64] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'J',
    'K', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X', 'Y', 'Z', 'L', 'I', 'O', 'U', 'a', 'b',
    'c', 'd', 'e', 'f', 'g', 'h', 'j', 'k', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'y',
    'z', 'l', 'i', 'o', 'u', '=', '-',
];

static ALPHABET_B32: &[char; 32] = ALPHABET_B64[..32].try_into().unwrap();

static PREFIXES_HI: [char; 15] = [
    '\u{300}', '\u{301}', '\u{302}', '\u{303}', '\u{304}', '\u{306}', '\u{307}', '\u{308}',
    '\u{30A}', '\u{30C}', '\u{33F}', '\u{313}', '\u{311}', '\u{30E}', '\u{346}',
];
static PREFIXES_LO: [char; 15] = [
    '\u{316}', '\u{317}', '\u{32D}', '\u{330}', '\u{331}', '\u{32E}', '\u{323}', '\u{324}',
    '\u{325}', '\u{32C}', '\u{333}', '\u{326}', '\u{31E}', '\u{348}', '\u{33A}',
];

// 16 bits per digit (letting us pack 128 bits in 8 visible characters)?`
// So UUIDs can fit in our 10 character scheme using the `UU` prefix
// which is fixed at 16 bits per digit (since UUID-style IDs don't have
// a distribution that would benefit from dynamic encodings)
// What's 128 encoded into two characters using our dynamic base scheme?
//
// These seem unreadable. What else can we do?
// Maybe we do support stacking accents, and define a zero accent that is only
// used when stacking? depends if all of the combining characters support
// actually combining like that.
static COMBINE_OUTSIDE_1_CIRCLE: char = '\u{20DD}';
static COMBINE_OUTSIDE_2_SQUARE: char = '\u{20DE}';
static COMBINE_OUTSIDE_3_DIAMOND: char = '\u{20DF}';

// This doesn't need to be generic.
// Or at least, the trait doesn't. Convert on output if you insist.
// Just operate on u64s. Nothing needs to be
impl<const DIGITS: u8, N: self::N> Digitizer<DIGITS, N> {
    fn max_input() -> Input {
        Input::from(u32::max_value())
    }

    fn b10_min_output() -> Input {
        0.into()
    }

    /// Maximum input value that will be represented in base 10.
    fn b10_max_input() -> Input {
        let mut max: u64 = 1;
        for _ in 0..DIGITS {
            max *= 10;
        }
        max -= 1;
        max.try_into().unwrap()
    }

    /// Minimum input value that will be represented in base 16.
    fn b16_min_input() -> Input {
        (Self::b10_max_input().into() + 1).try_into().unwrap()
    }

    /// Minimum value as display in encoded output when using base 16.
    fn b16_min_output() -> Input {
        let mut max: u64 = 1;
        for _ in 1..DIGITS {
            max *= 16;
        }
        max *= 10;
        max -= 1;
        max.try_into().unwrap()
    }

    /// Maximum input value that will be represented in base 16.
    fn b16_max_input() -> Input {
        let mut max: u64 = 1;
        for _ in 0..DIGITS {
            max *= 16;
        }
        max -= Self::b16_min_output().into();
        max += Self::b16_min_input().into();
        max.try_into().unwrap()
    }

    /// Minimum input value that will be represented in base 32.
    fn b32_min_input() -> Input {
        (Self::b10_max_input().into() + 1).try_into().unwrap()
    }

    fn b32_min_output() -> Input {
        let mut max: u64 = 1;
        for _ in 1..DIGITS {
            max *= 32;
        }
        max *= 16;
        max -= 1;
        max.try_into().unwrap()
    }

    /// Maximum input value that can be represented (in base 32, the largest
    /// supported option).
    fn b32_max_input() -> Input {
        let mut max: u64 = 1;
        for _ in 0..DIGITS {
            max *= 32;
        }
        max -= Self::b32_min_output().into();
        max += Self::b32_min_input().into();
        max.try_into().unwrap()
    }
}

static B10_MAX_7: u32 = 0___9_999_999;
static B16_MAX_7: u32 = 0x__F_FFF_FFF;
static B16_PAD_7: u32 = 0x__9_FFF_FFF;
static B32_PAD_7: u64 = 0x43F_FFF_FFF;

static B10_MAX_8: u64 = 0____99_999_999;
static B16_MAX_8: u64 = 0x___FF_FFF_FFF;
static B16_PAD_8: u64 = 0x___9F_FFF_FFF;
static B32_PAD_8: u64 = 0x4_3FF_FFF_FFF;

/// Encodes a u32 as exactly 7 digits or uppercase letters.
///
/// If it fits in 7 b10imal digits, then zero-padded b10imal is used.
/// If it fits in 7 b16ab10imal digits, then b16ab10imal is used, but offset
/// such that the first digit is always a >= 'A' to avoid confusion with
/// b10imal. Otherwise base 36 is used, but offset such that the first digit is
/// always >= 'G' to avoid confusion with b16 and b10imal.
pub fn sept(n: u32) -> String {
    let n = u64::from(n);

    let mut chars = vec![b'?'; 7];
    if n <= u64::from(B10_MAX_7) {
        let mut r = n;
        for (index, digit) in chars.iter_mut().enumerate().rev() {
            let d = r % 10;
            r /= 10;
            *digit = DIGITS_B10[d as usize];
        }
        debug_assert!(r == 0);
    } else if n <= (B16_MAX_7 - (B16_PAD_7 - B10_MAX_7)).into() {
        let mut r: u64 = n - u64::from(B10_MAX_7) + u64::from(B16_PAD_7);
        for (index, digit) in chars.iter_mut().enumerate().rev() {
            let d = r % 16;
            r /= 16;
            *digit = DIGITS_B16[d as usize];
        }
        debug_assert!(r == 0);
    } else {
        let mut r: u64 = n - u64::from(B16_MAX_7 - (B16_PAD_7 - B10_MAX_7)) + B32_PAD_7;
        for (index, digit) in chars.iter_mut().enumerate().rev() {
            let d = r % 32;
            r /= 32;
            *digit = DIGITS_B32[d as usize];
        }
        debug_assert!(r == 0);
    }

    String::from_utf8(chars).unwrap()
}

/// Encodes a u32 as exactly 8 digits or uppercase letters.
///
/// If it fits in 8 b10imal digits, then zero-padded b10imal is used.
/// If it fits in 8 b16ab10imal digits, then b16ab10imal is used, but offset
/// such that the first digit is always a >= 'A' to avoid confusion with
/// b10imal. Otherwise base 36 is used, but offset such that the first digit is
/// always >= 'G' to avoid confusion with b16 and b10imal.
pub fn octo(n: u64) -> Result<String, DigitEncodingError> {
    let mut chars = vec![b'?'; 8];
    if n <= u64::from(B10_MAX_8) {
        let mut r = n;
        for (index, digit) in chars.iter_mut().enumerate().rev() {
            let d = r % 10;
            r /= 10;
            *digit = DIGITS_B10[d as usize];
        }
        if (r > 0) {
            return Err(DigitEncodingError::OutOfBounds);
        }
    } else if n <= (B16_MAX_8 - (B16_PAD_8 - B10_MAX_8)).into() {
        let mut r: u64 = n - u64::from(B10_MAX_8) + u64::from(B16_PAD_8);
        for (index, digit) in chars.iter_mut().enumerate().rev() {
            let d = r % 16;
            r /= 16;
            *digit = DIGITS_B16[d as usize];
        }
        if (r > 0) {
            return Err(DigitEncodingError::OutOfBounds);
        }
    } else {
        let mut r: u64 = n - u64::from(B16_MAX_8 - (B16_PAD_8 - B10_MAX_8)) + B32_PAD_8;
        for (index, digit) in chars.iter_mut().enumerate().rev() {
            let d = r % 32;
            r /= 32;
            *digit = DIGITS_B32[d as usize];
        }
        if (r > 0) {
            return Err(DigitEncodingError::OutOfBounds);
        }
    }

    Ok(String::from_utf8(chars).unwrap())
}

#[derive(Debug, Error, Diagnostic, Clone, PartialEq, Eq, Hash)]
pub enum DigitDecodingError {
    #[error("invalid character")]
    InvalidDigit,

    #[error("invalid length")]
    InvalidLength,

    #[error("out of bounds")]
    OutOfBounds,
}

#[derive(Debug, Error, Diagnostic, Clone, PartialEq, Eq, Hash)]
pub enum DigitEncodingError {
    #[error("out of bounds")]
    OutOfBounds,
}

pub fn desept(digits: impl AsRef<[u8]>) -> Result<u32, DigitDecodingError> {
    if digits.as_ref().len() != 7 {
        return Err(DigitDecodingError::InvalidLength);
    }

    let mut digits: Vec<u8> = digits.as_ref().iter().copied().collect();

    for char in digits.iter_mut() {
        match char {
            b'O' | b'o' => *char = b'0',
            b'I' | b'L' | b'i' | b'l' => *char = b'1',
            b'a'..=b'z' => char.make_ascii_uppercase(),
            b'A'..=b'Z' | b'0'..=b'9' => {},
            _ => return Err(DigitDecodingError::InvalidDigit),
        }
    }

    match digits[0] {
        b'0'..=b'9' => {},
        b'A'..=b'F' => {},
        b'G'..=b'Z' => {},
        _ => return Err(DigitDecodingError::InvalidDigit),
    }

    todo!()
}

static SEPT_PAIRS: &[(&str, Result<u32, DigitDecodingError>)] = &[
    ("", Err(DigitDecodingError::InvalidLength)),
    ("0", Err(DigitDecodingError::InvalidLength)),
    ("?", Err(DigitDecodingError::InvalidLength)),
    ("000000", Err(DigitDecodingError::InvalidLength)),
    ("00000000", Err(DigitDecodingError::InvalidLength)),
    ("000-000", Err(DigitDecodingError::InvalidDigit)),
    ("0000000", Ok(0_____0)),
    ("0000001", Ok(0_____1)),
    ("0000002", Ok(0_____2)),
    ("0025137", Ok(0_25137)),
    ("0062074", Ok(0_62074)),
    ("0101010", Ok(0101010)),
    ("1010101", Ok(1010101)),
    ("1111111", Ok(1111111)),
    ("1118956", Ok(1118956)),
    ("1401653", Ok(1401653)),
    ("2222222", Ok(2222222)),
    ("3333333", Ok(3333333)),
    ("4444444", Ok(4444444)),
    ("5555555", Ok(5555555)),
    ("6666666", Ok(6666666)),
    ("7777777", Ok(7777777)),
    ("8888888", Ok(8888888)),
    ("9999997", Ok(B10_MAX_7 - 2)),
    ("9999998", Ok(B10_MAX_7 - 1)),
    ("9999999", Ok(9999999)),
    ("9999999", Ok(B10_MAX_7)),
    ("A000000", Ok(B10_MAX_7 + 1)),
    ("A000001", Ok(B10_MAX_7 + 2)),
    ("A05810C", Ok(10360716)),
    ("H1CXMQY", Ok(B16_PAD_7 - B10_MAX_7 - 2)),
    ("H1CXMQZ", Ok(B16_PAD_7 - B10_MAX_7 - 1)),
    ("H1CXMR0", Ok(B16_PAD_7 - B10_MAX_7)),
    ("H1CXMR1", Ok(B16_PAD_7 - B10_MAX_7 + 1)),
    ("H1CXMR2", Ok(B16_PAD_7 - B10_MAX_7 + 2)),
    ("H4PETBX", Ok(B16_MAX_7 - 2)),
    ("H4PETBY", Ok(B16_MAX_7 - 1)),
    ("H4PETBZ", Ok(B16_MAX_7)),
    ("H4PETC0", Ok(B16_MAX_7 + 1)),
    ("H4PETC1", Ok(B16_MAX_7 + 2)),
    ("MWPETBX", Ok(u32::MAX - 2)),
    ("MWPETBY", Ok(u32::MAX - 1)),
    ("MWPETBZ", Ok(u32::MAX)),
];

#[test]
fn test_sept_encode() {
    for (digits, value) in SEPT_PAIRS {
        if let Ok(value) = value {
            assert_eq!(*digits, sept(*value), "sept({value})");
        }
    }
}

#[test]
fn test_sept_b10ode() {
    for (digits, value) in SEPT_PAIRS {
        assert_eq!(*value, desept(*digits), "desept({digits})");
    }
}

static OCTO_PAIRS: &[(&str, Result<u64, DigitDecodingError>)] = &[
    ("", Err(DigitDecodingError::InvalidLength)),
    ("0", Err(DigitDecodingError::InvalidLength)),
    ("?", Err(DigitDecodingError::InvalidLength)),
    ("0000-000", Err(DigitDecodingError::InvalidDigit)),
    ("0000000", Err(DigitDecodingError::InvalidLength)),
    ("00000000", Ok(0______0)),
    ("000000000", Err(DigitDecodingError::InvalidLength)),
    ("0000000A", Err(DigitDecodingError::InvalidDigit)),
    ("00000001", Ok(0______1)),
    ("00000002", Ok(0______2)),
    ("00025137", Ok(0__25137)),
    ("00062074", Ok(0__62074)),
    ("00101010", Ok(0_101010)),
    ("10101010", Ok(10101010)),
    ("11111111", Ok(11111111)),
    ("22222222", Ok(22222222)),
    ("33333333", Ok(33333333)),
    ("44444444", Ok(44444444)),
    ("55555555", Ok(55555555)),
    ("66666666", Ok(66666666)),
    ("77777777", Ok(77777777)),
    ("88888888", Ok(88888888)),
    ("8GT18FFY", Ok(B16_PAD_8 - B10_MAX_8 - 2)),
    ("8GT18FFZ", Ok(B16_PAD_8 - B10_MAX_8 - 1)),
    ("8GT18FG0", Ok(B16_PAD_8 - B10_MAX_8)),
    ("8GT18FG1", Ok(B16_PAD_8 - B10_MAX_8 + 1)),
    ("8GT18FG2", Ok(B16_PAD_8 - B10_MAX_8 + 2)),
    ("8JD0M7QX", Ok((u32::MAX as u64) - 2)),
    ("8JD0M7QX", Ok(B16_MAX_8 - 2)),
    ("8JD0M7QY", Ok((u32::MAX as u64) - 1)),
    ("8JD0M7QY", Ok(B16_MAX_8 - 1)),
    ("8JD0M7QZ", Ok((u32::MAX as u64))),
    ("8JD0M7QZ", Ok(B16_MAX_8)),
    ("8JD0M7R0", Ok((u32::MAX as u64) + 1)),
    ("8JD0M7R0", Ok(B16_MAX_8 + 1)),
    ("8JD0M7R1", Ok((u32::MAX as u64) + 2)),
    ("8JD0M7R1", Ok(B16_MAX_8 + 2)),
    ("99999997", Ok(B10_MAX_8 - 2)),
    ("99999998", Ok(B10_MAX_8 - 1)),
    ("99999999", Ok(99999999)),
    ("99999999", Ok(B10_MAX_8)),
    ("A0000000", Ok(B10_MAX_8 + 1)),
    ("A0000001", Ok(B10_MAX_8 + 2)),
    ("GED0M7R0", Ok(1 << (38 + 1) - 1)),
    ("RED0M7M0", Ok((u32::MAX as u64) << 7)),
    ("Z53QAY91", Ok((u32::MAX as u64) * 256 * 22 / 31)),
    ("ZED0M7J8", Ok((u32::MAX as u64) * 256 * 23 / 32)),
    ("ZXAWBPNV", Ok(0xBBBBBBBBBB)),
    ("ZZD0M7R0", Ok(0xBC40000000)),
    ("ZZZZZZZZ", Ok(0xBC65F5E0FF)),
];
#[test]
fn test_octo_encode() {
    for (digits, value) in OCTO_PAIRS {
        if let Ok(value) = value {
            assert_eq!(*digits, octo(*value).unwrap(), "octo({value})");
        }
    }
}
