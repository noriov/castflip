#[cfg(feature = "alloc")] extern crate alloc;
#[cfg(feature = "alloc")] use alloc::vec::Vec;

use core::result::Result;
use core::slice;


///
/// Defines a method to extend a vector whose additional slots are
/// filled by the specified closure.
///
/// Trait `PushBulk` would be useful when reading data from a file or
/// a stream into a vector, especially when the data size is not known
/// at compile time.
///
/// Method `push_bulk` reserves capacity for at least additional more
/// elements and calls a closure to fill the additional slots, then
/// extends the length of the vector to expose the additional
/// elements.  The callee closure receives additional elements as a
/// slice of uninitialized elements.  It must fill all elements to
/// avoid exposing uninitialized slots.
///
/// Method `push_bulk` returns the same result returned by the
/// closure.  If the closure returns `Err`(), the vector is not
/// extended.
///
/// In typical use cases, method `push_bulk` would be called for an
/// empty vector to fill with data.  In some cases, it might be called
/// repeatedly for the same vector to fill with a series of data
/// (e.g. due to some API limitations).
///
/// Trait `PushBulk` is used internally in this crate.
///
/// # Example
///
/// In the example below, method `push_bulk` reads data from
/// `io::Cursor`s into a vector.  Method `push_bulk` is called four
/// times as if method `read_exact` had some API limitations.  The
/// initial capacity of the vector is set to 16 to avoid unexpected
/// copying.
///
/// ```
/// use std::io::{Cursor, Read};
/// use castflip::experimental::PushBulk;
///
/// // Input data 1 (6 bytes)
/// let bytes1: [u8; 16] = [0x10, 0x11, 0x12, 0x13,
///                         0x20, 0x21, 0x22, 0x23,
///                         0x30, 0x31, 0x32, 0x33,
///                         0x40, 0x41, 0x42, 0x43];
/// let mut input1 = Cursor::new(bytes1);
///
/// // Prepare an empty vector `vec2` with capacity = 16.
/// let mut vec2 = Vec::with_capacity(16);
///
/// // Fill the vector with loaded data.
/// for i in 0 .. 4 {
///     unsafe {
///         vec2.push_bulk(4, | buf | {
///             input1.read_exact(buf)
///         }).unwrap();
///     }
/// }
///
/// // Check the result (vec2)
/// assert_eq!(&vec2, &bytes1[..])
/// ```
///
/// # Safety
///
/// The closure must fill whole slots because extended slots are not
/// initialized.
///
pub trait PushBulk<T, R, E> {
    /// Extends a vector by `additional` and calls closure
    /// `fill_new_slice` to fill the extended slots.
    unsafe fn push_bulk<F>(&mut self, additional: usize, fill_new_slice: F)
			   -> Result<R, E>
    where
	F: FnMut(&mut [T]) -> Result<R, E>;
}

impl<T, R, E> PushBulk<T, R, E> for Vec<T>
{
    unsafe fn push_bulk<F>(&mut self, additional: usize, mut fill_new_slice: F)
			   -> Result<R, E>
    where
	F: FnMut(&mut [T]) -> Result<R, E>
    {
	// Prepare enough size of hidden area.
	self.reserve(additional);

	// Fill hidden area with caller-supplied closure `fill_new_slice`.
	// The hidden area is passed as an ephemeral slice (soon dropped).
	let result = fill_new_slice(
	    slice::from_raw_parts_mut(
		self.as_mut_ptr().offset(self.len() as isize),
		additional)
	);

	// If the result is ok, extend the length.
	if result.is_ok() {
	    self.set_len(self.len() + additional);
	}

	result
    }
}
