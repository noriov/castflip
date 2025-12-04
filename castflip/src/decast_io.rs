use core::mem;
use std::io;

use crate::{Cast, Endian, Flip};
use crate::experimental::AsifBytes;
#[cfg(doc)] use crate::BE;


///
/// Defines methods to `decast` and `endian-flip` through [`io::Write`].
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
/// the argument.  The methods whose name contain 's' (= slice) or 'v'
/// (= vector) `decast` to a series of structured binary data.  The
/// methods whose names end with 'f' flip the endianness of the
/// results.  Currently, an implementation for trait [`io::Write`] is
/// provided.
///
/// The output `self` should have enough room to encode to the
/// specified number of value(s) of the specified type `T`.  If there
/// is enough room, the specified variable(s) is/are encoded to bytes
/// and written to output `self`.  If successful, the size of written
/// bytes is returned in `Ok`().  If I/O error is detected,
/// `Err`([`io::Error`]) is returned.  The type of the return value is
/// [`io::Result`].
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
    /// Writes the memory representation of a value `val` to a writer
    /// `self` and returns the number of the output bytes in
    /// `Ok(usize)`.
    ///
    /// The endianness of the output bytes is the same as the
    /// endianness of `val`.  In typical cases, both are native.
    ///
    /// If an error is detected, `Err(io::Error)` is returned.
    fn decast<T>(&mut self, val: &T) -> io::Result<usize>
    where
        T: Cast;

    /// Writes the memory representation of a value `val` to a writer
    /// `self` and returns the number of the output bytes is
    /// `Ok(usize)`.
    ///
    /// The endianness of the output bytes is flipped as specified by
    /// `endian` on the assumption that the endianness of `val` is
    /// native endian.
    ///
    /// If an error is detected, `Err(io::Error)` is returned.
    fn decastf<T>(&mut self, val: &T, endian: Endian) -> io::Result<usize>
    where
        T: Cast + Flip;

    /// Writes the memory representation of the values in `slice` to a
    /// writer `self` and returns the number of the output bytes is
    /// `Ok(usize)`.
    ///
    /// The endianness of the output bytes is the same as the
    /// endianness of `slice`.  In typical cases, both are native.
    ///
    /// If an error is detected, `Err(io::Error)` is returned.
    fn decasts<T>(&mut self, slice: &[T]) -> io::Result<usize>
    where
        T: Cast;

    /// Writes the memory representation of the values `slice` to a
    /// writer `self` and returns the number of the output bytes is
    /// `Ok(usize)`.
    ///
    /// The endianness of the output bytes is flipped as specified by
    /// `endian` on the assumption that the endianness of `slice` is
    /// native endian.
    ///
    /// If an error is detected, `Err(io::Error)` is returned.
    fn decastsf<T>(&mut self, slice: &[T], endian: Endian) -> io::Result<usize>
    where
        T: Cast + Flip;

    /// Is equivalent to `decasts`.
    ///
    /// This method will be deprecated in a future release.
    fn decastv<T>(&mut self, slice: &[T]) -> io::Result<usize>
    where
        T: Cast;

    /// Is equivalent to `decastsf`.
    ///
    /// This method will be deprecated in a future release.
    fn decastvf<T>(&mut self, slice: &[T], endian: Endian) -> io::Result<usize>
    where
        T: Cast + Flip;
}


impl<W> DecastIO for W
where
    W: ?Sized + io::Write,
{
    #[inline]
    fn decast<T>(&mut self, val: &T) -> io::Result<usize>
    where
        T: Cast,
    {
        unsafe {
            // Write the memory representation of `val` to a writer
            // `self`.
            self.write_all(val.asif_bytes_ref())?;
        }

        Ok(mem::size_of::<T>())
    }

    #[inline]
    fn decastf<T>(&mut self, val: &T, endian: Endian) -> io::Result<usize>
    where
        T: Cast + Flip,
    {
        if !endian.need_swap() {
            // The endianness is kept untouched.
            self.decast::<T>(val)
        } else {
            // The endianness is flipped to swapped endian.
            self.decast::<T>(&val.flip_val_swapped())
        }
    }

    #[inline]
    fn decasts<T>(&mut self, slice: &[T]) -> io::Result<usize>
    where
        T: Cast,
    {
        unsafe {
            // Write the memory representations of the values in
            // `slice` to a writer `self`.
            self.write_all(slice.asif_bytes_ref())?;
        }

        Ok(mem::size_of::<T>() * slice.len())
    }

    #[inline]
    fn decastsf<T>(&mut self, slice: &[T], endian: Endian) -> io::Result<usize>
    where
        T: Cast + Flip,
    {
        if !endian.need_swap() {
            // The endianness is kept untouched.
            self.decasts::<T>(slice)
        } else {
            // The endianness is flipped to swapped endian.
            self.decastsf_swapped(slice)
        }
    }

    #[inline]
    fn decastv<T>(&mut self, slice: &[T]) -> io::Result<usize>
    where
        T: Cast,
    {
        // `decastv` is equivalent to `decasts`.
        self.decasts(slice)
    }

    #[inline]
    fn decastvf<T>(&mut self, slice: &[T], endian: Endian) -> io::Result<usize>
    where
        T: Cast + Flip,
    {
        // `decastvf` is equivalent to `decastsf`.
        self.decastsf(slice, endian)
    }
}


///
/// Defines an internal method to `decast` and `endian-flip` through
/// [`io::Write`].
///
trait DecastIOInternal {
    /// Writes the memory representation of the values in `slice` to a
    /// writer `self` and returns the number of output bytes in
    /// `Ok(usize)`.
    ///
    /// The endianness of the output bytes is the swapped one of the
    /// endianness of `slice`.
    ///
    /// If an error is detected, `Err(io::Error)` is returned.
    fn decastsf_swapped<T>(&mut self, slice: &[T]) -> io::Result<usize>
    where
        T: Cast + Flip;
}

impl<W> DecastIOInternal for W
where
    W: ?Sized + io::Write
{
    fn decastsf_swapped<T>(&mut self, slice: &[T]) -> io::Result<usize>
    where
        T: Cast + Flip,
    {
        for elem in slice {
            // Flip the endianness of the value in `elem` to swapped
            // endian and write the memory representation to `self`.
            self.decast::<T>(&elem.flip_val_swapped())?;
        }

        Ok(mem::size_of::<T>() * slice.len())
    }
}
