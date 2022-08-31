use std::fmt::Display;
use std::io::Write;
use std::str::FromStr;

use crate::digits::octo;
use crate::sept;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Identifier {
    /// ISBN (without a check digit)
    ///
    /// ISBN-10-compatible values (those starting with `978`)
    /// are normalized to start with `000`, which allows most ISBN-10 values
    /// to be hex-encoded and most SBN (9-digit) values to be decimal-encoded,
    /// which seems neat. ISBN-13 values that start with anything else, such as
    /// `979`, are not modified.
    BookNumber(u64),
    ArchiveOfOurOwn(u32),
    RoyalRoad(u32),
    FanFictionDotNet(u32),
    Wattpad(u32),
    FimFiction(u32),
}

impl Identifier {
    pub fn url(&self) -> String {
        match *self {
            Identifier::BookNumber(number) => format!("https://isbnsearch.org/isbn/{number}"),
            Identifier::ArchiveOfOurOwn(number) =>
                format!("https://archiveofourown.org/works/{number}"),
            Identifier::RoyalRoad(number) => format!("https://www.royalroad.com/fiction/{number}"),
            Identifier::FanFictionDotNet(number) =>
                format!("https://www.fanfiction.net/s/{number}"),
            Identifier::Wattpad(number) => format!("https://www.wattpad.com/story/{number}"),
            Identifier::FimFiction(number) => format!("https://www.fimfiction.net/story/{number}"),
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
            Identifier::FimFiction(number) => write!(f, "FIM{}", sept(number)),
        }
    }
}

impl Identifier {
    pub const SAMPLES: &'static [Identifier] = &[
        Identifier::FanFictionDotNet(10360716),
        Identifier::ArchiveOfOurOwn(1118956),
        Identifier::Wattpad(1401653),
        Identifier::RoyalRoad(25137),
        Identifier::FimFiction(62074),
        Identifier::BookNumber(4823091),
        Identifier::BookNumber(76535038),
    ];
}

pub fn test() {
    for id in Identifier::SAMPLES {
        println!("{id}");
    }
}
