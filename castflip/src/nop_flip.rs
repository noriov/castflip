use crate::Flip;
#[cfg(doc)] use crate::LE;


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
/// A struct type may contain a union type as its member.  A union
/// type can be `encast`ed and `decast`ed but cannot be automatically
/// `endian-flip`ped.  In order to automatically flip the endianness
/// of such container struct type without flipping the endianness of
/// its internal union type, `#[derive(NopFlip)]` is provided.
///
/// `#[derive(NopFlip)]` can be declared for a struct type or a union
/// type whose all members implement [`Flip`].  It nominally
/// implements trait [`Flip`] whose methods do nothing (Nop = No
/// operation).  It also implements trait `NopFlip`.
///
/// Trait `NopFlip` has no method.  It is purely defined as a trait
/// bound.
///
/// # Example
///
/// In the example below, `#[derive(NopFlip)]` nominally implements
/// trait [`Flip`] whose methods do nothing (Nop = No operation) for
/// `UnionB` so that method `encastf` and method `decastf` can flip
/// the endianness of the container struct `StructC` except its
/// internal union `UnionB`.
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
/// struct StructA {    // 8 bytes (total)
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
/// struct StructC {    // 16 bytes (total)
///     a: StructA,     //  8 bytes
///     b: UnionB,      //  4 bytes
///     f: f32,         //  4 bytes
/// }
///
/// // Input data (16 bytes)
/// let bytes1: [u8; 16] = [0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
///                         0x18, 0x19, 0x1A, 0x1B, 0x00, 0x00, 0x48, 0x41];
/// let mut input1 = Cursor::new(bytes1);
///
/// // Decode input `input1` to variable `var2_c` of type `StructC`.
/// let var2_c: StructC = input1.encastf(LE)?;  // LE = Little-Endian
///
/// // Encode variable `var2_c` to bytes and write them to `output3`.
/// let mut output3 = Cursor::new(vec![0_u8; 16]);
/// let size3 = output3.decastf(&var2_c, LE)?;
///
/// // Check the results (StructA in StructC)
/// assert_eq!(var2_c.a.x, [0x10_u8, 0x11]);
/// assert_eq!(var2_c.a.y, 0x1312);
/// assert_eq!(var2_c.a.z, 0x17161514);
///
/// // Check the results (UnionB in StructC)
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
/// // Check the result (f32 in StructC)
/// assert_eq!(var2_c.f, 12.5_f32);
///
/// // Check the result (output3)
/// assert_eq!(size3, 16);
/// assert_eq!(&output3.into_inner(), &bytes1[..]);
/// # Ok(())
/// # }
/// ```
///
/// In the example above, method `encastf` decodes bytes in `bytes1`
/// in little-endian ([`LE`]) to variable `var2_c` of type `StructC`.
/// Then, method `decastf` encodes the resulting value in `var2_c` to
/// bytes in little-endian ([`LE`]) and stores them in `bytes3`.
///
/// As the results show, field `a` and `f` of `StructC` are
/// `endian-flip`ped, but field `b` is not `endian-flip`ped.  The
/// endianness of the value(s) in field `b` needs to be flipped
/// manually, e.g., by using the methods provided by trait [`Flip`].
///
pub trait NopFlip: Flip {}
