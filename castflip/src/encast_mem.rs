use core::{mem, ptr};
#[cfg(feature = "alloc")] use alloc::vec::Vec;

use crate::{Cast, Endian, Flip};
#[cfg(doc)] use crate::BE;
#[cfg(feature = "alloc")] use crate::experimental::PushBulk;


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
    /// Creates a value of type `T` from a memory representation of
    /// the type at the head of a byte slice `self` and returns the
    /// resulting value in `Some(T)`.
    ///
    /// The endianness of the resulting value is the same as the
    /// endianness of the source bytes.  In typical cases, both are
    /// native.
    ///
    /// If `self` does not have enough bytes, `None` is returned.
    fn encast<T>(&self) -> Option<T>
    where
        T: Cast;

    /// Creates a value of type `T` from a memory representation of
    /// the type at the head of a byte slice `self` and returns the
    /// resulting value in `Some(T)`.
    ///
    /// The endianness of the resulting value is flipped to native
    /// endian.  The endianness of the source bytes must be correctly
    /// specified by `endian`.
    ///
    /// If `self` does not have enough bytes, `None` is returned.
    fn encastf<T>(&self, endian: Endian) -> Option<T>
    where
        T: Cast + Flip;

    /// Creates values of type `T` from memory representations of the
    /// type at the head of a byte slice `self`, saves the values in
    /// `slice`, and returns the number of the source bytes in
    /// `Some(usize)`.
    ///
    /// The endianness of each resulting value is the same as the
    /// endianness of the corresponding source bytes.  In typical
    /// cases, all are native.
    ///
    /// If `self` does not have enough bytes, `None` is returned.
    fn encasts<T>(&self, slice: &mut [T]) -> Option<usize>
    where
        T: Cast;

    /// Creates values of type `T` from memory representations of the
    /// type at the head of a byte slice `self`, saves the values in
    /// `slice`, and returns the number of the source bytes in
    /// `Some(usize)`.
    ///
    /// The endianness of each resulting value is flipped to native
    /// endian.  The endianness of the source bytes must be correctly
    /// specified by `endian`.
    ///
    /// If `self` does not have enough bytes, `None` is returned.
    fn encastsf<T>(&self, slice: &mut [T], endian: Endian) -> Option<usize>
    where
        T: Cast + Flip;

    /// Creates a vector of values of type `T` from memory
    /// representations of the type at the head of a byte slice `self`
    /// and returns the vector in `Some(Vec<T>)`.
    ///
    /// The number of elements in the resulting vector is specified by
    /// `nelem`.
    ///
    /// The endianness of each resulting value is the same as the
    /// endianness of the corresponding source bytes.  In typical
    /// cases, all are native.
    ///
    /// If `self` does not have enough bytes, `None` is returned.
    #[cfg(feature = "alloc")]
    fn encastv<T>(&self, nelem: usize) -> Option<Vec<T>>
    where
        T: Cast;

    /// Creates a vector of values of type `T` from memory
    /// representations of the type at the head of a byte slice `self`
    /// and returns the vector in `Some(Vec<T>)`.
    ///
    /// The number of elements in the resulting vector is specified by
    /// `nelem`.
    ///
    /// The endianness of each resulting value is flipped to native
    /// endian.  The endianness of the source bytes must be correctly
    /// specified by `endian`.
    ///
    /// If `self` does not have enough bytes, `None` is returned.
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
        T: Cast,
    {
        if self.len() >= mem::size_of::<T>() {
            unsafe {
                // ptr::read_unaligned copies the byte slice at the
                // head of `self` into its internal variable of type T
                // and returns the value.
                Some(ptr::read_unaligned(self.as_ptr() as *const T))
            }
        } else {
            None
        }

/*
        // The previous version was:

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
*/
    }

    #[inline]
    fn encastf<T>(&self, endian: Endian) -> Option<T>
    where
        T: Cast + Flip,
    {
        let mut val = self.encast::<T>()?;
        val.flip_var(endian); // Flip the endianness if necessary.
        Some(val)
    }

    #[inline]
    fn encasts<T>(&self, slice: &mut [T]) -> Option<usize>
    where
        T: Cast,
    {
        let nbytes = mem::size_of::<T>() * slice.len();

        if self.len() >= nbytes {
            unsafe {
                // Copy the byte slice at the head of `self` into
                // `slice`.
                ptr::copy_nonoverlapping(self.as_ptr() as *const u8,
                                         slice.as_mut_ptr() as *mut u8,
                                         nbytes);
            }
            Some(nbytes)
        } else {
            None
        }

/*
        // The previous version was:

        let bytes = self.get(0 .. mem::size_of::<T>() * slice.len())?;

        unsafe {
            ptr::copy_nonoverlapping::<u8>(bytes.as_ptr(),
                                           slice.as_mut_ptr() as *mut u8,
                                           bytes.len());
        }

        Some(bytes.len())
*/
    }

    #[inline]
    fn encastsf<T>(&self, slice: &mut [T], endian: Endian) -> Option<usize>
    where
        T: Cast + Flip,
    {
        if !endian.need_swap() {
            // The endianness is kept untouched.
            self.encasts::<T>(slice)
        } else {
            if self.len() >= mem::size_of::<T>() * slice.len() {
                // The endianness is flipped to swapped endian.
                self.encastsf_swapped(slice)
            } else {
                None
            }
        }

/*
        // The previous version was:

        let size = self.encasts::<T>(slice)?;
        for elem in slice {
            elem.flip_var(endian);
        }
        Some(size)
*/
    }

    #[cfg(feature = "alloc")]
    fn encastv<T>(&self, nelem: usize) -> Option<Vec<T>>
    where
        T: Cast,
    {
        let mut vec: Vec<T> = Vec::new();

        unsafe {
            // Append values created from `self` to `vec`.
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
        Some(vec)
*/
    }
}


///
/// Defines internal methods to `encast` and `endian-flip` on memory.
///
trait EncastMemInternal {
    /// Creates values of type `T` from memory representations of the
    /// type at the head of a byte slice `self`, saves the values in
    /// `slice`, and returns the number of the source bytes in
    /// `Some(usize)`.
    ///
    /// The endianness of each resulting value is the swapped one of
    /// the endianness of the corresponding source bytes.
    ///
    /// If `self` does not have enough bytes, `None` is returned.
    fn encastsf_swapped<T>(&self, slice: &mut [T]) -> Option<usize>
    where
        T: Cast + Flip;

    /// Creates a vector of values of type `T` from memory
    /// representations of the type at the head of a byte slice
    /// `self`, and returns the vector in `Some(Vec<T>)`.
    ///
    /// The number of elements in the resulting vector is specified by
    /// `nelem`.
    ///
    /// The endianness of each resulting value is the swapped one of
    /// the endianness of the corresponding source bytes.
    ///
    /// If `self` does not have enough bytes, `None` is returned.
    fn encastvf_swapped<T>(&self, nelem: usize) -> Option<Vec<T>>
    where
        T: Cast + Flip;
}

impl EncastMemInternal for [u8]
{
    fn encastsf_swapped<T>(&self, slice: &mut [T]) -> Option<usize>
    where
        T: Cast + Flip,
    {
        debug_assert!(self.len() >= mem::size_of::<T>() * slice.len());

        let mut off = 0;

        // Save endian-flipped values created from `self` to `slice`.
        for elem in slice {
            *elem = self[off ..].encast::<T>()?.flip_val_swapped();
            off += mem::size_of::<T>();
        }

        Some(off)
    }

    fn encastvf_swapped<T>(&self, nelem: usize) -> Option<Vec<T>>
    where
        T: Cast + Flip,
    {
        let mut vec: Vec<T> = Vec::new();

        unsafe {
            // Append endian-flipped values created from `self` to `vec`.
            vec.push_bulk(nelem, | new_slice | {
                match self.encastsf_swapped(new_slice) {
                    Some(nbytes) => Ok(nbytes),
                    None => Err(()),
                }
            }).ok()?;
        }

        Some(vec)
    }
}
