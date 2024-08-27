use core::mem::MaybeUninit;
use core::mem;
use std::io;

use crate::{Cast, Endian, Flip};
#[cfg(doc)] use crate::BE;
use crate::experimental::{AsifBytes, PushBulk};


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
    /// Creates a value of type `T` from a binary representation of
    /// the type read from a reader `self` and returns the resulting
    /// value in `Ok(T)`.
    ///
    /// The endianness of the resulting value is the same as the
    /// endianness of the source bytes.  In typical cases, both are
    /// native.
    ///
    /// If an error is detected, `Err(io::Error)` is returned.
    fn encast<T>(&mut self) -> io::Result<T>
    where
	T: Cast;

    /// Creates a value of type `T` from a binary representation of
    /// the type read from a reader `self` and returns the resulting
    /// value in `Ok(T)`.
    ///
    /// The endianness of the resulting value is flipped to native
    /// endian.  The endianness of the source bytes must be correctly
    /// specified by `endian`.
    ///
    /// If an error is detected, `Err(io::Error)` is returned.
    fn encastf<T>(&mut self, endian: Endian) -> io::Result<T>
    where
	T: Cast + Flip;

    /// Creates values of type `T` from binary representations of the
    /// type read from a reader `self`, saves the values in `slice`,
    /// and returns the number of the source bytes in `Ok(usize)`.
    ///
    /// The endianness of each resulting value is the same as the
    /// endianness of the corresponding source bytes.  In typical
    /// cases, all are native.
    ///
    /// If an error is detected, `Err(io::Error)` is returned.
    fn encasts<T>(&mut self, slice: &mut [T]) -> io::Result<usize>
    where
	T: Cast;

    /// Creates values of type `T` from binary representations of the
    /// type read from a reader `self`, saves the values in `slice`,
    /// and returns the number of the source bytes in `Ok(usize)`.
    ///
    /// The endianness of each resulting value is flipped to native
    /// endian.  The endianness of the source bytes must be correctly
    /// specified by `endian`.
    ///
    /// If an error is detected, `Err(io::Error)` is returned.
    fn encastsf<T>(&mut self, slice: &mut [T], endian: Endian)
		   -> io::Result<usize>
    where
	T: Cast + Flip;

    /// Creates a vector of values of type `T` from binary
    /// representations of the type read from a reader `self` and
    /// returns the vector in `Ok(Vec<T>)`.
    ///
    /// The number of elements in the resulting vector is specified by
    /// `nelem`.
    ///
    /// The endianness of each resulting value is the same as the
    /// endianness of the corresponding source bytes.  In typical
    /// cases, all are native.
    ///
    /// If an error is detected, `Err(io::Error)` is returned.
    fn encastv<T>(&mut self, nelem: usize) -> io::Result<Vec<T>>
    where
	T: Cast;

    /// Creates a vector of values of type `T` from binary
    /// representations of the type read from a reader `self` and
    /// returns the vector in `Ok(Vec<T>)`.
    ///
    /// The number of elements in the resulting vector is specified by
    /// `nelem`.
    ///
    /// The endianness of each resulting value is flipped to native
    /// endian.  The endianness of the source bytes must be correctly
    /// specified by `endian`.
    ///
    /// If an error is detected, `Err(io::Error)` is returned.
    fn encastvf<T>(&mut self, nelem: usize, endian: Endian)
		   -> io::Result<Vec<T>>
    where
	T: Cast + Flip;
}


impl<R> EncastIO for R
where
    R: ?Sized + io::Read,
{
    #[inline]
    fn encast<T>(&mut self) -> io::Result<T>
    where
	T: Cast,
    {
	unsafe {
	    let mut val = MaybeUninit::<T>::uninit();

	    // Read raw bytes from `self` and save them in `val`.
	    self.read_exact(val.asif_bytes_mut())?;

	    Ok(val.assume_init())
	}
    }

    #[inline]
    fn encastf<T>(&mut self, endian: Endian) -> io::Result<T>
    where
	T: Cast + Flip,
    {
	let mut val = self.encast::<T>()?;
	val.flip_var(endian); // Flip the endianness if necessary.
	Ok(val)
    }

    #[inline]
    fn encasts<T>(&mut self, slice: &mut [T]) -> io::Result<usize>
    where
	T: Cast,
    {
	unsafe {
	    // Read raw bytes from `self` and save them in `slice`.
	    self.read_exact(slice.asif_bytes_mut())?;
	}

	Ok(mem::size_of::<T>() * slice.len())
    }

    #[inline]
    fn encastsf<T>(&mut self, slice: &mut [T], endian: Endian)
		   -> io::Result<usize>
    where
	T: Cast + Flip,
    {
	if !endian.need_swap() {
	    // The endianness is kept untouched.
	    self.encasts::<T>(slice)
	} else {
	    // The endianness is flipped to swapped endian.
	    self.encastsf_swapped(slice)
	}

/*
	// The previous version was:

	let size = self.encasts::<T>(slice)?;
	for elem in slice {
	    elem.flip_var(endian);
	}
	Ok(size)
*/
    }

    fn encastv<T>(&mut self, nelem: usize) -> io::Result<Vec<T>>
    where
	T: Cast,
    {
	let mut vec: Vec<T> = Vec::new();

	unsafe {
	    // Append values created from `self` to `vec`.
	    vec.push_bulk(nelem, | new_slice | {
		self.encasts(new_slice)
	    })?;
	}

	Ok(vec)
    }

    #[inline]
    fn encastvf<T>(&mut self, nelem: usize, endian: Endian)
		   -> io::Result<Vec<T>>
    where
	T: Cast + Flip,
    {
	if !endian.need_swap() {
	    // The endianness is kept untouched.
	    self.encastv::<T>(nelem)
	} else {
	    // The endianness is flipped to swapped endian.
	    self.encastvf_swapped(nelem)
	}

/*
	// The previous version was:

	let mut vec = self.encastv::<T>(nelem)?;
	for elem in &mut vec {
	    elem.flip_var(endian);
	}
	Ok(vec)
*/
    }
}


///
/// Defines internal methods to `encast` and `endian-flip` through
/// [`io::Read`].
///
trait EncastIOInternal {
    /// Creates values of type `T` from binary representations of the
    /// type read from a reader `self`, saves the values in `slice`,
    /// and returns the number of the source bytes in `Ok(usize)`.
    ///
    /// The endianness of each resulting value is the swapped one of
    /// the endianness of the corresponding source bytes.
    ///
    /// If an error is detected, `Err(io::Error)` is returned.
    fn encastsf_swapped<T>(&mut self, slice: &mut [T]) -> io::Result<usize>
    where
	T: Cast + Flip;

    /// Creates a vector of values of type `T` from binary
    /// representations of the type read from a reader `self`, and
    /// returns the vector in `Ok(Vec<T>)`.
    ///
    /// The number of elements in the resulting vector is specified by
    /// `nelem`.
    ///
    /// The endianness of each resulting value is the swapped one of
    /// the endianness of the corresponding source bytes.
    ///
    /// If an error is detected, `Err(io::Error)` is returned.
    fn encastvf_swapped<T>(&mut self, nelem: usize) -> io::Result<Vec<T>>
    where
	T: Cast + Flip;
}

impl<R> EncastIOInternal for R
where
    R: ?Sized + io::Read,
{
    fn encastsf_swapped<T>(&mut self, slice: &mut [T]) -> io::Result<usize>
    where
	T: Cast + Flip,
    {
	let nbytes = mem::size_of::<T>() * slice.len();

	// Save endian-flipped values created from `self` to `slice`.
	for elem in slice {
	    *elem = self.encast::<T>()?.flip_val_swapped();
	}

	Ok(nbytes)
    }

    fn encastvf_swapped<T>(&mut self, nelem: usize) -> io::Result<Vec<T>>
    where
	T: Cast + Flip,
    {
	let mut vec: Vec<T> = Vec::new();

	unsafe {
	    // Append endian-flipped values created from `self` to `vec`.
	    vec.push_bulk(nelem, | new_slice | {
		let nbytes = mem::size_of::<T>() * new_slice.len();
		for elem in new_slice {
		    *elem = self.encast::<T>()?.flip_val_swapped();
		}
		Ok::<usize, io::Error>(nbytes)
	    })?;
	}

	Ok(vec)
    }
}
