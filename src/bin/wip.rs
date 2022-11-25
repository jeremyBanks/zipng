#![feature(async_traits)]
#![allow(non_camel_case_types)]

use once_cell::sync::OnceCell;

pub struct Deferred<Return: Sized> {
    return: OnceCell<Return>
}

pub struct write_framed_as_deflate {
    pub(crate) output: &'a mut impl WriteAndSeek,
    pub(crate) data: &'a [u8],
}

pub fn write_framed_as_deflate(output: &mut impl WriteAndSeek, data: &[u8]) {

}

pub fn main() {

}
