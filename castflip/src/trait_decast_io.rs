//
// This file defines trait `DecastIO`
//

use crate::{
    Cast, Endian, Flip, include_doc,
    experimental::AsifBytes,
};
use core::mem;
use std::io;

#[cfg(doc)]
use crate::BE;


#[doc = include_doc!("trait_decast_io.md")]
pub trait DecastIO {
    ///
    /// Decasts a value of type `T` in `value` as a byte representation
    /// of type `T`.
    ///
    /// If successful, the resulting bytes are written to `self` using
    /// trait [`std::io::Write`] and the number of the bytes is
    /// returned in [`Ok`]`(usize)`.  On failure, an error value is
    /// returned in [`Err`].
    ///
    /// The endianness of the resulting bytes is the same as the
    /// endianness of the value in `value`.  In typical cases, both
    /// are the native endianness.
    ///
    fn decast<T: Cast>(&mut self, value: &T) -> io::Result<usize>;

    ///
    /// Decasts a value of type `T` in `value` as a byte representation
    /// of type `T`.
    ///
    /// If successful, the resulting bytes are written to `self` using
    /// trait [`std::io::Write`] and the number of the bytes is
    /// returned in [`Ok`]`(usize)`.  On failure, an error value is
    /// returned in [`Err`].
    ///
    /// The resulting bytes are in `endian` on the assumption that the
    /// value in `value` is in native-endian.
    ///
    fn decastf<T: Cast + Flip>(
        &mut self,
        value: &T,
        endian: Endian,
    ) -> io::Result<usize>;

    ///
    /// Decasts values of type `T` in `slice` as byte representations
    /// of type `T`.
    ///
    /// If successful, the resulting bytes are written to `self` using
    /// trait [`std::io::Write`] and the number of the bytes is
    /// returned in [`Ok`]`(usize)`.  On failure, an error value is
    /// returned in [`Err`].
    ///
    /// The endianness of the resulting bytes is the same as the
    /// endianness of the values in `slice`.  In typical cases,
    /// both are the native endianness.
    ///
    fn decasts<T: Cast>(&mut self, slice: &[T]) -> io::Result<usize>;

    ///
    /// Decasts values of type `T` in `slice` as byte representations
    /// of type `T`.
    ///
    /// If successful, the resulting bytes are written to `self` using
    /// trait [`std::io::Write`] and the number of the bytes is
    /// returned in [`Ok`]`(usize)`.  On failure, an error value is
    /// returned in [`Err`].
    ///
    /// The resulting bytes are in `endian` on the assumption that the
    /// value in `value` is in native-endian.
    ///
    fn decastsf<T: Cast + Flip>(
        &mut self,
        slice: &[T],
        endian: Endian,
    ) -> io::Result<usize>;

    ///
    /// Is equivalent to [`DecastIO::decasts`].
    ///
    /// This method will be deprecated in a future release.
    ///
    fn decastv<T: Cast>(&mut self, slice: &[T]) -> io::Result<usize>;

    ///
    /// Is equivalent to [`DecastIO::decastsf`].
    ///
    /// This method will be deprecated in a future release.
    ///
    fn decastvf<T: Cast + Flip>(
        &mut self,
        slice: &[T],
        endian: Endian,
    ) -> io::Result<usize>;
}


impl<W: ?Sized + io::Write> DecastIO for W {
    #[inline]
    fn decast<T: Cast>(&mut self, value: &T) -> io::Result<usize> {
        unsafe {
            // SAFETY: The following function call to `io::Write::write_all`
            // is safe because those types that implement trait Cast can be
            // duplicated simply by copying bits by the definition of trait
            // Cast.
            self.write_all(value.asif_bytes_ref())?;
        }
        Ok(mem::size_of_val(value))
    }

    #[inline]
    fn decastf<T: Cast + Flip>(
        &mut self,
        value: &T,
        endian: Endian,
    ) -> io::Result<usize> {
        if !endian.need_swap() {
            // The endianness must not be reversed.
            self.decast::<T>(value)
        } else {
            // The endianness must be reversed.
            self.decast::<T>(&value.flip_val_swapped())
        }
    }

    #[inline]
    fn decasts<T: Cast>(&mut self, slice: &[T]) -> io::Result<usize> {
        unsafe {
            // SAFETY: The following function call to `io::Write::write_all`
            // is safe because those types that implement trait Cast can be
            // duplicated simply by copying bits by the definition of trait
            // Cast.
            self.write_all(slice.asif_bytes_ref())?;
        }
        Ok(mem::size_of_val(slice))
    }

    #[inline]
    fn decastsf<T: Cast + Flip>(
        &mut self,
        slice: &[T],
        endian: Endian,
    ) -> io::Result<usize> {
        if !endian.need_swap() {
            // The endianness must not be reversed.
            self.decasts::<T>(slice)
        } else {
            // The endianness must be reversed.
            self.decastsf_swapped(slice)
        }
    }

    #[inline]
    fn decastv<T: Cast>(&mut self, slice: &[T]) -> io::Result<usize> {
        // `decastv` is equivalent to `decasts`.
        self.decasts(slice)
    }

    #[inline]
    fn decastvf<T: Cast + Flip>(
        &mut self,
        slice: &[T],
        endian: Endian,
    ) -> io::Result<usize> {
        // `decastvf` is equivalent to `decastsf`.
        self.decastsf(slice, endian)
    }
}


///
/// Provides internal methods for trait `DecastIO`.
///
trait DecastIOInternal {
    /// Writes the memory representation of the values in `slice` to a
    /// writer `self` and returns the number of output bytes in
    /// `Ok(usize)`.
    ///
    /// The endianness of the output bytes is the swapped one of the
    /// endianness of `slice`.
    ///
    /// If an error is detected, `Err(io::Error)` is returned.
    fn decastsf_swapped<T: Cast + Flip>(
        &mut self,
        slice: &[T],
    ) -> io::Result<usize>;
}

impl<W: ?Sized + io::Write> DecastIOInternal for W {
    fn decastsf_swapped<T: Cast + Flip>(
        &mut self,
        slice: &[T],
    ) -> io::Result<usize> {
        for elem in slice {
            // Read values from `slice`, reverse their endiannesses, then
            // write the resulting byte representations to `self`.
            self.decast::<T>(&elem.flip_val_swapped())?;
        }

        Ok(mem::size_of_val(slice))
    }
}
