use core::option::Option::{self, Some, None};
use core::{mem, slice};

use crate::Cast;


///
/// Defines methods to convert a reference to a slice of a type to a
/// phantom reference to a slice of the specified type without copying
/// data.
///
/// # Example
///
/// In the example below, references to three types of arrays are
/// converted among them.  It is possible because their binary form on
/// the memory are the same.
///
/// ```
/// # fn main() { test(); }
/// # fn test() -> Option<()> {
/// use castflip::experimental::Reslice;
/// use castflip::Cast;
///
/// #[repr(C)]
/// #[derive(Cast, PartialEq, Debug)]
/// struct Duo {
///     a: u16,
///     b: u16
/// }
///
/// #[repr(C)]
/// #[derive(Cast, PartialEq, Debug)]
/// struct Trio {
///     x: u16,
///     y: u16,
///     z: u16
/// }
///
/// // Input data.  They have different types but the same binary form.
/// let solo1: [u16; 6] = [ 100, 200, 300, 400, 500, 600 ];
/// let duo1 = [ Duo { a: 100, b: 200 },
///              Duo { a: 300, b: 400 },
///              Duo { a: 500, b: 600 } ];
/// let trio1 = [ Trio { x: 100, y: 200, z: 300 },
///               Trio { x: 400, y: 500, z: 600 } ];
///
/// unsafe {
///     // Convert the original reference to `duo1` into a phantom
///     // reference to &[u16] without copying data.
///     let duo_to_solo = duo1.reslice::<u16>()?;
///     assert_eq!(duo_to_solo, &solo1[..]);
///
///     // Convert the original reference to `solo1` into a phantom
///     // reference to &Duo without copying data.
///     let solo_to_duo = solo1.reslice::<Duo>()?;
///     assert_eq!(solo_to_duo, &duo1[..]);
/// }
///
/// unsafe {
///     // Convert the original reference to `trio1` into a phantom
///     // reference to &[u16] without copying data.
///     let trio_to_solo = trio1.reslice::<u16>()?;
///     assert_eq!(trio_to_solo, &solo1[..]);
///
///     // Convert the original reference to `solo1` into a phantom
///     // reference to &Trio without copying data.
///     let solo_to_trio = solo1.reslice::<Trio>()?;
///     assert_eq!(solo_to_trio, &trio1[..]);
/// }
///
/// unsafe {
///     // Convert the original reference to `duo1` into a phantom
///     // reference to &Trio without copying data.
///     let duo_to_trio = duo1.reslice::<Trio>()?;
///     assert_eq!(duo_to_trio, &trio1[..]);
///
///     // Convert the original reference to `trio1` into a phantom
///     // reference to &Duo without copying data.
///     let trio_to_duo = trio1.reslice::<Duo>()?;
///     assert_eq!(trio_to_duo, &duo1[..]);
/// }
/// # Some(())
/// # }
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
/// When creating a phantom reference with this trait, take care with
/// the alignment issues.
///
pub trait Reslice {
    /// Converts a reference to slice `self` of a type
    /// into a phantom reference to a slice of type `U`.
    unsafe fn reslice<U>(&self) -> Option<&[U]>
    where
	U: Cast;

    /// Converts a mutable reference to slice `self` of a type
    /// into a mutable phantom reference to a slice of type `U`.
    unsafe fn reslice_mut<U>(&mut self) -> Option<&mut [U]>
    where
	U: Cast;
}


impl<T> Reslice for [T]
where
    T: Cast
{
    unsafe fn reslice<U>(&self) -> Option<&[U]>
    where
	U: Cast
    {
	let slice_size = mem::size_of::<T>() * self.len();
	let new_len = slice_size / mem::size_of::<U>();

	#[allow(unused_parens)]
	if (slice_size == mem::size_of::<U>() * new_len &&
	    self.as_ptr().align_offset(mem::align_of::<U>()) == 0) {
	    Some(slice::from_raw_parts::<U>(self.as_ptr() as *const U,
					    new_len))
	} else {
	    None
	}
    }

    unsafe fn reslice_mut<U>(&mut self) -> Option<&mut [U]>
    where
	U: Cast
    {
	let slice_size = mem::size_of::<T>() * self.len();
	let new_len = slice_size / mem::size_of::<U>();

	#[allow(unused_parens)]
	if (slice_size == mem::size_of::<U>() * new_len &&
	    self.as_ptr().align_offset(mem::align_of::<U>()) == 0) {
	    Some(slice::from_raw_parts_mut::<U>(self.as_ptr() as *mut U,
						new_len))
	} else {
	    None
	}
    }
}
