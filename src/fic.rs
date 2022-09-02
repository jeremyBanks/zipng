use std::fmt::Display;
use std::io::Write;
use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;

use crate::digits::octo;
use crate::sept;

#[allow(non_camel_case_types)]
type u160 = [u8; 20];

#[allow(non_camel_case_types)]
type u256 = [u8; 32];


#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Identifier {
    /// `BN` ISBN (without a check digit)
    ///
    /// ISBN-10-compatible values (those starting with `978`)
    /// are normalized to start with `000`, which allows most ISBN-10 values
    /// to be hex-encoded and most SBN (9-digit) values to be decimal-encoded,
    /// which seems neat. ISBN-13 values that start with anything else, such as
    /// `979`, are not modified.
    BookNumber(u64),
    /// `B` `Z` Amazon Standard Identification Number (ASIN), an 8-digit base 36
    /// value after we drop the leading `B` (ISBN-compatible values should
    /// use be ISBNs) and drop trailing check digit. If ASIN has a leading
    /// value other than `B` (not currently possible, but maybe in the
    /// future), then the prefix `ZZ` is used for the 9-digit base 36 value
    /// including the leading digit. Wait no ASIN's don't have a check
    /// digit. Shit. That's more than 9 base 32 digits even without the B
    /// (duh). It would fit in 9 base 64 digits, but I don't want that, do
    /// I? Well, if these are already 10 digits then I guess I could
    /// just concede the entire B prefix to them. Kind of shitty, though, since
    /// it would be better to use `BN` for ISBN/IAN/EAN values. But it's
    /// sensible. But on that note, do I want to concede the entire decimal
    /// prefixed range to ISBN-10s? Yes, I probably do. Wait no, since
    /// they're only 9 digits. Duh. Let's give them `A` for now.
    AmazonNumber(u128),
    /// `AO3`
    ArchiveOfOurOwn(u32),
    /// `ROY`
    RoyalRoad(u32),
    /// `FFN`
    FanFictionDotNet(u32),
    /// `WAT`
    Wattpad(u32),
}

impl Identifier {
    pub fn url(&self) -> String {
        match *self {
            Identifier::Uuid(uuid) => format!("{uuid:32X}"),
            Identifier::BookNumber(number) => format!("https://isbnsearch.org/isbn/{number}"),
            Identifier::ArchiveOfOurOwn(number) => {
                format!("https://archiveofourown.org/works/{number}")
            },
            Identifier::RoyalRoad(number) => format!("https://www.royalroad.com/fiction/{number}"),
            Identifier::FanFictionDotNet(number) => {
                format!("https://www.fanfiction.net/s/{number}")
            },
            Identifier::Wattpad(number) => format!("https://www.wattpad.com/story/{number}"),
        }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Identifier::BookNumber(number) => write!(f, "BN{}", octo(number).unwrap()),
            Identifier::ArchiveOfOurOwn(number) => write!(f, "AO3{}", sept(number)),
            Identifier::RoyalRoad(number) => write!(f, "ROY{}", sept(number)),
            Identifier::FanFictionDotNet(number) => write!(f, "FFN{}", sept(number)),
            Identifier::Wattpad(number) => write!(f, "WAT{}", sept(number)),
        }
    }
}

impl Identifier {
    pub const SAMPLES: &'static [Identifier] = &[
        Identifier::FanFictionDotNet(10360716),
        Identifier::ArchiveOfOurOwn(1118956),
        Identifier::Wattpad(1401653),
        Identifier::RoyalRoad(25137),
        Identifier::BookNumber(4823091),
        Identifier::BookNumber(76535038),
    ];
}

pub fn test() {
    for id in Identifier::SAMPLES {
        println!("{id}");
    }
}
