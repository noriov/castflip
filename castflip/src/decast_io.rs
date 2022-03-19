use core::mem;
use std::io::{Write, Result};

use crate::{Cast, Endian, Flip};
use crate::experimental::AsBytes;


///
/// Defines methods to `decast` and `Flip` through I/O.
///
/// Note: In this crate, the term `encast` means decoding a number of
/// bytes to one or more values, the term `decast` means encoding one
/// or more variables to a number of bytes, and the term `endian-flip`
/// means flipping the endianness of value(s).
///
/// # Example
///
/// In the example below, method `decastf` encodes the value in
/// `udp_hdr1` of type `UdpHdr` to bytes in Big-Endian (`BE`) and
/// stores them in `bytes2`.
///
/// ```
/// # use std::io::Result;
/// # fn main() { test(); }
/// # fn test() -> Result<()> {
/// use std::io::Cursor;
/// use castflip::{Cast, Flip, DecastIO, BE};
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
/// // Input data: UDP header (8 bytes)
/// let udp_hdr1 = UdpHdr { sport: 50121, dport: 53, len: 50, sum: 0x823F };
/// let bytes1: [u8; 8] = [0xC3, 0xC9, 0x00, 0x35, 0x00, 0x32, 0x82, 0x3F];
///
/// // Encode UDP header `udp_hdr2` to bytes in `output2`.
/// // Because the UDP header is 8 bytes as defined above,
/// // only the first 8 bytes of `output2` are filled with data.
/// let mut output2 = Cursor::new(vec![0_u8; 16]);
/// output2.decastf(&udp_hdr1, BE)?;
/// let bytes2 = output2.into_inner();
///
/// // Check the results.
/// assert_eq!(&bytes2[0..8], &bytes1[0..8]);
/// assert_eq!(&bytes2[8..16], &[0_u8; 8]);
/// # Ok(())
/// # }
/// ```
///
/// # Description
///
/// All methods in this trait `decast` one or more variables to a
/// number of bytes and writes to I/O.  Because the generic type can
/// be inferred from the argument, generic type parameter can be
/// omitted.  The endianness of resulting value(s) is flipped when
/// required and necessary.  Currently, only an implementation for
/// trait `Write` is provided.
///
/// The output `self` should have enough room to encode to the
/// specified number of value(s) of type `T`.  If there is enough
/// room, the specified variable(s) is/are encoded to bytes and
/// written to output `self`.  If successful, the size of written
/// bytes are returned in `Ok`().  If I/O error is detected,
/// `Err`(std::io::Error) is returned.
///
/// When `endian` is specified, the endianness of resulting bytes is
/// flipped if necessary.
///
pub trait DecastIO {
    /// Encodes the value pointed by `val_ptr` of type `T` to bytes
    /// and writes them to output `self`.  The endianness of the
    /// resulting bytes is not flipped.
    fn decast<T>(&mut self, val_ptr: &T) -> Result<usize>
    where
	T: Cast;

    /// Encodes the value pointed by `val_ptr` of type `T` to bytes
    /// and writes them to output `self`.  The endianness of resulting
    /// bytes is flipped if necessary as specified by `endian`.
    fn decastf<T>(&mut self, val_ptr: &T, endian: Endian) -> Result<usize>
    where
	T: Cast + Flip;

    /// Encodes the slice of value(s) of type `T` pointed by `slice`
    /// to bytes and writes them to output `self`.  The endianness of
    /// the resulting bytes is not flipped.
    fn decastv<T>(&mut self, slice: &[T]) -> Result<usize>
    where
	T: Cast;

    /// Encodes the slice of value(s) of type `T` pointed by `slice`
    /// to bytes and writes them to output `self`.  The endianness of
    /// resulting bytes is flipped if necessary as specified by
    /// `endian`.
    fn decastvf<T>(&mut self, slice: &[T], endian: Endian) -> Result<usize>
    where
	T: Cast + Flip;
}


impl<W> DecastIO for W
where
    W: ?Sized + Write
{
    fn decast<T>(&mut self, val_ptr: &T) -> Result<usize>
    where
	T: Cast
    {
	unsafe {
	    self.write_all(val_ptr.as_bytes_ref())?;
	}
	Ok(mem::size_of_val(val_ptr))
    }

    fn decastf<T>(&mut self, val_ptr: &T, endian: Endian) -> Result<usize>
    where
	T: Cast + Flip
    {
	if !endian.need_swap() {
	    self.decast::<T>(val_ptr)
	} else {
	    self.decast::<T>(&val_ptr.flip_val_swapped())
	}
    }

    fn decastv<T>(&mut self, slice: &[T]) -> Result<usize>
    where
	T: Cast
    {
	unsafe {
	    self.write_all(slice.as_bytes_ref())?;
	}
	Ok(slice.len())
    }

    fn decastvf<T>(&mut self, slice: &[T], endian: Endian) -> Result<usize>
    where
	T: Cast + Flip
    {
	if !endian.need_swap() {
	    self.decastv::<T>(slice)
	} else {
	    for elem in slice {
		self.decast::<T>(&elem.flip_val_swapped())?;
	    }
	    Ok(slice.len())
	}
    }
}
