use std::fmt::Debug;

use super::Error;
use crate::Blip;
use crate::Blob;
use crate::Request;
use crate::Response;

impl<T> Request for Blip<T>
where T: ?Sized
{
    const TAG: u32 = 0x00;
    type Response = Blob<T>;
    type Error = Error;
}

impl<T> Response for Blob<T>
where T: ?Sized
{
    type Request = Blip<T>;
}
