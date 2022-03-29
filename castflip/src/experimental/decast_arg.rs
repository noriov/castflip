use core::option::Option;

use crate::{Cast, DecastMem, Endian, Flip};
#[cfg(doc)] use crate::BE;


///
/// Defines functions to `decast` and `Flip` on memory.
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
/// use castflip::experimental::DecastArg;
/// use castflip::{Cast, Flip, BE};
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
/// let size2 = UdpHdr::decastf(&mut bytes2, &udp_hdr1, BE)?;
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
/// All functions in trait `DecastArg` `decast` one or more variables
/// to a number of bytes on memory.  The methods whose name contain
/// 's' (= slice) or 'v' (= vector) `decast` to a series of structured
/// binary data.  The methods whose names end with 'f' flip the
/// endianness of the results.  Currently, only an implementation for
/// `[u8]` is provided.
///
/// The size of argument `bytes` should be larger than or equal to the
/// specified number of value(s) of the specified type `T`.  If there
/// is enough room, the specified variable(s) is/are encoded to bytes
/// and stored at the head of `bytes`.  Then, the size of sotred bytes
/// are returned in `Some`().  If there are not enough room, `None` is
/// returned.
///
/// When argument `endian` is specified, the endianness of resulting
/// bytes is flipped if necessary.
///
/// The API of this trait is an older form of [`DecastMem`].
///
pub trait DecastArg: Cast {
    /// Encodes the value pointed by `val_ptr` of type `T` to bytes
    /// and stores them at the head of `bytes`.  The endianness of the
    /// resulting bytes is not flipped.
    fn decast(bytes: &mut [u8], val_ptr: &Self) -> Option<usize>;

    /// Encodes the value pointed by `val_ptr` of type `T` to bytes
    /// and stores them at the head of `bytes`.  The endianness of the
    /// resulting bytes is flipped if necessary.  The endianness of
    /// the bytes is specified in `endian`.
    fn decastf(bytes: &mut [u8], val_ptr: &Self, endian: Endian)
	       -> Option<usize>
    where
	Self: Flip;

    /// Encodes value(s) in `slice` of type `T` to bytes and stores
    /// them at the head of `self`.  The endianness of the resulting
    /// bytes is not flipped.
    fn decasts(bytes: &mut [u8], slice: &[Self]) -> Option<usize>;

    /// Encodes value(s) in `slice` of type `T` to bytes and stores
    /// them at the head of `self`.  The endianness of the resulting
    /// bytes is flipped if necessary.  The endianness of the
    /// resulting bytes is specified in `endian`.
    fn decastsf(bytes: &mut [u8], slice: &[Self], endian: Endian)
		-> Option<usize>
    where
	Self: Flip;

    /// Encodes the slice of value(s) of type `T` pointed by `slice`
    /// to bytes and stores them at the head of `bytes`.  The
    /// endianness of the resulting bytes is not flipped.
    /// (This method is replaced by `decasts`)
    #[cfg(feature = "std")]
    fn decastv(bytes: &mut [u8], slice: &[Self]) -> Option<usize>;

    /// Encodes the slice of value(s) of type `T` pointed by `slice`
    /// to bytes and stores them at the head of `bytes`.  The
    /// endianness of the resulting bytes is flipped if necessary.
    /// The endianness of the resulting bytes is specified in
    /// `endian`.
    /// (This method is replaced by `decastsf`)
    #[cfg(feature = "std")]
    fn decastvf(bytes: &mut [u8], slice: &[Self], endian: Endian)
		-> Option<usize>
    where
	Self: Flip;
}


impl<T> DecastArg for T
where
    T: Cast
{
    fn decast(bytes: &mut [u8], val_ptr: &T) -> Option<usize>
    {
	bytes.decast(val_ptr)
    }

    fn decastf(bytes: &mut [u8], val_ptr: &T, endian: Endian) -> Option<usize>
    where
	Self: Flip
    {
	bytes.decastf(val_ptr, endian)
    }

    fn decasts(bytes: &mut [u8], slice: &[Self]) -> Option<usize>
    {
	bytes.decasts(slice)
    }

    fn decastsf(bytes: &mut [u8], slice: &[Self], endian: Endian)
		-> Option<usize>
    where
	Self: Flip
    {
	bytes.decastsf(slice, endian)
    }

    #[cfg(feature = "std")]
    fn decastv(bytes: &mut [u8], slice: &[T]) -> Option<usize>
    {
	bytes.decastv(slice)
    }

    #[cfg(feature = "std")]
    fn decastvf(bytes: &mut [u8], slice: &[T], endian: Endian) -> Option<usize>
    where
	Self: Flip
    {
	bytes.decastvf(slice, endian)
    }
}
