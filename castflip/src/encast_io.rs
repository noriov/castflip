use core::mem::MaybeUninit;
use core::mem;
use std::io::{Read, Result};
#[cfg(doc)] use std::io;

use crate::{Cast, Endian, Flip};
use crate::experimental::{AsifBytes, PushBulk};
#[cfg(doc)] use crate::BE;


///
/// Defines methods to `encast` and `endian-flip` through [`io::Read`].
///
/// Note: In this crate, the term `encast` means decoding a number of
/// bytes to one or more values, the term `decast` means encoding one
/// or more variables to a number of bytes, and the term `endian-flip`
/// means flipping the endianness of value(s).
///
/// # Example 1
///
/// In the example below, method `encastf` decodes bytes from `input1`
/// in Big-Endian ([`BE`]) to variable `udp_hdr2` of type `UdpHdr`.
/// Note that `io::Cursor` wraps an in-memory buffer and provides it
/// through `io::Read`.
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
/// // Check the results (udp_hdr2)
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
/// compiler can infer it.  The methods whose name contain 's' (=
/// slice) or 'v' (= vector) `encast` a series of structured binary
/// data.  The methods whose names end with 'f' flip the endianness of
/// the results.  Currently, an implementation for trait [`io::Read`]
/// is provided.
///
/// The input `self` should have enough bytes to decode to the
/// specified number of value(s) of the specified type `T`.  If there
/// are enough bytes, the required number of bytes are read from I/O
/// and decoded to the specified type of value(s).  The remaining
/// bytes in input `self` are untouched.  If successful, return value
/// is enclosed in `Ok`().  If failed, `Err`([`io::Error`]) is
/// returned.  The type of the return value is [`io::Result`].
///
/// When argument `endian` is specified, the endianness of value(s) is
/// flipped if necessary.
///
/// # Example 2
///
/// Because `io::Read` is implemented for `&[u8]`, `EncastIO` can
/// `encast` from memory.  The example below is almost the same with
/// Example 1 except it uses a mutable slice instead of `io::Cursor`.
///
/// ```
/// # use std::io::Result;
/// # fn main() { test(); }
/// # fn test() -> Result<()> {
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
/// let mut slice1 = &bytes1[..];
///
/// // Decode slice `slice1` to variable `udp_hdr2`.
/// // Because the UDP header is 8 bytes as defined above, only
/// // the first 8 bytes are decoded, remaining 8 bytes are ignored.
/// let udp_hdr2: UdpHdr = slice1.encastf(BE)?;  // BE = Big-Endian
///
/// // Check the results (udp_hdr2)
/// assert_eq!(udp_hdr2.sport, 0xC3C9); // = 50121
/// assert_eq!(udp_hdr2.dport, 0x0035); // = 53 (DNS)
/// assert_eq!(udp_hdr2.len,   0x0032); // = 50
/// assert_eq!(udp_hdr2.sum,   0x823F);
///
/// // Check the result (slice1)
/// // Note: `slice1` contains unread part.
/// assert_eq!(slice1.len(), 8);
/// assert_eq!(slice1, &bytes1[8..16]);
/// # Ok(())
/// # }
/// ```
///
pub trait EncastIO {
    /// Decodes the bytes read from input `self` to a value of type
    /// `T` and returns the value.  The endianness of the resulting
    /// value is not flipped.
    fn encast<T>(&mut self) -> Result<T>
    where
	T: Cast;

    /// Decodes the bytes read from input `self` to a value of type
    /// `T` and returns the value.  The endianness of the resulting
    /// value is flipped if necessary.  The endianness of the bytes is
    /// specified in `endian`.
    fn encastf<T>(&mut self, endian: Endian) -> Result<T>
    where
	T: Cast + Flip;

    /// Decodes the bytes read from input `self` to value(s) of type
    /// `T` and fill `slice` with the value(s).  It returns the number
    /// of decoded bytes.  The endianness of the resulting value(s) is
    /// not flipped.
    fn encasts<T>(&mut self, slice: &mut [T]) -> Result<usize>
    where
	T: Cast;

    /// Decodes the bytes read from input `self` to value(s) of type
    /// `T` and fill `slice` with the value(s).  It returns the number
    /// of decoded bytes.  The endianness of the resulting value(s) is
    /// flipped if necessary.  The endianness of the bytes is
    /// specified in `endian`.
    fn encastsf<T>(&mut self, slice: &mut [T], endian: Endian) -> Result<usize>
    where
	T: Cast + Flip;

    /// Decodes the bytes read from input `self` to a vector of
    /// value(s) of type `T` and returns the vector.  The endianness
    /// of the resulting value(s) is not flipped.  The number of
    /// elements in the resulting vecotr is specified in `nelem`.
    fn encastv<T>(&mut self, nelem: usize) -> Result<Vec<T>>
    where
	T: Cast;

    /// Decodes the bytes read from input `self` to a vector of
    /// value(s) of type `T` and returns the vector.  The endianness
    /// of the resulting value(s) is flipped if necessary.  The
    /// endianness of the bytes is specified in `endian`.  The number
    /// of elements in the resulting vecotr is specified in `nelem`.
    fn encastvf<T>(&mut self, nelem: usize, endian: Endian) -> Result<Vec<T>>
    where
	T: Cast + Flip;
}


impl<R> EncastIO for R
where
    R: ?Sized + Read
{
    #[inline]
    fn encast<T>(&mut self) -> Result<T>
    where
	T: Cast
    {
	// Decode a value of type T from from `self`.
	unsafe {
	    let mut val = MaybeUninit::<T>::uninit();
	    self.read_exact(val.asif_bytes_mut())?;
	    Ok(val.assume_init())
	}
    }

    #[inline]
    fn encastf<T>(&mut self, endian: Endian) -> Result<T>
    where
	T: Cast + Flip
    {
	let mut val = self.encast::<T>()?;
	val.flip_var(endian);
	Ok(val)
    }

    #[inline]
    fn encasts<T>(&mut self, slice: &mut [T]) -> Result<usize>
    where
	T: Cast
    {
	unsafe {
	    self.read_exact(slice.asif_bytes_mut())?;
	}
	Ok(mem::size_of::<T>() * slice.len())
    }

    #[inline]
    fn encastsf<T>(&mut self, slice: &mut [T], endian: Endian) -> Result<usize>
    where
	T: Cast + Flip
    {
	let size = self.encasts::<T>(slice)?;
	for elem in slice {
	    elem.flip_var(endian);
	}
	Ok(size)
    }

    #[inline]
    fn encastv<T>(&mut self, nelem: usize) -> Result<Vec<T>>
    where
	T: Cast
    {
	let mut vec: Vec<T> = Vec::new();

	unsafe {
	    vec.push_bulk(nelem, | new_slice | { self.encasts(new_slice) })?;
	}

	Ok(vec)
    }

    #[inline]
    fn encastvf<T>(&mut self, nelem: usize, endian: Endian) -> Result<Vec<T>>
    where
	T: Cast + Flip
    {
	let mut vec = self.encastv::<T>(nelem)?;
	for elem in &mut vec {
	    elem.flip_var(endian);
	}
	Ok(vec)
    }
}
