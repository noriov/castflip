//
// This file defines trait `EncastMem`
//

use crate::{Cast, Endian, Flip, include_doc};
use core::{mem, ptr};

#[cfg(doc)]
use crate::BE;
#[cfg(feature = "alloc")]
use crate::experimental::PushBulk;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;


#[doc = include_doc!("trait_encast_mem.md")]
pub trait EncastMem {
    ///
    /// Encasts a byte representation of type `T` at the head of `self`
    /// as a value of type `T`.
    ///
    /// If successful, the resulting value is returned in [`Some`]`(T)`.
    /// On failure, [`None`] is returned.
    ///
    /// The endianness of the resulting value is the same as the
    /// endianness of the source bytes.  In typical cases, both are
    /// the native endianness.
    ///
    fn encast<T: Cast>(&self) -> Option<T>;

    ///
    /// Encasts a byte representation of type `T` at the head of `self`
    /// as a value of type `T`.
    ///
    /// If successful, the resulting value is returned in [`Some`]`(T)`.
    /// On failure, [`None`] is returned.
    ///
    /// The resulting value is in native-endian.  The endianness of
    /// the source bytes is specified by `endian`.
    ///
    fn encastf<T: Cast + Flip>(&self, endian: Endian) -> Option<T>;

    ///
    /// Encasts a byte representation of type `T` at the head of `self`
    /// as values of type `T`.
    ///
    /// If successful, the resulting values are saved in `slice` and
    /// the number of the source bytes is returned in [`Some`]`(usize)`.
    /// On failure, [`None`] is returned.
    ///
    /// The endianness of each resulting value is the same as the
    /// endianness of the corresponding source bytes.  In typical
    /// cases, all are the native endiannesses.
    ///
    fn encasts<T: Cast>(&self, slice: &mut [T]) -> Option<usize>;

    ///
    /// Encasts a byte representation of type `T` at the head of `self`
    /// as values of type `T`.
    ///
    /// If successful, the resulting values are saved in `slice` and
    /// the number of the source bytes is returned in [`Some`]`(usize)`.
    /// On failure, [`None`] is returned.
    ///
    /// The resulting values are in native-endian.  The endianness of
    /// the source bytes is specified by `endian`.
    ///
    fn encastsf<T: Cast + Flip>(
        &self,
        slice: &mut [T],
        endian: Endian,
    ) -> Option<usize>;

    ///
    /// Encasts a byte representation of type `T` at the head of `self`
    /// as values of type `T`.  The number of values in the source bytes
    /// is specified by `len`.
    ///
    /// If successful, the resulting values are returned in
    /// [`Some`]`(Vec<T>)`.  On failure, [`None`] is returned.
    ///
    /// The endianness of each resulting value is the same as the
    /// endianness of the corresponding source bytes.  In typical
    /// cases, all are the native endiannesses.
    ///
    #[cfg(feature = "alloc")]
    fn encastv<T: Cast>(&self, len: usize) -> Option<Vec<T>>;

    ///
    /// Encasts a byte representation of type `T` at the head of `self`
    /// as values of type `T`.  The number of values in the source bytes
    /// is specified by `len`.
    ///
    /// If successful, the resulting values are returned in
    /// [`Some`]`(Vec<T>)`.  On failure, [`None`] is returned.
    ///
    /// The resulting values are in native-endian.  The endianness of
    /// the source bytes is specified by `endian`.
    ///
    #[cfg(feature = "alloc")]
    fn encastvf<T: Cast + Flip>(
        &self,
        len: usize,
        endian: Endian,
    ) -> Option<Vec<T>>;
}


impl EncastMem for [u8] {
    #[inline]
    fn encast<T: Cast>(&self) -> Option<T> {
        if self.len() >= mem::size_of::<T>() {
            unsafe {
                // SAFETY: The following function call to `ptr::read_unaligned`
                // is safe because those types that implement trait Cast can be
                // duplicated simply by copying bits by the definition of trait
                // Cast.
                Some(ptr::read_unaligned::<T>(self.as_ptr() as *const T))
            }
        } else {
            None
        }
    }

    #[inline]
    fn encastf<T: Cast + Flip>(&self, endian: Endian) -> Option<T> {
        let mut value = self.encast::<T>()?;

        // Flips the endianness of the value in `value` if `endian` is
        // not equivalent to the endianness of the target system.
        value.flip_var(endian);

        Some(value)
    }

    #[inline]
    fn encasts<T: Cast>(&self, slice: &mut [T]) -> Option<usize> {
        let nbytes = mem::size_of_val(slice);

        if self.len() >= nbytes {
            unsafe {
                // SAFETY: The following function call to
                // `ptr::copy_nonoverlapping` is safe because those types that
                // implement trait Cast can be duplicated simply by copying
                // bits by the definition of trait Cast.
                ptr::copy_nonoverlapping::<u8>(
                    self.as_ptr(),
                    slice.as_mut_ptr() as *mut u8,
                    nbytes,
                );
            }
            Some(nbytes)
        } else {
            None
        }
    }

    #[inline]
    fn encastsf<T: Cast + Flip>(
        &self,
        slice: &mut [T],
        endian: Endian,
    ) -> Option<usize> {
        if !endian.need_swap() {
            // The endianness must not be reversed.
            self.encasts::<T>(slice)
        } else {
            // The endianness must be reversed.
            self.encastsf_swapped(slice)
        }
    }

    #[cfg(feature = "alloc")]
    fn encastv<T: Cast>(&self, len: usize) -> Option<Vec<T>> {
        let mut vec: Vec<T> = Vec::new();

        unsafe {
            // SAFETY: The following method call to `PushBulk::push_bulk` is
            // safe because the closure fills whole elements in `new_slice`.
            vec.push_bulk(len, |new_slice| {
                self.encasts(new_slice).ok_or(())
            }).ok()?;
        }

        Some(vec)
    }

    #[cfg(feature = "alloc")]
    #[inline]
    fn encastvf<T: Cast + Flip>(
        &self,
        len: usize,
        endian: Endian,
    ) -> Option<Vec<T>> {
        if !endian.need_swap() {
            // The endianness must not be reversed.
            self.encastv::<T>(len)
        } else {
            // The endianness must be reversed.
            self.encastvf_swapped(len)
        }
    }
}


///
/// Provides internal methods for trait `EncastMem`.
///
trait EncastMemInternal {
    ///
    /// Encasts a byte representation of type `T` at the head of
    /// `self` as a value of type `T`.
    ///
    /// If successful, the resulting value is returned in [`Some`]`(T)`.
    /// On failure, [`None`] is returned.
    ///
    /// The endianness of each resulting value is the swapped
    /// endianness of the corresponding source bytes.
    ///
    fn encastsf_swapped<T: Cast + Flip>(
        &self,
        slice: &mut [T],
    ) -> Option<usize>;

    ///
    /// Encasts a byte representation of type `T` at the head of
    /// `self` as values of type `T`.  The number of values in the
    /// source bytes is specified by `len`.
    ///
    /// If successful, the resulting values are returned in
    /// [`Some`]`(Vec<T>)`.  On failure, [`None`] is returned.
    ///
    /// The endianness of each resulting value is the swapped
    /// endianness of the corresponding source bytes.
    ///
    fn encastvf_swapped<T: Cast + Flip>(&self, len: usize) -> Option<Vec<T>>;
}

impl EncastMemInternal for [u8] {
    fn encastsf_swapped<T: Cast + Flip>(
        &self,
        slice: &mut [T],
    ) -> Option<usize> {
        if self.len() >= mem::size_of_val(slice) {
            let mut off = 0;

            // Read byte representations from `self`, reverse their
            // endiannesses, then save the resulting values to `elem`
            // in `slice`.
            for elem in slice {
                *elem = self[off ..].encast::<T>()?.flip_val_swapped();
                off += mem::size_of_val(elem);
            }

            Some(off)
        } else {
            None
        }
    }

    fn encastvf_swapped<T: Cast + Flip>(&self, len: usize) -> Option<Vec<T>> {
        let mut vec: Vec<T> = Vec::new();

        unsafe {
            // SAFETY: The following method call to `PushBulk::push_bulk` is
            // safe because the closure fills whole elements in `new_slice`.
            vec.push_bulk(len, |new_slice| {
                self.encastsf_swapped(new_slice).ok_or(())
            }).ok()?;
        }

        Some(vec)
    }
}
