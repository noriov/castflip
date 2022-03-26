use core::mem;
use std::io::{Write, Result};

use crate::{Cast, Endian, Flip};
use crate::experimental::AsifBytes;
#[cfg(doc)] use crate::BE;


///
/// Defines methods to `decast` and `Flip` through `io::Write`
///
/// Note: In this crate, the term `encast` means decoding a number of
/// bytes to one or more values, the term `decast` means encoding one
/// or more variables to a number of bytes, and the term `endian-flip`
/// means flipping the endianness of value(s).
///
/// # Example 1
///
/// In the example below, method `decastf` encodes the value in
/// `udp_hdr1` of type `UdpHdr` to bytes in Big-Endian ([`BE`]) and
/// stores them in `bytes2`.  Note that `io::Cursor` wraps an
/// in-memory buffer and provides it through `io::Write`.
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
///
/// // Encode UDP header `udp_hdr1` to bytes in `output2`.
/// // Because the UDP header is 8 bytes as defined above,
/// // only the first 8 bytes of `output2` are filled with data.
/// let mut output2 = Cursor::new(vec![0_u8; 16]);
/// let size2 = output2.decastf(&udp_hdr1, BE)?;
/// let bytes2 = output2.into_inner();
///
/// // `udp_hdr1` should be encoded as following (8 bytes)
/// let bytes3: [u8; 8] = [0xC3, 0xC9, 0x00, 0x35, 0x00, 0x32, 0x82, 0x3F];
///
/// // Check the results (bytes2)
/// assert_eq!(size2, 8);
/// assert_eq!(&bytes2[0..8], &bytes3[0..8]);
/// assert_eq!(&bytes2[8..16], &[0_u8; 8]);
/// # Ok(())
/// # }
/// ```
///
/// # Description
///
/// All methods in trait `DecastIO` `decast` one or more variables to
/// a number of bytes and writes to I/O.  The type of the value(s) can
/// be explicitly specified as the generic type parameter of its
/// method or simply omitted because the Rust compiler can infer from
/// the argument.  The endianness of resulting value(s) is flipped
/// when required and necessary.  Currently, only an implementation
/// for trait `std::io::Write` is provided.
///
/// The output `self` should have enough room to encode to the
/// specified number of value(s) of the specified type `T`.  If there
/// is enough room, the specified variable(s) is/are encoded to bytes
/// and written to output `self`.  If successful, the size of written
/// bytes are returned in `Ok`().  If I/O error is detected,
/// `Err`(`std::io::Error`) is returned.
///
/// When argument `endian` is specified, the endianness of resulting
/// bytes is flipped if necessary.
///
///
/// # Example 2
///
/// Because `io::Write` is implemented for `&[u8]`, `DecastIO` can
/// `decast` to memory.  The example below is almost the same with
/// Example 1 except it uses a mutable slice instead of `io::Cursor`.
///
/// ```
/// # use std::io::Result;
/// # fn main() { test(); }
/// # fn test() -> Result<()> {
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
///
/// // Encode UDP header `udp_hdr1` to bytes in `output2`.
/// // Because the UDP header is 8 bytes as defined above,
/// // only the first 8 bytes of `output2` are filled with data.
/// let mut bytes2 = [0_u8; 16];
/// let mut slice2 = &mut bytes2[..];
/// let size2 = slice2.decastf(&udp_hdr1, BE)?;
///
/// // `udp_hdr1` should be encoded as following (8 bytes)
/// let bytes3: [u8; 8] = [0xC3, 0xC9, 0x00, 0x35, 0x00, 0x32, 0x82, 0x3F];
///
/// // Check the result (slice2)
/// // Note: `slice2` contains unwritten part.
/// assert_eq!(slice2.len(), 8);
/// assert_eq!(slice2, [0_u8; 8]);
///
/// // Check the results (bytes2)
/// assert_eq!(size2, 8);
/// assert_eq!(&bytes2[0..8], &bytes3[0..8]);  // Written part
/// assert_eq!(&bytes2[8..16], &[0_u8; 8]);    // Unwritten part
/// # Ok(())
/// # }
/// ```
///
pub trait DecastIO {
    /// Encodes the value pointed by `val_ptr` of type `T` to bytes
    /// and writes them to output `self`.  The endianness of the
    /// resulting bytes is not flipped.
    fn decast<T>(&mut self, val_ptr: &T) -> Result<usize>
    where
	T: Cast;

    /// Encodes the value pointed by `val_ptr` of type `T` to bytes
    /// and writes them to output `self`.  The endianness of the
    /// resulting bytes is flipped if necessary.  The endianness of
    /// the resulting bytes is specified in `endian`.
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
    /// the resulting bytes is flipped if necessary.  The endianness
    /// of the resulting bytes is specified in `endian`.
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
	    self.write_all(val_ptr.asif_bytes_ref())?;
	}
	Ok(mem::size_of::<T>())
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
	    self.write_all(slice.asif_bytes_ref())?;
	}
	Ok(mem::size_of::<T>() * slice.len())
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
	    Ok(mem::size_of::<T>() * slice.len())
	}
    }
}
