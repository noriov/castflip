use core::mem::MaybeUninit;
use core::{mem, slice};

use crate::Cast;


///
/// Defines methods to convert a reference to a variable or a slice
/// into a phantom reference to a lice of bytes without copying data.
///
/// Trait `AsifBytes` is used internally in this crate.
///
/// # Example 1
///
/// In the example below, method `asif_bytes_ref` converts the
/// original reference to type `ElfIdHdr` into a phantom reference to
/// slice `bytes2` without copying data.
///
/// ```
/// use castflip::experimental::AsifBytes;
/// use castflip::Cast;
///
/// #[repr(C)]
/// #[derive(Cast)]
/// struct ElfIdHdr {
///     magic:    [u8; 4],  //00-03: Magic Number 0x7F "ELF"
///     class:    u8,       //04   : File Class
///     encoding: u8,       //05   : Data Encoding
///     version:  u8,       //06   : Version (should be 1)
///     os_abi:   u8,       //07   : OS and ABI
///     abi_ver:  u8,       //08   : ABI Version
///     pad:      [u8; 7],  //09-0F: Padding (should be 0)
/// }
///
/// // Input data: ELF Identification (16 bytes)
/// let hdr1 = ElfIdHdr {
///     magic:    *b"\x7FELF",  // 7F 45 4C 46
///     class:    2,
///     encoding: 1,
///     version:  1,
///     os_abi:   0,
///     abi_ver:  0,
///     pad:      [0_u8; 7],
/// };
///
/// unsafe {
///     // Convert the original reference to `hdr1` into `bytes2`.
///     // Both &`hdr1` and `bytes2` point to the same entity.
///     let bytes2 = hdr1.asif_bytes_ref();
///
///     // Check the result (bytes2)
///     assert_eq!(bytes2, &[0x7F, 0x45, 0x4C, 0x46, 0x02, 0x01, 0x01, 0x00,
///                          0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
/// }
/// ```
///
/// Note: [ELF] is a common standard file format for executable files.
/// ELF Identification is the first 16 bytes of the ELF header.
///
/// [ELF]: https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
///
/// # Example 2
///
/// In the example below, method `asif_bytes_ref` converts the
/// original reference to an array of type `Pair` into a phantom
/// reference to slice `bytes2` without copying data.
///
/// ```
/// use castflip::experimental::AsifBytes;
/// use castflip::Cast;
///
/// #[repr(C)]
/// #[derive(Cast)]
/// struct Pair (u16, u16); // 4 bytes
///
/// // Input data (16 bytes)
/// let pairs1 = [Pair(0x3031, 0x3233), Pair(0x3435, 0x3637),
///               Pair(0x3839, 0x3A3B), Pair(0x3C3D, 0x3E3F)];
///
/// unsafe {
///     // Convert the original reference to `pairs1` into `bytes2`.
///     // Both &`pairs1` and `bytes2` point to the same entity.
///     let bytes2 = pairs1.asif_bytes_ref();
///
///     // Check the result.
///     if cfg!(target_endian = "little") {
///         assert_eq!(bytes2,
///                    &[0x31, 0x30, 0x33, 0x32, 0x35, 0x34, 0x37, 0x36,
///                      0x39, 0x38, 0x3B, 0x3A, 0x3D, 0x3C, 0x3F, 0x3E]);
///     } else if cfg!(target_endian = "big") {
///         assert_eq!(bytes2,
///                    &[0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37,
///                      0x38, 0x39, 0x3A, 0x3B, 0x3C, 0x3D, 0x3E, 0x3F]);
///     }
/// }
/// ```
///
/// # Safety
///
/// We do not understand clearly what kind of problems could occur
/// with this trait.
///
/// Because the Rust compiler would not recognize what is happening,
/// it may reorder instructions unexpectedly.  When a phantom
/// reference is created by this trait, it would be better not to use
/// the original reference until the phantom reference is dropped,
/// expecially when the original reference is mutable.
///
pub trait AsifBytes {
    /// Converts a reference to `self` into a phantom reference to a
    /// slice of u8 without copying data.  `self` can be a variable or
    /// a slice.
    unsafe fn asif_bytes_ref(&self) -> &[u8];

    /// Converts a mutable reference to `self` into a mutable phantom
    /// reference to a slice of u8 without copying data.  `self` can
    /// be a variable or a slice.
    unsafe fn asif_bytes_mut(&mut self) -> &mut [u8];
}


impl<T> AsifBytes for T
where
    T: Cast
{
    unsafe fn asif_bytes_ref(&self) -> &[u8] {
	slice::from_raw_parts::<u8>(self as *const T as *const u8,
				    mem::size_of::<T>())
    }

    unsafe fn asif_bytes_mut(&mut self) -> &mut [u8] {
	slice::from_raw_parts_mut::<u8>(self as *mut T as *mut u8,
					mem::size_of::<T>())
    }
}


impl<T> AsifBytes for [T]
where
    T: Cast
{
    unsafe fn asif_bytes_ref(&self) -> &[u8] {
	slice::from_raw_parts::<u8>(self as *const [T] as *const u8,
				    mem::size_of::<T>() * self.len())
    }

    unsafe fn asif_bytes_mut(&mut self) -> &mut [u8] {
	slice::from_raw_parts_mut::<u8>(self as *mut [T] as *mut u8,
					mem::size_of::<T>() * self.len())
    }
}


impl<T> AsifBytes for MaybeUninit<T>
where
    T: Cast
{
    unsafe fn asif_bytes_ref(&self) -> &[u8] {
	slice::from_raw_parts::<u8>(self.as_ptr() as *const u8,
				    mem::size_of::<T>())
    }

    unsafe fn asif_bytes_mut(&mut self) -> &mut [u8] {
	slice::from_raw_parts_mut::<u8>(self.as_mut_ptr() as *mut u8,
					mem::size_of::<T>())
    }
}
