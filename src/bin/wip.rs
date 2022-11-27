#![allow(non_camel_case_types)]
use std::borrow::BorrowMut;
use std::mem;
use std::ops::Deref;

// use zipng::default;
use once_cell::sync::OnceCell;
use tracing::error;
use tracing::info;

pub fn main() {

}

#[derive(Debug, Clone)]
pub enum Operation<Invocation: self::Invocation> {
    Invocation(Invocation),
    Completion(Invocation::Completion),
}

impl<Invocation: self::Invocation> Operation<Invocation> {
    fn with<F: FnOnce(&mut Invocation)>(mut self, f: F) -> Self {
        if let Operation::Invocation(ref mut invocation) = self {
            f(invocation);
        } else {
            error!("Ignoring attempted with() on an Operation::Completion.");
        }
        self
    }
    
    fn complete(&mut self) -> &Invocation::Completion {
        match self {
            Operation::Invocation(invocation) => {
                let mut result = Operation::<Invocation>::Completion(invocation.execute());

                let completion = invocation.execute();
                mem::swap(self, &mut result);
                
                &completion
            }
            Operation::Completion(completion) => {
                info!("Ignoring attempted complete() on an Operation::Completion.");
                &completion
            }
        }
    }
}

pub trait Invocation: Sized {
    type Completion: Sized ;
    fn execute(self) -> Self::Completion;
}


// #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
// pub struct Deferred<Invocation> where Invocation: self::Invocation {
//     invocation: Invocation,
// }

// impl<Invocation> IntoFuture for Deferred<Invocation> where Invocation: self::Invocation {
//     type Output = Invocation::Completion;
//     type IntoFuture = impl Future<Output = Invocation::Completion>;
//     fn into_future(self) -> Self::IntoFuture {
//         async {
//             self.invocation.call().await
//         }
//     }
// }

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
