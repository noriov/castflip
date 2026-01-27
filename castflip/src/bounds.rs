//
// This file defines three traits: `Cast`, `Flip` and `NopFlip`.
// They act as trait bounds in the generic type programming.
//

use crate::{Endian, include_doc};
use core::{
    mem::{self, MaybeUninit},
    ptr,
};

#[cfg(doc)]
use crate::{BE, EncastMem, LE};


//
// Trait `Cast`
//
#[doc = include_doc!("bound_cast.md")]
pub trait Cast: Sized {}

impl Cast for i8 {}
impl Cast for i16 {}
impl Cast for i32 {}
impl Cast for i64 {}
impl Cast for i128 {}
impl Cast for isize {}

impl Cast for u8 {}
impl Cast for u16 {}
impl Cast for u32 {}
impl Cast for u64 {}
impl Cast for u128 {}
impl Cast for usize {}

impl Cast for f32 {}
impl Cast for f64 {}

impl<T: Cast, const N: usize> Cast for [T; N] {}


//
// Trait `Flip`
//
#[doc = include_doc!("bound_flip.md")]
pub trait Flip: Sized {
    ///
    /// Returns the value in `self` with its endianness reversed.
    ///
    fn flip_val_swapped(&self) -> Self;

    ///
    /// Reads the value in `self`, flips the endianness of the value
    /// if `endian` is not equivalent to the endianness of the target
    /// system, then returns the resulting value.
    ///
    #[inline]
    fn flip_val(&self, endian: Endian) -> Self {
        if !endian.need_swap() {
            unsafe {
                // SAFETY: The following function call to `ptr::read` is safe
                // because those types that implement trait Flip can be
                // duplicated simply by copying bits by the definition of
                // trait Flip.
                ptr::read(self)
            }
        } else {
            self.flip_val_swapped()
        }
    }

    ///
    /// Reverses the endianness of the value in `self`.
    ///
    #[inline]
    fn flip_var_swapped(&mut self) {
        *self = self.flip_val_swapped();
    }

    ///
    /// Flips the endianness of the value in `self` if `endian` is not
    /// equivalent to the endianness of the target system.
    ///
    #[inline]
    fn flip_var(&mut self, endian: Endian) {
        if endian.need_swap() {
            self.flip_var_swapped();
        }
    }
}


macro_rules! impl_flip_for_int {
    ( $( $ty:ty ),* ) => { $(
        impl Flip for $ty {
            #[inline]
            fn flip_val_swapped(&self) -> Self {
                self.swap_bytes()
            }
        }
    )* }
}

macro_rules! impl_flip_for_float {
    ( $( $ty:ty ),* ) => { $(
        impl Flip for $ty {
            #[inline]
            fn flip_val_swapped(&self) -> Self {
                <$ty>::from_bits(self.to_bits().swap_bytes())
            }
        }
    )* }
}

impl_flip_for_int!(i8, i16, i32, i64, i128, isize);
impl_flip_for_int!(u8, u16, u32, u64, u128, usize);
impl_flip_for_float!(f32, f64);


impl<T: Flip, const N: usize> Flip for [T; N] {
    #[inline]
    fn flip_val_swapped(&self) -> Self {
        let mut array = MaybeUninit::<[T; N]>::uninit();

        if mem::size_of_val(&array) > 0 {
            unsafe {
                // SAFETY: The following function call to `mem::transmute`
                // is safe because the memory layouts of [T; N] and
                // MaybeUninit::<[T; N]> are exactly the same.
                let array2: &mut [T; N] = mem::transmute(&mut array);

                // NOTE: All elements in `array2` (=`array`) is initialized.
                for i in 0 .. N {
                    array2[i] = self[i].flip_val_swapped();
                }
            }
        }

        unsafe {
            // SAFETY: The following method call to `MaybeUninit::assume_init`
            // is safe because all elements in `array` have been initialized.
            array.assume_init()
        }
    }

    #[inline]
    fn flip_var_swapped(&mut self) {
        for elem in self {
            elem.flip_var_swapped();
        }
    }
}


//
// Trait `NopFlip`
//
#[doc = include_doc!("bound_nop_flip.md")]
pub trait NopFlip: Flip {}
