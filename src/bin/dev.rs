#![cfg_attr(debug_assertions, allow(unused))]
use serde::Serialize;
use serde::Deserialize;
use std::fmt::Debug;

pub fn main() {
    println!("hello");
}

macro_rules! alias {
    ($name:ident = $($bounds:tt)+) => {
        pub trait $name: $($bounds)+ {}
        impl<T> $name for T where T: $($bounds)+ {}
    };
}
macro_rules! union {
    ($(#![derive($($ty:ty),+)])? $name:ident = $(|)? $($variant:ident)|* $(|)? $(;)?) => {
        $(#[derive($($ty),+)])?
        pub enum $name {
            $($variant($variant)),*
        }
    };
}

alias! { Record = Serialize + Deserialize<'static> + Clone + Send + Debug + 'static }
union! {
    #![derive(Clone, Debug, Serialize, Deserialize)]
    Request =
        | HttpGet
        | WavsToOpus
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpGet {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WavsToOpus {
    pub wavs: Vec<Vec<u8>>,
}

pub struct Query<Record: self::Record> {
    pub record: Record
}

