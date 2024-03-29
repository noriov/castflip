#[cfg(feature = "alloc")] use alloc::vec::Vec;
#[cfg(feature = "alloc")] use crate::experimental::PushBulk;

use core::mem::MaybeUninit;
use core::{mem, ptr};

use crate::{Cast, Endian, Flip};
#[cfg(doc)] use crate::BE;


///
/// Defines methods to `encast` and `endian-flip` on memory.
///
/// Note: In this crate, the term `encast` means decoding a number of
/// bytes to one or more values, the term `decast` means encoding one
/// or more variables to a number of bytes, and the term `endian-flip`
/// means flipping the endianness of value(s).
///
/// # Example
///
/// In the example below, method `encastf` decodes bytes in `bytes1`
/// in Big-Endian ([`BE`]) to variable `udp_hdr2` of type `UdpHdr`.
///
/// ```
/// # fn main() { test(); }
/// # fn test() -> Option<()> {
/// use castflip::{Cast, Flip, EncastMem, BE};
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
///
/// // Decode input `bytes1` to variable `udp_hdr2`.
/// // Because the UDP header is 8 bytes as defined above, only
/// // the first 8 bytes are decoded, remaining 8 bytes are ignored.
/// let udp_hdr2: UdpHdr = bytes1.encastf(BE)?;  // BE = Big-Endian
///
/// // Check the results (udp_hdr2)
/// assert_eq!(udp_hdr2.sport, 0xC3C9); // = 50121
/// assert_eq!(udp_hdr2.dport, 0x0035); // = 53 (DNS)
/// assert_eq!(udp_hdr2.len,   0x0032); // = 50
/// assert_eq!(udp_hdr2.sum,   0x823F);
/// # Some(())
/// # }
/// ```
///
/// # Description
///
/// All methods in trait `EncastMem` `encast` a number of bytes on
/// memory to one or more values of the specified type.  The type of
/// the value(s) can be explicitly specified as the generic type
/// parameter of the methods or implicitly specified so that the Rust
/// compiler can infer it.  The methods whose name contain 's' (=
/// slice) or 'v' (= vector) `encast` a series of structured binary
/// data.  The methods whose names end with 'f' flip the endianness of
/// the results.  Currently, an implementation for `[u8]` is provided.
///
/// The size of `self` should be larger than or equal to the specified
/// number of value(s) of the specified type `T`.  If there are enough
/// bytes, the required number of bytes at the head of `self` are
/// decoded to the specified type of value(s).  The remaining bytes
/// are ignored.  If successful, return value is enclosed in `Some`().
/// If there are not enough bytes, `None` is returned.
///
/// When argument `endian` is specified, the endianness of value(s) is
/// flipped if necessary.
///
pub trait EncastMem {
    /// Decodes the bytes at the head of `self` to a value of type `T`
    /// and returns the value.  The endianness of the resulting value
    /// is not flipped.
    fn encast<T>(&self) -> Option<T>
    where
	T: Cast;

    /// Decodes the bytes at the head of `self` to a value of type `T`
    /// and returns the value.  The endianness of the resulting value
    /// is flipped if necessary.  The endianness of the bytes is
    /// specified in `endian`.
    fn encastf<T>(&self, endian: Endian) -> Option<T>
    where
	T: Cast + Flip;

    /// Decodes the bytes at the head of `self` to value(s) of type
    /// `T` and fill `slice` with the value(s).  It returns the number
    /// of decoded bytes.  The endianness of the resulting value(s) is
    /// not flipped.
    fn encasts<T>(&self, slice: &mut [T]) -> Option<usize>
    where
	T: Cast;

    /// Decodes the bytes at the head of `self` to value(s) of type
    /// `T` and fill `slice` with the value(s).  It returns the number
    /// of decoded bytes.  The endianness of the resulting value(s) is
    /// flipped if necessary.  The endianness of the bytes is
    /// specified in `endian`.
    fn encastsf<T>(&self, slice: &mut [T], endian: Endian) -> Option<usize>
    where
	T: Cast + Flip;

    /// Decodes the bytes at the head of `self` to a vector of
    /// value(s) of type `T` and returns the vector.  The endianness
    /// of the resulting value(s) is not flipped.  The number of
    /// elements in the resulting vecotr is specified in `nelem`.
    #[cfg(feature = "alloc")]
    fn encastv<T>(&self, nelem: usize) -> Option<Vec<T>>
    where
	T: Cast;

    /// Decodes the bytes at the head of `self` to a vector of
    /// value(s) of type `T` and returns the vector.  The endianness
    /// of the resulting value(s) is flipped if necessary.  The
    /// endianness of the bytes is specified in `endian`.  The number
    /// of elements in the resulting vecotr is specified in `nelem`.
    #[cfg(feature = "alloc")]
    fn encastvf<T>(&self, nelem: usize, endian: Endian) -> Option<Vec<T>>
    where
	T: Cast + Flip;
}


impl EncastMem for [u8]
{
    #[inline]
    fn encast<T>(&self) -> Option<T>
    where
	T: Cast
    {
	let bytes = self.get(0 .. mem::size_of::<T>())?;

	// Decode a value of type T from `bytes`.
	let mut val = MaybeUninit::<T>::uninit();
	unsafe {
	    ptr::copy_nonoverlapping::<u8>(bytes.as_ptr(),
					   val.as_mut_ptr() as *mut u8,
					   mem::size_of::<T>());
	    Some(val.assume_init())
	}

	// The code fragment above is equivalent to:
	// unsafe {
	//     Some(ptr::read_unaligned(bytes.as_ptr() as *const T))
	// }
    }

    #[inline]
    fn encastf<T>(&self, endian: Endian) -> Option<T>
    where
	T: Cast + Flip
    {
	let mut val = self.encast::<T>()?;
	val.flip_var(endian);
	Some(val)
    }

    #[inline]
    fn encasts<T>(&self, slice: &mut [T]) -> Option<usize>
    where
	T: Cast
    {
	let bytes = self.get(0 .. mem::size_of::<T>() * slice.len())?;

	unsafe {
	    ptr::copy_nonoverlapping::<u8>(bytes.as_ptr(),
					   slice.as_mut_ptr() as *mut u8,
					   bytes.len());
	}

	Some(bytes.len())
    }

    #[inline]
    fn encastsf<T>(&self, slice: &mut [T], endian: Endian) -> Option<usize>
    where
	T: Cast + Flip
    {
	let size = self.encasts::<T>(slice)?;
	for elem in slice {
	    elem.flip_var(endian);
	}
	Some(size)
    }

    #[cfg(feature = "alloc")]
    #[inline]
    fn encastv<T>(&self, nelem: usize) -> Option<Vec<T>>
    where
	T: Cast
    {
	let mut vec: Vec<T> = Vec::new();

	unsafe {
	    vec.push_bulk(nelem, | new_slice | {
		self.encasts(new_slice).ok_or(())
	    }).ok()?;
	}

	Some(vec)
    }

    #[cfg(feature = "alloc")]
    #[inline]
    fn encastvf<T>(&self, nelem: usize, endian: Endian) -> Option<Vec<T>>
    where
	T: Cast + Flip
    {
	let mut vec = self.encastv::<T>(nelem)?;
	for elem in &mut vec {
	    elem.flip_var(endian);
	}
	Some(vec)
    }
}
