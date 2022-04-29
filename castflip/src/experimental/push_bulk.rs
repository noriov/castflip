#[cfg(feature = "alloc")] extern crate alloc;
#[cfg(feature = "alloc")] use alloc::vec::Vec;

use core::result::Result;
use core::slice;


///
/// Defines a method to extend a vector, with additional slots filled
/// by the specified closure.
///
/// Trait `PushBulk` would be useful when a number of extended slots
/// are filled with data read from a file or a stream because extended
/// slots are not initialized before the specified closure is called.
///
/// Trait `PushBulk` is used internally in this crate.
///
/// # Example
///
/// In the example below, method `push_bulk` fills a slice with data
/// read from `io::Cursor` and returns a `Result`.  As this example
/// shows, extended slots are passed to the closure as a slice.
///
/// ```
/// use std::io::{Cursor, Read};
/// use castflip::experimental::PushBulk;
///
/// // Input data
/// let bytes1: [u8; 4] = [0x10, 0x11, 0x12, 0x13];
/// let mut input1 = Cursor::new(bytes1);
///
/// // Initial data
/// let mut vec2 = vec![0x20, 0x21, 0x22];
///
/// // Read input data at the tail of `vec2`.
/// unsafe {
///     vec2.push_bulk(3, | buf | {
///         input1.read_exact(buf)
///     }).unwrap();
/// }
///
/// // Check the results (vec2)
/// assert_eq!(vec2, vec![0x20, 0x21, 0x22, 0x10, 0x11, 0x12]);
/// ```
///
/// # Safety
///
/// Because extended slots are not initialized before the specified
/// closure is called, the closure must fill whole slots.
///
pub trait PushBulk<T, R, E> {
    /// Extend a vector by `nelem` and call closure `fill_new_slice`
    /// to fill the extended slots.
    unsafe fn push_bulk<F>(&mut self, nelem: usize, fill_new_slice: F)
			   -> Result<R, E>
    where
	F: FnMut(&mut [T]) -> Result<R, E>;
}

impl<T, R, E> PushBulk<T, R, E> for Vec<T>
{
    unsafe fn push_bulk<F>(&mut self, nelem: usize, mut fill_new_slice: F)
			   -> Result<R, E>
    where
	F: FnMut(&mut [T]) -> Result<R, E>
    {
	// Prepare enough size of hidden area.
	self.reserve(nelem);

	// Fill hidden area with caller-supplied closure `fill_new_slice`.
	// The hidden area is passed as an ephemeral slice (soon dropped).
	let result = fill_new_slice(
	    slice::from_raw_parts_mut(
		self.as_mut_ptr().offset(self.len() as isize),
		nelem)
	);

	// If the result is ok, extend the length.
	if result.is_ok() {
	    self.set_len(self.len() + nelem);
	}

	result
    }
}
