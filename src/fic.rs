use std::fmt::Display;
use std::io::Write;
use std::str::FromStr;

use crate::sept;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Identifier {
    ArchiveOfOurOwn(u32),
    RoyalRoad(u32),
    FanFictionDotNet(u32),
    Wattpad(u32),
    FimFiction(u32),
}

impl Identifier {
    pub fn url(&self) -> String {
        match self {
            Identifier::ArchiveOfOurOwn(id) => format!("https://archiveofourown.org/works/{}", id),
            Identifier::RoyalRoad(id) => format!("https://www.royalroad.com/fiction/{}", id),
            Identifier::FanFictionDotNet(id) => format!("https://www.fanfiction.net/s/{}", id),
            Identifier::Wattpad(id) => format!("https://www.wattpad.com/story/{}", id),
            Identifier::FimFiction(id) => format!("https://www.fimfiction.net/story/{}", id),
        }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Identifier::ArchiveOfOurOwn(id) => write!(f, "AO3{}", sept(*id)),
            Identifier::RoyalRoad(id) => write!(f, "ROY{}", sept(*id)),
            Identifier::FanFictionDotNet(id) => write!(f, "FFN{}", sept(*id)),
            Identifier::Wattpad(id) => write!(f, "WAT{}", sept(*id)),
            Identifier::FimFiction(id) => write!(f, "FIM{}", sept(*id)),
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
    ];
}

pub fn test() {
    for id in Identifier::SAMPLES {
        println!("{id}");
    }
}
