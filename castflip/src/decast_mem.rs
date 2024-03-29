use core::{mem, ptr};

use crate::{Cast, Endian, Flip};
#[cfg(doc)] use crate::BE;


///
/// Defines methods to `decast` and `endian-flip` on memory.
///
/// Note: In this crate, the term `encast` means decoding a number of
/// bytes to one or more values, the term `decast` means encoding one
/// or more variables to a number of bytes, and the term `endian-flip`
/// means flipping the endianness of value(s).
///
/// # Example
///
/// In the example below, method `decastf` encodes the value in
/// `udp_hdr1` of type `UdpHdr` to bytes in Big-Endian ([`BE`]) and
/// stores them in `bytes2`.
///
/// ```
/// # fn main() { test(); }
/// # fn test() -> Option<()> {
/// use castflip::{Cast, Flip, DecastMem, BE};
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
/// // Encode UDP header `udp_hdr1` to bytes in `bytes2`.
/// // Because the UDP header is 8 bytes as defined above,
/// // only the first 8 bytes of `bytes2` are filled with data.
/// let mut bytes2 = [0_u8; 16];
/// let size2 = bytes2.decastf(&udp_hdr1, BE)?;  // BE = Big-Endian
///
/// // `udp_hdr1` should be encoded as following (8 bytes)
/// let bytes3: [u8; 8] = [0xC3, 0xC9, 0x00, 0x35, 0x00, 0x32, 0x82, 0x3F];
///
/// // Check the results (bytes2)
/// assert_eq!(size2, 8);
/// assert_eq!(&bytes2[0..8], &bytes3[0..8]);  // Written part
/// assert_eq!(&bytes2[8..16], &[0_u8; 8]);    // Unwritten part
/// # Some(())
/// # }
/// ```
///
/// # Description
///
/// All methods in trait `DecastMem` `decast` one or more variables to
/// a number of bytes on memory.  The type of the value(s) may be
/// explicitly specified as the generic type parameter of its methods
/// or simply omitted because the Rust compiler can infer it from the
/// argument.  The methods whose name contain 's' (= slice) or 'v' (=
/// vector) `decast` to a series of structured binary data.  The
/// methods whose names end with 'f' flip the endianness of the
/// results.  Currently, an implementation for `[u8]` is provided.
///
/// The size of `self` should be larger than or equal to the specified
/// number of value(s) of the specified type `T`.  If there is enough
/// room, the specified variable(s) is/are encoded to bytes and stored
/// at the head of `self`.  Then, the size of stored bytes are
/// returned in `Some`().  If there are not enough room, `None` is
/// returned.
///
/// When argument `endian` is specified, the endianness of resulting
/// bytes is flipped if necessary.
///
pub trait DecastMem {
    /// Encodes the value pointed by `val_ptr` of type `T` to bytes
    /// and stores them at the head of `self`.  The endianness of the
    /// resulting bytes is not flipped.
    fn decast<T>(&mut self, val_ptr: &T) -> Option<usize>
    where
	T: Cast;

    /// Encodes the value pointed by `val_ptr` of type `T` to bytes
    /// and stores them at the head of `self`.  The endianness of the
    /// resulting bytes is flipped if necessary.  The endianness of
    /// the bytes is specified in `endian`.
    fn decastf<T>(&mut self, val_ptr: &T, endian: Endian) -> Option<usize>
    where
	T: Cast + Flip;

    /// Encodes value(s) in `slice` of type `T` to bytes and stores
    /// them at the head of `self`.  The endianness of the resulting
    /// bytes is not flipped.
    fn decasts<T>(&mut self, slice: &[T]) -> Option<usize>
    where
	T: Cast;

    /// Encodes value(s) in `slice` of type `T` to bytes and stores
    /// them at the head of `self`.  The endianness of the resulting
    /// bytes is flipped if necessary.  The endianness of the
    /// resulting bytes is specified in `endian`.
    fn decastsf<T>(&mut self, slice: &[T], endian: Endian) -> Option<usize>
    where
	T: Cast + Flip;

    /// Encodes value(s) in `slice` of type `T` to bytes and stores
    /// them at the head of `self`.  The endianness of the resulting
    /// bytes is not flipped.
    /// (This method is replaced by `decasts`)
    #[cfg(feature = "alloc")]
    fn decastv<T>(&mut self, slice: &[T]) -> Option<usize>
    where
	T: Cast;

    /// Encodes value(s) in `slice` of type `T` to bytes and stores
    /// them at the head of `self`.  The endianness of the resulting
    /// bytes is flipped if necessary.  The endianness of the
    /// resulting bytes is specified in `endian`.
    /// (This method is replaced by `decastsf`)
    #[cfg(feature = "alloc")]
    fn decastvf<T>(&mut self, slice: &[T], endian: Endian) -> Option<usize>
    where
	T: Cast + Flip;
}


impl DecastMem for [u8]
{
    #[inline]
    fn decast<T>(&mut self, val_ptr: &T) -> Option<usize>
    where
	T: Cast
    {
	let bytes = self.get_mut(0 .. mem::size_of::<T>())?;

	unsafe {
	    ptr::copy_nonoverlapping::<u8>(val_ptr as *const T as *const u8,
					   bytes.as_mut_ptr(),
					   mem::size_of::<T>());
	}

	Some(mem::size_of::<T>())

	// The unsafe block above is almost equivalent to:
	// unsafe {
	//    ptr::write_unaligned(bytes.as_mut_ptr() as *mut T,
	//                         ptr::read(val_ptr))
	// }
    }

    #[inline]
    fn decastf<T>(&mut self, val_ptr: &T, endian: Endian) -> Option<usize>
    where
	T: Cast + Flip
    {
	if !endian.need_swap() {
	    self.decast::<T>(val_ptr)
	} else {
	    self.decast::<T>(&val_ptr.flip_val_swapped())
	}
    }

    #[inline]
    fn decasts<T>(&mut self, slice: &[T]) -> Option<usize>
    where
	T: Cast
    {
	let bytes = self.get_mut(0 .. mem::size_of::<T>() * slice.len())?;

	unsafe {
	    ptr::copy_nonoverlapping::<u8>(slice.as_ptr() as *const u8,
					   bytes.as_mut_ptr(),
					   bytes.len());
	}

	Some(bytes.len())
    }

    #[inline]
    fn decastsf<T>(&mut self, slice: &[T], endian: Endian) -> Option<usize>
    where
	T: Cast + Flip
    {
	if !endian.need_swap() {
	    self.decasts::<T>(slice)
	} else {
	    let bytes = self.get_mut(0 .. mem::size_of::<T>() * slice.len())?;
	    let mut off = 0;
	    for elem in slice {
		bytes[off ..].decast::<T>(&elem.flip_val_swapped())?;
		off += mem::size_of::<T>();
	    }
	    Some(off)
	}
    }

    #[cfg(feature = "alloc")]
    #[inline]
    fn decastv<T>(&mut self, slice: &[T]) -> Option<usize>
    where
	T: Cast
    {
	self.decasts::<T>(slice)
    }

    #[cfg(feature = "alloc")]
    #[inline]
    fn decastvf<T>(&mut self, slice: &[T], endian: Endian) -> Option<usize>
    where
	T: Cast + Flip
    {
	self.decastsf::<T>(slice, endian)
    }
}
