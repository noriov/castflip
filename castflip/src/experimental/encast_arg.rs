#[cfg(feature = "alloc")] use alloc::vec::Vec;

use crate::{Cast, EncastMem, Endian, Flip};
#[cfg(doc)] use crate::BE;


///
/// Defines functions to `encast` and `Flip` on memory.
///
/// Note: In this crate, the term `encast` means decoding a number of
/// bytes to one or more values, the term `decast` means encoding one
/// or more variables to a number of bytes, and the term `endian-flip`
/// means flipping the endianness of value(s).
///
/// # Example
///
/// In the example below, function `encastf` decodes bytes in `bytes1`
/// in Big-Endian ([`BE`]) to variable `udp_hdr2` of type `UdpHdr`.
///
/// ```
/// # fn main() { test(); }
/// # fn test() -> Option<()> {
/// use castflip::experimental::EncastArg;
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
/// // Input data: UDP header (8 bytes) + part of DNS header (8 bytes)
/// let bytes1: [u8; 16] = [0xC3, 0xC9, 0x00, 0x35, 0x00, 0x32, 0x82, 0x3F,
///                         0x1A, 0xD1, 0x01, 0x20, 0x00, 0x01, 0x00, 0x00];
///
/// // Decode input `bytes1` to variable `udp_hdr2`.
/// // Because the UDP header is 8 bytes as defined above, only
/// // the first 8 bytes are decoded, remaining 8 bytes are ignored.
/// let udp_hdr2 = UdpHdr::encastf(&bytes1, BE)?;  // BE = Big-Endian
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
/// All functions in trait `EncastArg` `encast` a number of bytes on
/// the memory to one or more values of the specified type.  The
/// methods whose name contain 's' (= slice) or 'v' (= vector)
/// `encast` a series of structured binary data.  The methods whose
/// names end with 'f' flip the endianness of the results.
///
/// The bytes in argument `bytes` should be larger than or equal to
/// the specified number of value(s) of the specified type `T`.  If
/// there are enough bytes, the required number of bytes at the head
/// of `bytes` are decoded to the specified type of value(s).  The
/// remaining bytes are ignored.  If successful, return value is
/// enclosed in `Some`().  If there are not enough bytes, `None` is
/// returned.
///
/// When argument `endian` is specified, the endianness of value(s) is
/// flipped if necessary.
///
/// The API of this trait is an older form of [`EncastMem`].
///
pub trait EncastArg: Cast {
    /// Decodes the bytes at the head of `bytes` to a value of type
    /// `T` and returns the value.  The endianness of the resulting
    /// value is not flipped.
    fn encast(bytes: &[u8]) -> Option<Self>;

    /// Decodes the bytes at the head of `bytes` to a value of type
    /// `T` and returns the value.  The endianness of the resulting
    /// value is flipped if necessary.  The endianness of the bytes is
    /// specified in `endian`.
    fn encastf(bytes: &[u8], endian: Endian) -> Option<Self>
    where
	Self: Flip;

    /// Decodes the bytes at the head of `bytes` to value(s) of type
    /// `T` and fill `slice` with the value(s).  It returns the number
    /// of decoded bytes.  The endianness of the resulting value(s) is
    /// not flipped.
    fn encasts(bytes: &[u8], slice: &mut [Self]) -> Option<usize>;

    /// Decodes the bytes at the head of `bytes` to value(s) of type
    /// `T` and fill `slice` with the value(s).  It returns the number
    /// of decoded bytes.  The endianness of the resulting value(s) is
    /// flipped if necessary.  The endianness of the bytes is
    /// specified in `endian`.
    fn encastsf(bytes: &[u8], slice: &mut [Self], endian: Endian)
		-> Option<usize>
    where
	Self: Flip;

    /// Decodes the bytes at the head of `bytes` to a vector of
    /// value(s) of type `T` and returns the vector.  The endianness
    /// of the resulting value(s) is not flipped.  The number of
    /// elements in the resulting vecotr is specified in `nelem`.
    #[cfg(feature = "alloc")]
    fn encastv(bytes: &[u8], nelem: usize) -> Option<Vec<Self>>;

    /// Decodes the bytes at the head of `bytes` to a vector of
    /// value(s) of type `T` and returns the vector.  The endianness
    /// of the resulting value(s) is flipped if necessary.  The
    /// endianness of the bytes is specified in `endian`.  The number
    /// of elements in the resulting vecotr is specified in `nelem`.
    #[cfg(feature = "alloc")]
    fn encastvf(bytes: &[u8], nelem: usize, endian: Endian)
		-> Option<Vec<Self>>
    where
	Self: Flip;
}


impl<T> EncastArg for T
where
    T: Cast
{
    fn encast(bytes: &[u8]) -> Option<Self>
    {
	bytes.encast::<Self>()
    }

    fn encastf(bytes: &[u8], endian: Endian) -> Option<Self>
    where
	Self: Flip
    {
	bytes.encastf::<Self>(endian)
    }

    fn encasts(bytes: &[u8], slice: &mut [Self]) -> Option<usize>
    {
	bytes.encasts::<Self>(slice)
    }

    fn encastsf(bytes: &[u8], slice: &mut [Self], endian: Endian)
		-> Option<usize>
    where
	Self: Flip
    {
	bytes.encastsf::<Self>(slice, endian)
    }

    #[cfg(feature = "alloc")]
    fn encastv(bytes: &[u8], nelem: usize) -> Option<Vec<Self>>
    {
	bytes.encastv::<Self>(nelem)
    }

    #[cfg(feature = "alloc")]
    fn encastvf(bytes: &[u8], nelem: usize, endian: Endian)
		-> Option<Vec<Self>>
    where
	Self: Flip
    {
	bytes.encastvf::<Self>(nelem, endian)
    }
}
