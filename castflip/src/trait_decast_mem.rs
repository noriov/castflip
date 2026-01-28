//
// This file defines trait `DecastMem`
//

use crate::{Cast, Endian, Flip, include_doc};
use core::{mem, ptr};

#[cfg(doc)]
use crate::BE;


#[doc = include_doc!("trait_decast_mem.md")]
pub trait DecastMem {
    ///
    /// Decasts a value of type `T` in `value` as a byte representation
    /// of type `T`.
    ///
    /// If successful, the resulting bytes are saved to the head of
    /// `self` and the number of the bytes is returned in
    /// [`Some`]`(usize)`.  On failure, [`None`] is returned.
    ///
    /// The endianness of the resulting bytes is the same as the
    /// endianness of the value in `value`.  In typical cases, both
    /// are the native endianness.
    ///
    fn decast<T: Cast>(&mut self, value: &T) -> Option<usize>;

    ///
    /// Decasts a value of type `T` in `value` as a byte representation
    /// of type `T`.
    ///
    /// If successful, the resulting bytes are saved to the head of
    /// `self` and the number of the bytes is returned in
    /// [`Some`]`(usize)`.  On failure, [`None`] is returned.
    ///
    /// The resulting bytes are in `endian` on the assumption that the
    /// value in `value` is in native-endian.
    ///
    fn decastf<T: Cast + Flip>(
        &mut self,
        value: &T,
        endian: Endian,
    ) -> Option<usize>;

    ///
    /// Decasts values of type `T` in `slice` as byte representations
    /// of type `T`.
    ///
    /// If successful, the resulting bytes are saved to the head of
    /// `self` and the number of the bytes is returned in
    /// [`Some`]`(usize)`.  On failure, [`None`] is returned.
    ///
    /// The endianness of the resulting bytes is the same as the
    /// endianness of the values in `slice`.  In typical cases, both
    /// are the native endianness.
    ///
    fn decasts<T: Cast>(&mut self, slice: &[T]) -> Option<usize>;

    ///
    /// Decasts values of type `T` in `slice` as byte representations
    /// of type `T`.
    ///
    /// If successful, the resulting bytes are saved to the head of
    /// `self` and the number of the bytes is returned in
    /// [`Some`]`(usize)`.  On failure, [`None`] is returned.
    ///
    /// The resulting bytes are in `endian` on the assumption that the
    /// value in `value` is in native-endian.
    ///
    fn decastsf<T: Cast + Flip>(
        &mut self,
        slice: &[T],
        endian: Endian,
    ) -> Option<usize>;

    ///
    /// Is equivalent to [`DecastMem::decasts`].
    ///
    /// This method will be deprecated in a future release.
    ///
    #[cfg(feature = "alloc")]
    fn decastv<T: Cast>(&mut self, slice: &[T]) -> Option<usize>;

    ///
    /// Is equivalent to [`DecastMem::decastsf`].
    ///
    /// This method will be deprecated in a future release.
    ///
    #[cfg(feature = "alloc")]
    fn decastvf<T: Cast + Flip>(
        &mut self,
        slice: &[T],
        endian: Endian,
    ) -> Option<usize>;
}


impl DecastMem for [u8] {
    #[inline]
    fn decast<T: Cast>(&mut self, value: &T) -> Option<usize> {
        let nbytes = mem::size_of_val(value);

        if self.len() >= nbytes {
            unsafe {
                // SAFETY: The following function call to
                // `ptr::copy_nonoverlapping` is safe because those types that
                // implement trait Cast can be duplicated simply by copying
                // bits by the definition of trait Cast.
                ptr::copy_nonoverlapping::<u8>(
                    value as *const T as *const u8,
                    self.as_mut_ptr(),
                    nbytes,
                );
            }
            Some(nbytes)
        } else {
            None
        }
    }

    #[inline]
    fn decastf<T: Cast + Flip>(
        &mut self,
        value: &T,
        endian: Endian,
    ) -> Option<usize> {
        if !endian.need_swap() {
            // The endianness must not be reversed.
            self.decast::<T>(value)
        } else {
            // The endianness must be reversed.
            self.decast::<T>(&value.flip_val_swapped())
        }
    }

    #[inline]
    fn decasts<T: Cast>(&mut self, slice: &[T]) -> Option<usize> {
        let nbytes = mem::size_of_val(slice);

        if self.len() >= nbytes {
            unsafe {
                // SAFETY: The following function call to
                // `ptr::copy_nonoverlapping` is safe because those types that
                // implement trait Cast can be duplicated simply by copying
                // bits by the definition of trait Cast.
                ptr::copy_nonoverlapping::<u8>(
                    slice.as_ptr() as *const u8,
                    self.as_mut_ptr(),
                    nbytes,
                );
            }
            Some(nbytes)
        } else {
            None
        }
    }

    #[inline]
    fn decastsf<T: Cast + Flip>(
        &mut self,
        slice: &[T],
        endian: Endian,
    ) -> Option<usize> {
        if !endian.need_swap() {
            // The endianness must not be reversed.
            self.decasts::<T>(slice)
        } else {
            // The endianness must be reversed.
            self.decastsf_swapped(slice)
        }
    }

    #[cfg(feature = "alloc")]
    #[inline]
    fn decastv<T: Cast>(&mut self, slice: &[T]) -> Option<usize> {
        // `decastv` is equivalent to `decasts`.
        self.decasts::<T>(slice)
    }

    #[cfg(feature = "alloc")]
    #[inline]
    fn decastvf<T: Cast + Flip>(
        &mut self,
        slice: &[T],
        endian: Endian,
    ) -> Option<usize> {
        // `decastvf` is equivalent to `decastsf`.
        self.decastsf::<T>(slice, endian)
    }
}


///
/// Provides internal methods for trait `DecastMem`.
///
trait DecastMemInternal {
    ///
    /// Decasts values in `slice` as their byte representations.
    ///
    /// If successful, the resulting bytes are saved to the head of
    /// `self` and the number of the bytes is returned in
    /// [`Some`]`(usize)`.  On failure, [`None`] is returned.
    ///
    /// The endianness of the resulting bytes is the swapped
    /// endianness of the value(s) in `self`.
    ///
    fn decastsf_swapped<T: Cast + Flip>(
        &mut self,
        slice: &[T],
    ) -> Option<usize>;
}

impl DecastMemInternal for [u8] {
    fn decastsf_swapped<T: Cast + Flip>(
        &mut self,
        slice: &[T],
    ) -> Option<usize> {
        if self.len() >= mem::size_of_val(slice) {
            let mut off = 0;

            for elem in slice {
                // Read values from `elem` in `slice`, reverse
                // their endiannesses, then save the resulting
                // byte representations to `self`.
                self[off ..].decast::<T>(&elem.flip_val_swapped())?;
                off += mem::size_of_val(elem);
            }

            Some(off)
        } else {
            None
        }
    }
}
