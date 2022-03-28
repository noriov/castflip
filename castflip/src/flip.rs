use core::marker::Sized;
use core::mem::MaybeUninit;
use core::ptr;

use crate::Endian;
#[cfg(doc)] use crate::{Cast, NopFlip, LE, BE};


///
/// Defines types whose values can be `endian-flip`ped.
///
/// Note: In this crate, the term `encast` means decoding a number of
/// bytes to one or more values, the term `decast` means encoding one
/// or more variables to a number of bytes, and the term `endian-flip`
/// means flipping the endianness of value(s).
///
/// # Description
///
/// A type is `Flip` if
/// - it is a numeric type, i.e.,
///   * i8, i16, i32, i64, i128, isize,
///   * u8, u16, u32, u64, u128, usize,
///   * f32, or f64,
/// - it is an array type whose element is `Flip`, or
/// - it is a struct type whose all members are `Flip` and
///   implements `Flip` by declaring `#[derive(Flip)]`.
///
/// As you may have noticed, the types that trait `Flip` can be
/// implemented for is similar to the types that trait [`Cast`] can be
/// implemented for.  The difference is that trait `Flip` cannot be
/// implemented for a union type by declaring `#[derive(Flip)]`, while
/// trait [`Cast`] can be implemented for a union type by declaring
/// `#[derive(`[`Cast`]`)]`.  Because there is no automatic way to
/// flip the endianness of a union type, `#[derive(Flip)]` does not
/// support a union type.  This is the reason why trait `Flip` is
/// defined independently from trait [`Cast`].
///
/// FYI: There would be a need to flip the endianness of a struct type
/// containing a union type.  In such a case, those members whose types
/// are not union types should be automatically `endian-flip`ped,
/// while those members whose types are union types must be manually
/// `endian-flip`ped.  In such a situation, `#[derive([`[`NopFlip`]`)]`
/// would be a help.  It nominally implements trait `Flip` whose
/// methods do nothing (Nop = No operation).  See the description of
/// trait [`NopFlip`] for more information.
///
/// # Example 1
///
/// In the example below, `#[derive(Flip)]` makes the value of
/// `UdpHdr` `endian-flip`pable so that method `encastf` and method
/// `decastf` can flip the endianness of their results.
///
/// ```
/// # fn main() { test(); }
/// # fn test() -> Option<()> {
/// use castflip::{Cast, Flip, EncastMem, DecastMem, BE};
///
/// #[repr(C)]
/// #[derive(Cast, Flip)]
/// struct UdpHdr {     // UDP: https://www.rfc-editor.org/rfc/rfc768.txt
///     sport:  u16,    // UDP Source Port
///     dport:  u16,    // UDP Destination Port
///     len:    u16,    // UDP Length
///     sum:    u16,    // UDP Checksum
/// }
///
/// // Input data: UDP header (8 bytes)
/// let bytes1: [u8; 8] = [0xC3, 0xC9, 0x00, 0x35, 0x00, 0x32, 0x82, 0x3F];
///
/// // Decode bytes `bytes1` to variable `udp_hdr2`.
/// let udp_hdr2: UdpHdr = bytes1.encastf(BE)?;  // BE = Big-Endian
///
/// // Encode the resulting UDP header `udp_hdr2` to bytes `bytes3`.
/// let mut bytes3 = [0_u8; 8];
/// let size3 = bytes3.decastf(&udp_hdr2, BE)?;
///
/// // Check the results (udp_hdr2)
/// assert_eq!(udp_hdr2.sport, 0xC3C9); // = 50121
/// assert_eq!(udp_hdr2.dport, 0x0035); // = 53 (DNS)
/// assert_eq!(udp_hdr2.len,   0x0032); // = 50
/// assert_eq!(udp_hdr2.sum,   0x823F);
///
/// // Check the results (bytes3)
/// assert_eq!(size3, 8);
/// assert_eq!(bytes3, bytes1);
/// # Some(())
/// # }
/// ```
///
/// In the example above, method `encastf` decodes bytes in `bytes1`
/// in big-endian ([`BE`]) to variable `udp_hdr2` of type `UdpHdr`.
/// Then, method `decastf` encodes the resulting value in `udp_hdr2`
/// to bytes in big-endian ([`BE`]) and stores them in `bytes3`.
///
/// Note: [UDP] is one of the fundamental protocols in the internet
/// protocol suite.
///
/// [UDP]: https://en.wikipedia.org/wiki/User_Datagram_Protocol
///
/// # Example 2
///
/// Trait `Flip` has two types of methods.  Typically, they are called
/// automatically inside this crate.  However, users may need to call
/// them explicitly in certain situations, e.g. to flip the endianness
/// of union types.  The example below shows how to use the methods of
/// trait `Flip`.
///
/// ```
/// # fn main() { test(); }
/// # fn test() -> Option<()> {
/// use castflip::{Flip, LE, BE};
///
/// // Get the endian-flipped value of `val1`.
/// let val1 = 0x1234_u16;
/// let val2 = val1.flip_val(LE);  // LE = Little-Endian
///
/// // Get the endian-flipped value of an integer.
/// let val3 = 0x5678_u16.flip_val(BE);  // BE = Big-Endian
///
/// // Flip the endianness of variable `var4`.
/// let mut var4 = 0xABCD_u16;
/// var4.flip_var(LE);  // LE = Little-Endian
///
/// if cfg!(target_endian = "little") {
///     assert_eq!(val2, 0x1234);
///     assert_eq!(val3, 0x7856);  // Swapped
///     assert_eq!(var4, 0xABCD);
/// } else if cfg!(target_endian = "big") {
///     assert_eq!(val2, 0x3412);  // Swapped
///     assert_eq!(val3, 0x5678);
///     assert_eq!(var4, 0xCDAB);  // Swapped
/// # } else {
/// #   panic!();
/// }
/// # Some(())
/// # }
/// ```
///
/// In the example above, method `flip_val` returns a value.  If the
/// specified `endian` is the same with the endianness of the target
/// system, it returns exactly the same value with `self`.  Otherwise,
/// it returns the `endian-flip`ped value of `self`.  In contrast,
/// method `flip_var` flips the endianness of variable `self` if the
/// specified `endian` is different from the endianness of the target
/// system.  Here, [`LE`] is an alias of [`Endian`]`::Little`, which
/// means Little-Endian.  [`BE`] is an alias of [`Endian`]`::Big`,
/// which means Big-Endian.
///
pub trait Flip: Sized {
    /// Returns the endian-flipped value of `self`.
    fn flip_val_swapped(&self) -> Self;

    /// Returns the endian-flipped value of `self` if `endian` is
    /// different from the endianness of the target system.
    /// Otherwise, returns the same value with `self`.
    fn flip_val(&self, endian: Endian) -> Self {
	if !endian.need_swap() {
	    unsafe {
		ptr::read(self)
	    }
	} else {
	    self.flip_val_swapped()
	}
    }

    /// Flips the endianness of the variable (`self`).
    fn flip_var_swapped(&mut self) {
	*self = self.flip_val_swapped();
    }

    /// Flips the endianness of the variable (`self`) if `endian` is
    /// different from the endianness of the target system.
    fn flip_var(&mut self, endian: Endian) {
	if endian.need_swap() {
	    self.flip_var_swapped();
	}
    }
}


macro_rules! impl_flip_for_int {
    ( $( $ty:ty ),* ) => {
	$(
	    impl Flip for $ty {
		fn flip_val_swapped(&self) -> Self {
		    self.swap_bytes()
		}
	    }
	)*
    }
}

macro_rules! impl_flip_for_float {
    ( $( $ty:ty ),* ) => {
	$(
	    impl Flip for $ty {
		fn flip_val_swapped(&self) -> Self {
		    <$ty>::from_bits(self.to_bits().swap_bytes())
		}
	    }
	)*
    }
}

impl_flip_for_int!(u8, u16, u32, u64, u128, usize,
		   i8, i16, i32, i64, i128, isize);
impl_flip_for_float!(f32, f64);


impl<T: Flip, const N: usize> Flip for [T; N] {
    fn flip_val_swapped(&self) -> Self
    {
	unsafe {
	    let mut array: [MaybeUninit<T>; N] =
		MaybeUninit::uninit().assume_init();
	    for i in 0 .. N {
		array[i] = MaybeUninit::new(self[i].flip_val_swapped());
	    }
	    ptr::read(array.as_ptr() as *const [T; N])
	}
    }
}
