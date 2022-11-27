#![feature(async_fn_in_trait)]
#![feature(type_alias_impl_trait)]
#![allow(non_camel_case_types)]

use std::future::Future;
use std::future::IntoFuture;

#[tokio::main]
pub async fn main() {
    
}

pub trait Invocation {
    type Completion;
    async fn call(self) -> Self::Completion;
}
 

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Deferred<Invocation> where Invocation: self::Invocation {
    invocation: Invocation,
}

impl<Invocation> IntoFuture for Deferred<Invocation> where Invocation: self::Invocation {
    type Output = Invocation::Completion;
    type IntoFuture = impl Future<Output = Invocation::Completion>;
    fn into_future(self) -> Self::IntoFuture {
        async {
            self.invocation.call().await
        }
    }
}

// pub fn zip() -> Deferred<MakeZip> {
//     Deferred::Invocation(MakeZip::default())
// }

// pub struct MakeZip {

// }
// impl Invocation for MakeZip {
//     type Invocation = Self;
//     type Completion = Result<(), panic>;
//     fn call(invocation: Invocation) -> Self::Completion {
//         unimplemented!()
//     }
// }


// pub struct write_framed_as_deflate {
//     pub(crate) output: &'a mut impl WriteAndSeek,
//     pub(crate) data: &'a [u8],
// }

// pub fn write_framed_as_deflate(output: &mut impl WriteAndSeek, data: &[u8]) {

// }

// pub fn main() {

// }
