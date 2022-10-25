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

macro_rules! fun {
    (out $($output:tt)*) => { $($output)* };

    ({$($tail:tt)*} $($head:tt)*) => {
        fun! { $($head)* }
        fun! { $($tail)* }
    };
}

// macro_rules! def {
//     {
//         $(# $attr:tt)*
//         enum $ident:ident $block:tt
//     } => {
//         def! { @enum [$($attr)*] [$ident] [$block] [] }
//     };

//     {
//         $(# $attr:tt)*
//         struct $ident:ident $block:tt
//     } => {
//         def! { @struct [$($attr)*] [$ident] [$block] [] }
//     };

//     { @enum [$($attrs:tt)+] [$ident:ident] [{
//         $(
//             $(# $attr:tt)*
//             $(struct $struct_ident:ident $struct_block:block $)*
//             $(enum $enum_ident:ident $enum_block:block $)*
//         )*
//     }] [$($out:tt)*] } => {
//         $(# $attrs)*
//         pub enum $ident {
//             $($struct $block)*
//         }

//         $(def! { @struct [$($attrs)*] [$struct] [$block] })*
//     };

//     { @struct [$($attrs:tt)+] [$ident:ident] [$block:tt] [$($out:tt)*] } => {
//         $(# $attrs)*
//         pub struct $ident $block
//     };
// }

// def! {
//     #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash)]
//     enum Request {
//         #[derive(PartialOrd)]
//         struct HttpGet {
//             url: String,
//         }

//         struct WavsToOpus {
//             wavs: Vec<Vec<u8>>,
//         }
//     }
// }
