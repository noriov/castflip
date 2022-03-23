use core::mem::MaybeUninit;
use std::io::{Read, Result};

use crate::{Cast, Endian, Flip};
use crate::experimental::AsBytes;
#[cfg(doc)] use crate::BE;


///
/// Defines methods to `encast` and `Flip` through I/O.
///
/// Note: In this crate, the term `encast` means decoding a number of
/// bytes to one or more values, the term `decast` means encoding one
/// or more variables to a number of bytes, and the term `endian-flip`
/// means flipping the endianness of value(s).
///
/// # Example
///
/// In the example below, method `encastf` decodes bytes from `input1`
/// in Big-Endian ([`BE`]) to variable `udp_hdr2` of type `UdpHdr`.
///
/// ```
/// # use std::io::Result;
/// # fn main() { test(); }
/// # fn test() -> Result<()> {
/// use std::io::Cursor;
/// use castflip::{Cast, Flip, EncastIO, BE};
///
/// #[repr(C)]
/// #[derive(Cast, Flip)]
/// struct UdpHdr {     // UDP: https://www.rfc-editor.org/rfc/rfc768.txt
///     sport:  u16,    // UDP Source Port
///     dport:  u16,    // UDP Destination Port
///     len:    u16,    // UDP Length
///     sum:    u16,    // UDP Checksum
/// }
///
/// // Input data: UDP header (8 bytes) + part of DNS header (8 bytes)
/// let bytes1: [u8; 16] = [0xC3, 0xC9, 0x00, 0x35, 0x00, 0x32, 0x82, 0x3F,
///                         0x1A, 0xD1, 0x01, 0x20, 0x00, 0x01, 0x00, 0x00];
/// let mut input1 = Cursor::new(bytes1);
///
/// // Decode input `input1` to variable `udp_hdr2`.
/// // Because the UDP header is 8 bytes as defined above, only
/// // the first 8 bytes are decoded, remaining 8 bytes are ignored.
/// let udp_hdr2: UdpHdr = input1.encastf(BE)?;  // BE = Big-Endian
///
/// // Check the results.
/// assert_eq!(udp_hdr2.sport, 0xC3C9); // = 50121
/// assert_eq!(udp_hdr2.dport, 0x0035); // = 53 (DNS)
/// assert_eq!(udp_hdr2.len,   0x0032); // = 50
/// assert_eq!(udp_hdr2.sum,   0x823F);
/// # Ok(())
/// # }
/// ```
///
/// # Description
///
/// All methods in trait `EncastIO` `encast` a number of bytes read
/// from I/O to one or more values of the specified type.  The type of
/// the value(s) can be explicitly specified as the generic type
/// parameter of the methods or implicitly specified so that the Rust
/// compiler can infer it.  The endianness of resulting value(s) is
/// flipped when required and necessary.  Currently, only an
/// implementation for trait `std::io::Read` is provided.
///
/// The input `self` should have enough bytes to decode to the
/// specified number of value(s) of the specified type `T`.  If there
/// are enough bytes, the required number of bytes are read from I/O
/// and decoded to the specified type of value(s), which is/are
/// returned in `Ok`().  The remaining bytes in input `self` are
/// untouched.  If I/O error is detected, `Err`(`std::io::Error`) is
/// returned.
///
/// When argument `endian` is specified, the endianness of value(s) is
/// flipped if necessary.
///
pub trait EncastIO {
    /// Decodes the bytes read from input `self` to a value of type `T`.
    /// The endianness of the resulting value is not flipped.
    fn encast<T>(&mut self) -> Result<T>
    where
	T: Cast;

    /// Decodes the bytes read from input `self` to a value of type `T`.
    /// The endianness of the resulting value is flipped if necessary.
    /// The endianness of the bytes is specified in `endian`.
    fn encastf<T>(&mut self, endian: Endian) -> Result<T>
    where
	T: Cast + Flip;

    /// Decodes the bytes read from input `self` to a vector of
    /// value(s) of type `T`.  The endianness of the resulting
    /// value(s) is not flipped.  The number of elements in the
    /// resulting vecotr is specified in `nelem`.
    fn encastv<T>(&mut self, nelem: usize) -> Result<Vec<T>>
    where
	T: Cast;

    /// Decodes the bytes read from input `self` to a vector of
    /// value(s) of type `T`.  The endianness of the resulting
    /// value(s) is flipped if necessary.  The endianness of the bytes
    /// is specified in `endian`.  The number of elements in the
    /// resulting vecotr is specified in `nelem`.
    fn encastvf<T>(&mut self, nelem: usize, endian: Endian) -> Result<Vec<T>>
    where
	T: Cast + Flip;
}


impl<R> EncastIO for R
where
    R: ?Sized + Read
{
    fn encast<T>(&mut self) -> Result<T>
    where
	T: Cast
    {
	unsafe {
	    let mut val = MaybeUninit::<T>::uninit().assume_init();
	    self.read_exact(val.as_bytes_mut())?;
	    Ok(val)
	}
    }

    fn encastf<T>(&mut self, endian: Endian) -> Result<T>
    where
	T: Cast + Flip
    {
	Ok(self.encast::<T>()?.flip_val(endian))
    }

    fn encastv<T>(&mut self, nelem: usize) -> Result<Vec<T>>
    where
	T: Cast
    {
	let mut vec = Vec::<T>::with_capacity(nelem);
	for _i in 0 .. nelem {
	    vec.push(self.encast::<T>()?);
	}
	Ok(vec)
    }

    fn encastvf<T>(&mut self, nelem: usize, endian: Endian) -> Result<Vec<T>>
    where
	T: Cast + Flip
    {
	let mut vec = Vec::<T>::with_capacity(nelem);
	for _i in 0 .. nelem {
	    vec.push(self.encastf::<T>(endian)?);
	}
	Ok(vec)
    }
}
