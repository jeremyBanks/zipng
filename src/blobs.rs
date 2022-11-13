pub mod blip;
pub mod blob;
pub mod blobbable;
pub mod serialization;

pub use self::blobbable::*;
pub use self::serialization::*;

/// [`Blob<T, Json>`][blob::Blob]
pub type JsonBlob<T: ?Sized> = blob::Blob<T, Json>;
/// [`Blob<T, Postcard>`][blob::Blob]
pub type PostcardBlob<T: ?Sized> = blob::Blob<T, Postcard>;
/// [`Blob<T, FlexBuffers>`][blob::Blob]
pub type FlexBuffersBlob<T: ?Sized> = blob::Blob<T, FlexBuffers>;
/// [`Blob<T, Cbor>`][blob::Blob]
pub type CborBlob<T: ?Sized> = blob::Blob<T, Cbor>;

/// [`Blip<T, Json>`][blip::Blip]
pub type JsonBlip<T: ?Sized> = blip::Blip<T, Json>;
/// [`Blip<T, Postcard>`][blip::Blip]
pub type PostcardBlip<T: ?Sized> = blip::Blip<T, Postcard>;
/// [`Blip<T, FlexBuffers>`][blip::Blip]
pub type FlexBuffersBlip<T: ?Sized> = blip::Blip<T, FlexBuffers>;
/// [`Blip<T, Cbor>`][blip::Blip]
pub type CborBlip<T: ?Sized> = blip::Blip<T, Cbor>;

use std::primitive;

/// [`Blip<T, Postcard>`][blip::Blip], using our default serialization format
pub type Blip<T: ?Sized> = PostcardBlip<T>;
/// [`Blob<T, Postcard>`][blob::Blob], using our default serialization format
pub type Blob<T: ?Sized> = PostcardBlob<T>;

/// [`Blip<bytes, Postcard>`][blip::Blip], representing untyped bytes using our
/// default serialization format
pub type ByteBlip = Blip<bytes>;
/// [`Blip<string, Postcard>`][blip::Blip], representing an otherwise-untyped
/// UTF-8 string using our default serialization format
pub type StringBlip = Blip<str>;

/// [`Blob<bytes, Postcard>`][blob::Blob], containing untyped bytes using our
/// default serialization format
pub type ByteBlob = Blob<bytes>;
/// [`Blob<string, Postcard>`][blob::Blob], containing an otherwise-untyped
/// UTF-8 string using our default serialization format
pub type StringBlob = Blob<string>;

/// [`[u8]`][slice], an externally-/un-sized byte string
#[allow(non_camel_case_types)]
pub type bytes = [u8];

/// [`str`][primitive::str], an externally-/un-sized UTF-8 text string
#[allow(non_camel_case_types)]
pub type string = primitive::str;
