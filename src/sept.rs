use std::fmt::Display;
use std::io::Write;
use std::str::FromStr;

static DIGITS_C32: &'static [u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";
static DIGITS_HEX: &'static [u8; 16] = b"0123456789ABCDEF";
static DIGITS_DEC: &'static [u8; 10] = b"0123456789";

static DEC_MAX: u32 = 0___9_999_999;
static HEX_MAX: u32 = 0x__F_FFF_FFF;
static HEX_PAD: u32 = 0x__A_000_000 - 1;
static C32_PAD: u64 = 0x440_000_000 - 1;

/// Encodes a u32 as exactly 7 digits or uppercase letters.
///
/// If it fits in 7 decimal digits, then zero-padded decimal is used.
/// If it fits in 7 hexadecimal digits, then hexidecimal is used, but offset
/// such that the first digit is always a >= 'A' to avoid confusion with
/// decimal. Otherwise base 36 is used, but offset such that the first digit is
/// always >= 'G' to avoid confusion with hex and decimal.
pub fn sept(n: u32) -> String {
    let n = u64::from(n);

    let mut chars = vec![b'?'; 7];
    if n <= u64::from(DEC_MAX) {
        let mut r = n;
        for (index, digit) in chars.iter_mut().enumerate().rev() {
            let d = r % 10;
            r /= 10;
            *digit = DIGITS_DEC[d as usize];
        }
        debug_assert!(r == 0);
    } else if n <= (HEX_MAX - (HEX_PAD - DEC_MAX)).into() {
        let mut r: u64 = n - u64::from(DEC_MAX) + u64::from(HEX_PAD);
        for (index, digit) in chars.iter_mut().enumerate().rev() {
            let d = r % 16;
            r /= 16;
            *digit = DIGITS_HEX[d as usize];
        }
        debug_assert!(r == 0);
    } else {
        let mut r: u64 = n - u64::from(HEX_MAX - (HEX_PAD - DEC_MAX)) + C32_PAD;
        for (index, digit) in chars.iter_mut().enumerate().rev() {
            let d = r % 32;
            r /= 32;
            *digit = DIGITS_C32[d as usize];
        }
        debug_assert!(r == 0);
    }

    String::from_utf8(chars).unwrap()
}

/// The opposite of [`sept()`].
pub fn seven(s: &str) -> u32 {
    if s < "0000000" {
        panic!("out of bounds");
    } else if s > "MWPETBZ" {
        panic!("out of bounds");
    }

    todo!()
}

#[test]
fn test() {
    for (s, n) in [
        ("0000000", u32::MIN),
        ("0025137", 25137),
        ("0062074", 62074),
        ("1118956", 1118956),
        ("1401653", 1401653),
        ("9999997", DEC_MAX - 2),
        ("9999998", DEC_MAX - 1),
        ("9999999", DEC_MAX),
        ("A000000", DEC_MAX + 1),
        ("A000001", DEC_MAX + 2),
        ("A05810C", 10360716),
        ("H1PETBX", HEX_PAD - 2),
        ("H1PETBY", HEX_PAD - 1),
        ("H1PETBZ", HEX_PAD),
        ("H1PETC0", HEX_PAD + 1),
        ("H1PETC1", HEX_PAD + 2),
        ("H4PETBX", HEX_MAX - 2),
        ("H4PETBY", HEX_MAX - 1),
        ("H4PETBZ", HEX_MAX),
        ("H4PETC0", HEX_MAX + 1),
        ("H4PETC1", HEX_MAX + 2),
        ("MWPETBZ", u32::MAX),
    ] {
        assert_eq!(s, sept(n));
        // assert_eq!(n, seven(s));
    }
}
