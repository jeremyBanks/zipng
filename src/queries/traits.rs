use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::queries::Context;

pub trait Request: Debug + Serialize + DeserializeOwned + 'static {
    type Response: Response;

    fn query(&self, context: &mut Context) -> Self::Response;
}

pub trait Response: Debug + Serialize + DeserializeOwned + 'static {
    const NO_SAVE: u32 = 0x_______0; // no cache
    const BRIEFLY: u32 = 0x______10; // 16 seconds
    const FOR_NOW: u32 = 0x____1000; // ~2 hours
    const A_WHILE: u32 = 0x__300000; // ~1 month
    const FOREVER: u32 = 0x10000000; // ~8 years

    fn max_age_seconds(&self) -> u32 {
        Self::FOR_NOW
    }
}
