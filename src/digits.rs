use std::fmt::Display;
use std::io::Write;
use std::str::FromStr;

use miette::Diagnostic;
use thiserror::Error;

static DIGITS_C32: &'static [u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";
static DIGITS_HEX: &'static [u8; 16] = b"0123456789ABCDEF";
static DIGITS_DEC: &'static [u8; 10] = b"0123456789";

static DEC_MAX_7: u32 = 0___9_999_999;
static HEX_MAX_7: u32 = 0x__F_FFF_FFF;
static HEX_PAD_7: u32 = 0x__9_FFF_FFF;
static C32_PAD_7: u64 = 0x43F_FFF_FFF;

static DEC_MAX_8: u64 = 0____99_999_999;
static HEX_MAX_8: u64 = 0x___FF_FFF_FFF;
static HEX_PAD_8: u64 = 0x___9F_FFF_FFF;
static C32_PAD_8: u64 = 0x4_3FF_FFF_FFF;

/// Encodes a u32 as exactly 7 digits or uppercase letters.
///
/// If it fits in 7 decimal digits, then zero-padded decimal is used.
/// If it fits in 7 hexadecimal digits, then hexadecimal is used, but offset
/// such that the first digit is always a >= 'A' to avoid confusion with
/// decimal. Otherwise base 36 is used, but offset such that the first digit is
/// always >= 'G' to avoid confusion with hex and decimal.
pub fn sept(n: u32) -> String {
    let n = u64::from(n);

    let mut chars = vec![b'?'; 7];
    if n <= u64::from(DEC_MAX_7) {
        let mut r = n;
        for (index, digit) in chars.iter_mut().enumerate().rev() {
            let d = r % 10;
            r /= 10;
            *digit = DIGITS_DEC[d as usize];
        }
        debug_assert!(r == 0);
    } else if n <= (HEX_MAX_7 - (HEX_PAD_7 - DEC_MAX_7)).into() {
        let mut r: u64 = n - u64::from(DEC_MAX_7) + u64::from(HEX_PAD_7);
        for (index, digit) in chars.iter_mut().enumerate().rev() {
            let d = r % 16;
            r /= 16;
            *digit = DIGITS_HEX[d as usize];
        }
        debug_assert!(r == 0);
    } else {
        let mut r: u64 = n - u64::from(HEX_MAX_7 - (HEX_PAD_7 - DEC_MAX_7)) + C32_PAD_7;
        for (index, digit) in chars.iter_mut().enumerate().rev() {
            let d = r % 32;
            r /= 32;
            *digit = DIGITS_C32[d as usize];
        }
        debug_assert!(r == 0);
    }

    String::from_utf8(chars).unwrap()
}

/// Encodes a u32 as exactly 8 digits or uppercase letters.
///
/// If it fits in 8 decimal digits, then zero-padded decimal is used.
/// If it fits in 8 hexadecimal digits, then hexadecimal is used, but offset
/// such that the first digit is always a >= 'A' to avoid confusion with
/// decimal. Otherwise base 36 is used, but offset such that the first digit is
/// always >= 'G' to avoid confusion with hex and decimal.
pub fn octo(n: u64) -> Result<String, DigitEncodingError> {
    let mut chars = vec![b'?'; 8];
    if n <= u64::from(DEC_MAX_8) {
        let mut r = n;
        for (index, digit) in chars.iter_mut().enumerate().rev() {
            let d = r % 10;
            r /= 10;
            *digit = DIGITS_DEC[d as usize];
        }
        if (r > 0) {
            return Err(DigitEncodingError::OutOfBounds);
        }
    } else if n <= (HEX_MAX_8 - (HEX_PAD_8 - DEC_MAX_8)).into() {
        let mut r: u64 = n - u64::from(DEC_MAX_8) + u64::from(HEX_PAD_8);
        for (index, digit) in chars.iter_mut().enumerate().rev() {
            let d = r % 16;
            r /= 16;
            *digit = DIGITS_HEX[d as usize];
        }
        if (r > 0) {
            return Err(DigitEncodingError::OutOfBounds);
        }
    } else {
        let mut r: u64 = n - u64::from(HEX_MAX_8 - (HEX_PAD_8 - DEC_MAX_8)) + C32_PAD_8;
        for (index, digit) in chars.iter_mut().enumerate().rev() {
            let d = r % 32;
            r /= 32;
            *digit = DIGITS_C32[d as usize];
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
    ("9999997", Ok(DEC_MAX_7 - 2)),
    ("9999998", Ok(DEC_MAX_7 - 1)),
    ("9999999", Ok(9999999)),
    ("9999999", Ok(DEC_MAX_7)),
    ("A000000", Ok(DEC_MAX_7 + 1)),
    ("A000001", Ok(DEC_MAX_7 + 2)),
    ("A05810C", Ok(10360716)),
    ("H1CXMQY", Ok(HEX_PAD_7 - DEC_MAX_7 - 2)),
    ("H1CXMQZ", Ok(HEX_PAD_7 - DEC_MAX_7 - 1)),
    ("H1CXMR0", Ok(HEX_PAD_7 - DEC_MAX_7)),
    ("H1CXMR1", Ok(HEX_PAD_7 - DEC_MAX_7 + 1)),
    ("H1CXMR2", Ok(HEX_PAD_7 - DEC_MAX_7 + 2)),
    ("H4PETBX", Ok(HEX_MAX_7 - 2)),
    ("H4PETBY", Ok(HEX_MAX_7 - 1)),
    ("H4PETBZ", Ok(HEX_MAX_7)),
    ("H4PETC0", Ok(HEX_MAX_7 + 1)),
    ("H4PETC1", Ok(HEX_MAX_7 + 2)),
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
fn test_sept_decode() {
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
    ("8GT18FFY", Ok(HEX_PAD_8 - DEC_MAX_8 - 2)),
    ("8GT18FFZ", Ok(HEX_PAD_8 - DEC_MAX_8 - 1)),
    ("8GT18FG0", Ok(HEX_PAD_8 - DEC_MAX_8)),
    ("8GT18FG1", Ok(HEX_PAD_8 - DEC_MAX_8 + 1)),
    ("8GT18FG2", Ok(HEX_PAD_8 - DEC_MAX_8 + 2)),
    ("8JD0M7QX", Ok((u32::MAX as u64) - 2)),
    ("8JD0M7QX", Ok(HEX_MAX_8 - 2)),
    ("8JD0M7QY", Ok((u32::MAX as u64) - 1)),
    ("8JD0M7QY", Ok(HEX_MAX_8 - 1)),
    ("8JD0M7QZ", Ok((u32::MAX as u64))),
    ("8JD0M7QZ", Ok(HEX_MAX_8)),
    ("8JD0M7R0", Ok((u32::MAX as u64) + 1)),
    ("8JD0M7R0", Ok(HEX_MAX_8 + 1)),
    ("8JD0M7R1", Ok((u32::MAX as u64) + 2)),
    ("8JD0M7R1", Ok(HEX_MAX_8 + 2)),
    ("99999997", Ok(DEC_MAX_8 - 2)),
    ("99999998", Ok(DEC_MAX_8 - 1)),
    ("99999999", Ok(99999999)),
    ("99999999", Ok(DEC_MAX_8)),
    ("A0000000", Ok(DEC_MAX_8 + 1)),
    ("A0000001", Ok(DEC_MAX_8 + 2)),
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
