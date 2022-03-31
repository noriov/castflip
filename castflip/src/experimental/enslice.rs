#[cfg(not(feature = "std"))] use core::option::Option;

use core::slice;

use crate::Cast;
use crate::experimental::Reslice;


///
/// Defines methods to convert a reference to a variable into a
/// phantom reference to a slice of the specified type without copying
/// data.
///
/// The antonym of [`Enslice`] is [`Deslice`].
///
/// [`Deslice`]: ./trait.Deslice.html
///
/// # Example
///
/// In the example below, method `enslice` converts the original
/// reference to type `ElfIdHdr` into a phantom reference to
/// slice `bytes2` without copying data.
///
/// ```
/// # fn main() { test(); }
/// # fn test() -> Option<()> {
/// use castflip::experimental::Enslice;
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
///     // Hence, `bytes2` is the same with `bytes1`.
///     let bytes2: &[u8] = hdr1.enslice()?;
///
///     // Check the result (bytes2)
///     assert_eq!(bytes2, &[0x7F, 0x45, 0x4C, 0x46, 0x02, 0x01, 0x01, 0x00,
///                          0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
/// }
/// # Some(())
/// # }
/// ```
///
/// Note: [ELF] is a common standard file format for executable files.
/// ELF Identification is the first 16 bytes of the ELF header.
///
/// [ELF]: https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
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
pub trait Enslice {
    /// Converts a reference to `self` of a type
    /// into a phantom reference to a slice of type `U`.
    unsafe fn enslice<U>(&self) -> Option<&[U]>
    where
	U: Cast;

    /// Converts a mutable reference to `self` of a type
    /// into a mutable phantom reference to a slice of type `U`.
    unsafe fn enslice_mut<U>(&mut self) -> Option<&mut [U]>
    where
	U: Cast;
}


impl<T> Enslice for T
where
    T: Cast
{
    unsafe fn enslice<U>(&self) -> Option<&[U]>
    where
	U: Cast
    {
	slice::from_raw_parts::<T>(self as *const T, 1)
	    .reslice::<U>()
    }

    unsafe fn enslice_mut<U>(&mut self) -> Option<&mut [U]>
    where
	U: Cast
    {
	slice::from_raw_parts_mut::<T>(self as *mut T, 1)
	    .reslice_mut::<U>()
    }
}
