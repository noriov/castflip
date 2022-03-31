#[cfg(not(feature = "std"))] use core::option::Option::{self, Some, None};

use core::mem;

use crate::Cast;
use crate::experimental::Reslice;


///
/// Defines methods to convert a reference to a slice into a phantom
/// reference to a variable of the specified type without copying
/// data.
///
/// The antonym of [`Deslice`] is [`Enslice`].
///
/// [`Enslice`]: ./trait.Enslice.html
///
/// # Example
///
/// In the example below, method `deslice` converts the original
/// reference to bytes `bytes1` into a phantom reference to type
/// `ElfIdHdr` without copying data.
///
/// ```
/// # fn main() { test(); }
/// # fn test() -> Option<()> {
/// use castflip::experimental::Deslice;
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
/// let bytes1: [u8; 16] = [0x7F, 0x45, 0x4C, 0x46, 0x02, 0x01, 0x01, 0x00,
///                         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
///
/// unsafe {
///     // Convert the original reference to `bytes1` into `hdr`.
///     // Both &`bytes1` and `hdr` point to the same entity.
///     let hdr: &ElfIdHdr = bytes1.deslice()?;
///
///     // Check the results (hdr)
///     assert_eq!(&hdr.magic, b"\x7FELF"); // Magic Number: 7F 45 4C 46
///     assert_eq!(hdr.class, 2);           // File Class: 64-bit
///     assert_eq!(hdr.encoding, 1);        // Data Encoding: Little-Endian
///     assert_eq!(hdr.version, 1);         // Version 1
///     assert_eq!(hdr.os_abi, 0);          // (unspecified)
///     assert_eq!(hdr.abi_ver, 0);         // (unspecified)
///     assert_eq!(hdr.pad, [0_u8; 7]);     // Padding
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
/// When creating a phantom reference with this trait, take care with
/// the alignment issues.
///
pub trait Deslice {
    /// Converts a reference to the slice `self` of a type
    /// into a phantom reference to a variable of type `U`.
    unsafe fn deslice<U>(&self) -> Option<&U>
    where
	U: Cast;

    /// Converts a mutable reference to the slice `self` of a type
    /// into a mutable phantom reference to a variable of type `U`.
    unsafe fn deslice_mut<U>(&mut self) -> Option<&mut U>
    where
	U: Cast;
}


impl<T> Deslice for [T]
where
    T: Cast
{
    unsafe fn deslice<U>(&self) -> Option<&U>
    where
	U: Cast
    {
	if mem::size_of::<T>() * self.len() == mem::size_of::<U>() {
	    let slice1 = self.reslice::<U>()?;
	    Some(&slice1[0])
	} else {
	    None
	}
    }

    unsafe fn deslice_mut<U>(&mut self) -> Option<&mut U>
    where
	U: Cast
    {
	if mem::size_of::<T>() * self.len() == mem::size_of::<U>() {
	    let slice1 = self.reslice_mut::<U>()?;
	    Some(&mut slice1[0])
	} else {
	    None
	}
    }
}
