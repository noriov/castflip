//
// This file defines trait `EncastIO`
//

use crate::{
    Cast, Endian, Flip, include_doc,
    experimental::{AsifBytes, PushBulk},
};
use core::mem::{self, MaybeUninit};
use std::io;

#[cfg(doc)]
use crate::BE;


#[doc = include_doc!("trait_encast_io.md")]
pub trait EncastIO {
    ///
    /// Encasts a byte representation of type `T` read from `self`
    /// using trait [`std::io::Read`] as a value of type `T`.
    ///
    /// If successful, the resulting value is returned in [`Ok`]`(T)`.
    /// On failure, an error value of struct [`std::io::Error`] is
    /// returned in [`Err`].
    ///
    /// The endianness of the resulting value is the same as the
    /// endianness of the source bytes.  In typical cases, both are
    /// the native endianness.
    ///
    fn encast<T: Cast>(&mut self) -> io::Result<T>;

    ///
    /// Encasts a byte representation of type `T` read from `self`
    /// using trait [`std::io::Read`] as a value of type `T`.
    ///
    /// If successful, the resulting value is returned in [`Ok`]`(T)`.
    /// On failure, an error value of struct [`std::io::Error`] is
    /// returned in [`Err`].
    ///
    /// The resulting value is in native-endian.  The endianness of
    /// the source bytes is specified by `endian`.
    ///
    fn encastf<T: Cast + Flip>(&mut self, endian: Endian) -> io::Result<T>;

    ///
    /// Encasts a byte representation of type `T` read from `self`
    /// using trait [`std::io::Read`] as a value of type `T`.
    ///
    /// If successful, the resulting values are saved in `slice` and
    /// the number of the source bytes is returned in [`Ok`]`(usize)`.
    /// On failure, an error value of struct [`std::io::Error`] is
    /// returned in [`Err`].
    ///
    /// The endianness of each resulting value is the same as the
    /// endianness of the corresponding source bytes.  In typical
    /// cases, all are the native endiannesses.
    ///
    fn encasts<T: Cast>(&mut self, slice: &mut [T]) -> io::Result<usize>;

    ///
    /// Encasts a byte representation of type `T` read from `self`
    /// using trait [`std::io::Read`] as a value of type `T`.
    ///
    /// If successful, the resulting values are saved in `slice` and
    /// the number of the source bytes is returned in [`Ok`]`(usize)`.
    /// On failure, an error value of struct [`std::io::Error`] is
    /// returned in [`Err`].
    ///
    /// The resulting values are in native-endian.  The endianness of
    /// the source bytes is specified by `endian`.
    ///
    fn encastsf<T: Cast + Flip>(
        &mut self,
        slice: &mut [T],
        endian: Endian,
    ) -> io::Result<usize>;

    ///
    /// Encasts a byte representation of type `T` read from `self`
    /// using trait [`std::io::Read`] as a value of type `T`.  The
    /// number of values in the source bytes is specified by `len`.
    ///
    /// If successful, the resulting values are returned in
    /// [`Ok`]`(Vec<T>)`.  On failure, an error value of struct
    /// [`std::io::Error`] is returned in [`Err`].
    ///
    /// The endianness of each resulting value is the same as the
    /// endianness of the corresponding source bytes.  In typical
    /// cases, all are the native endiannesses.
    ///
    fn encastv<T: Cast>(&mut self, len: usize) -> io::Result<Vec<T>>;

    ///
    /// Encasts a byte representation of type `T` read from `self`
    /// using trait [`std::io::Read`] as a value of type `T`.  The
    /// number of values in the source bytes is specified by `len`.
    ///
    /// If successful, the resulting values are returned in
    /// [`Ok`]`(Vec<T>)`.  On failure, an error value of struct
    /// [`std::io::Error`] is returned in [`Err`].
    ///
    /// The resulting value is in native-endian.  The endianness of
    /// the source bytes is specified by `endian`.
    ///
    fn encastvf<T: Cast + Flip>(
        &mut self,
        len: usize,
        endian: Endian,
    ) -> io::Result<Vec<T>>;
}


impl<R: ?Sized + io::Read> EncastIO for R {
    #[inline]
    fn encast<T: Cast>(&mut self) -> io::Result<T> {
        let mut value = MaybeUninit::<T>::uninit();

        unsafe {
            // SAFETY: The following function call to `io::Read::read_exact`
            // is safe because those types that implement trait Cast can be
            // duplicated simply by copying bits by the definition of trait
            // Cast.
            self.read_exact(value.asif_bytes_mut())?;

            // SAFETY: The following method call to `MaybeUninit::assume_init`
            // is safe because `value` has been initialized.
            Ok(value.assume_init())
        }
    }

    #[inline]
    fn encastf<T: Cast + Flip>(&mut self, endian: Endian) -> io::Result<T> {
        let mut value = self.encast::<T>()?;

        // Flips the endianness of the value in `value` if `endian` is
        // not equivalent to the endianness of the target system.
        value.flip_var(endian);

        Ok(value)
    }

    #[inline]
    fn encasts<T: Cast>(&mut self, slice: &mut [T]) -> io::Result<usize> {
        unsafe {
            // SAFETY: The following function call to `io::Read::read_exact`
            // is safe because those types that implement trait Cast can be
            // duplicated simply by copying bits by the definition of trait
            // Cast.
            self.read_exact(slice.asif_bytes_mut())?;
        }

        Ok(mem::size_of_val(slice))
    }

    #[inline]
    fn encastsf<T: Cast + Flip>(
        &mut self,
        slice: &mut [T],
        endian: Endian,
    ) -> io::Result<usize> {
        if !endian.need_swap() {
            // The endianness must not be reversed.
            self.encasts::<T>(slice)
        } else {
            // The endianness must be reversed.
            self.encastsf_swapped(slice)
        }
    }

    fn encastv<T: Cast>(&mut self, len: usize) -> io::Result<Vec<T>> {
        let mut vec: Vec<T> = Vec::new();

        unsafe {
            // SAFETY: The following method call to `PushBulk::push_bulk` is
            // safe because the closure fills whole elements in `new_slice`.
            vec.push_bulk(len, |new_slice| {
                self.encasts(new_slice)
            })?;
        }

        Ok(vec)
    }

    #[inline]
    fn encastvf<T: Cast + Flip>(
        &mut self,
        len: usize,
        endian: Endian,
    ) -> io::Result<Vec<T>> {
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
/// Provides internal methods for trait `EncastIO`.
///
trait EncastIOInternal {
    ///
    /// Encasts a byte representation of type `T` read from `self`
    /// using trait [`std::io::Read`] as a value of type `T`.
    ///
    /// If successful, the resulting values are saved in `slice` and
    /// the number of the source bytes is returned in [`Ok`]`(usize)`.
    /// On failure, an error value of struct [`std::io::Error`] is
    /// returned in [`Err`].
    ///
    /// The endianness of each resulting value is the swapped
    /// endianness of the corresponding source bytes.
    ///
    fn encastsf_swapped<T: Cast + Flip>(
        &mut self,
        slice: &mut [T],
    ) -> io::Result<usize>;

    ///
    /// Encasts a byte representation of type `T` read from `self`
    /// using trait [`std::io::Read`] as a value of type `T`.  The
    /// number of values in the source bytes is specified by `len`.
    ///
    /// If successful, the resulting values are returned in
    /// [`Ok`]`(Vec<T>)`.  On failure, an error value of struct
    /// [`std::io::Error`] is returned in [`Err`].
    ///
    /// The endianness of each resulting value is the swapped
    /// endianness of the corresponding source bytes.
    ///
    fn encastvf_swapped<T: Cast + Flip>(
        &mut self,
        len: usize,
    ) -> io::Result<Vec<T>>;
}

impl<R: ?Sized + io::Read> EncastIOInternal for R {
    fn encastsf_swapped<T: Cast + Flip>(
        &mut self,
        slice: &mut [T],
    ) -> io::Result<usize> {
        let nbytes = mem::size_of_val(slice);

        // Read values from `self`, reverse their endiannesses, then
        // save the resulting values to `elem` in `slice`.
        for elem in slice {
            *elem = self.encast::<T>()?.flip_val_swapped();
        }

        Ok(nbytes)
    }

    fn encastvf_swapped<T: Cast + Flip>(
        &mut self,
        len: usize,
    ) -> io::Result<Vec<T>> {
        let mut vec: Vec<T> = Vec::new();

        unsafe {
            // SAFETY: The following method call to `PushBulk::push_bulk` is
            // safe because the closure fills whole elements in `new_slice`.
            vec.push_bulk(len, |new_slice| {
                self.encastsf_swapped(new_slice)
            })?;
        }

        Ok(vec)
    }
}
