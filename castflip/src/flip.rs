use core::ptr;

use crate::Endian;


///
/// Declares types whose values can be `endian-flip`ped.
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
/// Some methods of `Flip` is used internally in this crate.
///
/// # Example 1
///
/// In the example below, #[derive(`Flip`)] makes the value of
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
/// let bytes3_size = bytes3.decastf(&udp_hdr2, BE)?;
///
/// // Check the results.
/// assert_eq!(udp_hdr2.sport, 0xC3C9); // = 50121
/// assert_eq!(udp_hdr2.dport, 0x0035); // = 53 (DNS)
/// assert_eq!(udp_hdr2.len,   0x0032); // = 50
/// assert_eq!(udp_hdr2.sum,   0x823F);
/// assert_eq!(bytes3_size, 8);
/// assert_eq!(bytes3, bytes1);
/// # Some(())
/// # }
/// ```
///
/// In the example above, method `encastf` decodes bytes in `bytes1`
/// in big-endian (`BE`) to variable `udp_hdr2` of type `UdpHdr`.
/// Then, method `decastf` encodes the resulting value in `udp_hdr2`
/// to bytes in big-endian (`BE`) and stores them in `bytes3`.
///
/// Note: [UDP] is one of the fundamental protocols in the internet
/// protocol suite.
///
/// [UDP]: https://en.wikipedia.org/wiki/User_Datagram_Protocol
///
/// # Example 2
///
/// In the example below, two values and one variable are
/// `endian-flip`ped by using methods defined in `Flip`.
///
/// ```
/// # fn main() { test(); }
/// # fn test() -> Option<()> {
/// use castflip::{Flip, SE};
///
/// // Get the endian-flipped value of `val1`.
/// let val1 = 0x1234_u16;
/// let val2 = val1.flip_val(SE);  // SE = Swapped-Endian
/// assert_eq!(val2, 0x3412);
///
/// // Flip the endianness of variable `var3`.
/// let mut var3 = 0x5678_u16;
/// var3.flip_var(SE);
/// assert_eq!(var3, 0x7856);
///
/// // Get the endian-flipped value of an integer.
/// let val4 = 0xABCD_u16.flip_val(SE);
/// assert_eq!(val4, 0xCDAB);
/// # Some(())
/// # }
/// ```
///
/// In the example above, method `flip_val` returns the `endian-flip`ped
/// value of `self` (e.g., `val1` and `0xABCD_u16`), and method `flip_var`
/// flips the endianness of `self` (e.g., `val3`).
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
	let mut vec = Vec::new();
	for i in 0 .. N {
	    vec.push(self[i].flip_val_swapped());
	}

	match vec.try_into() {
	    Ok(array) => array,
	    Err(_e) => panic!(),
	}
    }
}


///
/// Declares types whose values may not be `endian-flip`ped.
///
/// Note: In this crate, the term `encast` means decoding a number of
/// bytes to one or more values, the term `decast` means encoding one
/// or more variables to a number of bytes, and the term `endian-flip`
/// means flipping the endianness of value(s).
///
/// # Description
///
/// A struct type may contain a union type.  A union type can be
/// `encast`ed and `decast`ed but cannot be automatically
/// `endian-flip`ped.  In order to automatically flip the endianness
/// of such struct type without `endian-flip`ping its internal union
/// type, `NopFlip` is provided.
///
/// `NopFlip` is a subtrait of [`Flip`].  It has the same methods with
/// [`Flip`] but all methods do nothing.  In the above example, if
/// `NopFlip` is implemented for the union type, `Flip` can be
/// implemented for the struct type.
///
/// In general, `NopFlip` can be implemented by declaring
/// `#[derive(NopFlip)]` for a struct type or a union type whose all
/// members implement `Flip`.  Types implementing `NopFlip` can be
/// used as a member of a struct type or a union type implementing
/// `Flip`.
///
/// # Example
///
/// In the example below, #[derive(`NopFlip`)] marks `UnionU` as
/// `endian-flip`pable but the implemented methods do nothing (Nop,
/// i.e., No operation) so that method `encastf` and method `decastf`
/// do not flip its endianness even if in a case where it is a member
/// of another struct type `TypeC` which implements `Flip`.
///
/// ```
/// # use std::io::Result;
/// # fn main() { test(); }
/// # fn test() -> Result<()> {
/// use std::io::Cursor;
/// use castflip::{Cast, Flip, NopFlip, EncastIO, DecastIO, LE};
///
/// #[repr(C)]
/// #[derive(Cast, Flip)]
/// struct TypeA {      // 8 bytes (total)
///     x: [u8; 2],     // 2 bytes
///     y: u16,         // 2 bytes
///     z: u32,         // 4 bytes
/// }
///
/// #[repr(C)]
/// #[derive(Cast, NopFlip)]
/// union UnionB {      // 4 bytes (largest)
///     u: [u8; 4],     // 4 bytes
///     v: [u16; 2],    // 4 bytes
///     w: u32,         // 4 bytes
/// }
///
/// #[repr(C)]
/// #[derive(Cast, Flip)]
/// struct TypeC {      // 16 bytes (total)
///     a: TypeA,       //  8 bytes
///     b: UnionB,      //  4 bytes
///     f: f32,         //  4 bytes
/// }
///
/// // Input data (16 bytes)
/// let bytes1: [u8; 16] = [0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
///                         0x18, 0x19, 0x1A, 0x1B, 0x00, 0x00, 0x48, 0x41];
/// let mut input1 = Cursor::new(bytes1);
///
/// // Decode input `input1` to variable `var2_c` of type `TypeC`.
/// let var2_c: TypeC = input1.encastf(LE)?;  // LE = Little-Endian
///
/// // Encode variable `var2_c` to bytes and write them to `output3`.
/// let mut output3 = Cursor::new(vec![0_u8; 16]);
/// output3.decastf(&var2_c, LE)?;
///
/// // Check the results (TypeA in TypeC)
/// assert_eq!(var2_c.a.x, [0x10_u8, 0x11]);
/// assert_eq!(var2_c.a.y, 0x1312);
/// assert_eq!(var2_c.a.z, 0x17161514);
///
/// // Check the results (UnionB in TypeC)
/// unsafe {
///     assert_eq!(var2_c.b.u, [0x18_u8, 0x19, 0x1A, 0x1B]);
///     if cfg!(target_endian = "little") {
///         assert_eq!(var2_c.b.v, [0x1918_u16, 0x1B1A]);
///         assert_eq!(var2_c.b.w, 0x1B1A1918);
///     } else if cfg!(target_endian = "big") {
///         assert_eq!(var2_c.b.v, [0x1819_u16, 0x1A1B]);
///         assert_eq!(var2_c.b.w, 0x18191A1B);
///     }
/// }
///
/// // Check the result (f32 in TypeC)
/// assert_eq!(var2_c.f, 12.5_f32);
///
/// // Check the result (output3)
/// assert_eq!(&output3.into_inner(), &bytes1[..]);
/// # Ok(())
/// # }
/// ```
///
/// In the example above, method `encastf` decodes bytes in `bytes1`
/// in little-endian (`LE`) to variable `var2_c` of type `TypeC`.
/// Then, method `decastf` encodes the resulting value in `var2_c` to
/// bytes in little-endian (`LE`) and stores them in `bytes3`.
///
/// As the results show, field `a` and `f` of `TypeC` are
/// `endian-flip`ped, but field `b` is not `endian-flip`ped.
/// The endianness of the value(s) in field `b` needs to be
/// flipped manually, e.g., by using the methods of [`Flip`].
///
pub trait NopFlip: Flip {}
