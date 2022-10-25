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
macro_rules! record {
    ($($name:ident {
        $($field:ident: $ty:ty),* $(,)?
    })+) => {$(
        #[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
        pub struct $name {
            $(pub $field: $ty),*
        }

        impl $name {
            fn static_assert_record(record: $name) {
                fn static_assert_record(_: impl Record) {}
                static_assert_record(record);
            }
        }
    )+}
}

macro_rules! def {
    {
        $(
            $(# $attr:tt)*
            enum $enum_ident:ident $enum_block:tt
        )+
    } => {
        $(
            def! { @enum [$($attr)*] [$enum_ident] [$enum_block] }
        )+
    };

    { @enum [$($attrs:tt)+] [$ident:ident] [{
        $(struct $)*
    }] } => {
        $(# $attrs)*
        pub enum $ident $block
    };

    { @struct [$($attrs:tt)+] [$ident:ident] [$block:tt] } => {
        $(# $attrs)*
        pub struct $ident $block
    };
}

def! {
    #[derive(Debug, Clone, Serialize, Deserialize)]
    enum Request {
        struct HttpGet {
            url: String,
        }
        struct WavsToOpus {
            wavs: Vec<Vec<u8>>,
        }
    }
}
