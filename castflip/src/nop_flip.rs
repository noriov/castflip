use crate::Flip;
#[cfg(doc)] use crate::LE;


///
/// Defines types whose values may not be `endian-flip`ped.
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
/// # Example 1
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
/// #   } else {
/// #       panic!();
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
/// manually, e.g. by using the methods provided by trait [`Flip`].
///
/// # Example 2
///
/// `#[derive(NopFlip)]` can be declared for a struct type in order to
/// keep the original value.
/// In the example below, a struct type containing a magic number
/// which indicates whether native-endian or swapped-endian is
/// declared as `NopFlip` to ease debugging.
///
/// ```
/// # fn main() { test(); }
/// # fn test() -> Option<()> {
/// use castflip::{Cast, Flip, NopFlip, Endian, EncastMem, DecastMem};
///
/// // struct fat_header and magic numbers are defined in <mach-o/fat.h> at
/// // /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/
///
/// #[repr(C)]
/// #[derive(Cast, NopFlip)]
/// struct FatMagic {
///     number: u32,
/// }
///
/// const FAT_MAGIC32: u32 = 0xCAFEBABE;  // Native Endian, 32-bit
/// const FAT_MAGIC64: u32 = 0xCAFEBABF;  // Native Endian, 64-bit
/// const FAT_CIGAM32: u32 = 0xBEBAFECA;  // Swapped Endian, 32-bit
/// const FAT_CIGAM64: u32 = 0xBFBAFECA;  // Swapped Endian, 64-bit
///
/// #[repr(C)]
/// #[derive(Cast, Flip)]
/// struct FatHeader {
///     magic:     FatMagic,
///     nfat_arch: u32,
/// }
///
/// let bytes1: [u8; 8] = [0xCA, 0xFE, 0xBA, 0xBE, 0x00, 0x00, 0x00, 0x02];
///
/// // Decode the first 4 bytes in `bytes1` without endian-flipping
/// // to determine the endianness of `FatHeader`.
/// // The generic type parameter can be inferred as u32
/// // because the types of the constants are u32.
/// let endian = match bytes1.encast()? {
///     FAT_MAGIC32 | FAT_MAGIC64 => Endian::Native,
///     FAT_CIGAM32 | FAT_CIGAM64 => Endian::Swapped,
///     _ => panic!(),
/// };
///
/// // Decode bytes `bytes1` to variable `fat_hdr2` of type `FatHeader`.
/// let fat_hdr2: FatHeader = bytes1.encastf(endian)?;
///
/// // Check the results (fat_hdr2)
/// if cfg!(target_endian = "little") {
///     assert_eq!(fat_hdr2.magic.number, FAT_CIGAM32); // Not endian-flipped.
/// } else if cfg!(target_endian = "big") {
///     assert_eq!(fat_hdr2.magic.number, FAT_MAGIC32); // Not endian-flipped.
/// # } else {
/// #   panic!();
/// }
/// assert_eq!(fat_hdr2.nfat_arch, 2);  // Endian-flipped.
/// # Some(())
/// # }
/// ```
///
/// The example above decodes the fat header located at the head of
/// [Mach-o].  It has two fields: magic number (4 bytes) and the
/// number of architectures in the file (4 bytes).
///
/// In the example above, at first, method `encast` decodes the first
/// 4 bytes in `bytes1` without endian-flipping in order to determine
/// the endianness of `FatHeader`.  Then, method `encastf` decodes the
/// first 8 bytes in `bytes1` to variable `fat_hdr2` of `FatHeader`
/// according to the determined endianness.
///
/// Note: [Mach-o] is a file format for executable files.
/// It is used by most systems based on the Mach kernel.
///
/// [Mach-o]: https://en.wikipedia.org/wiki/Mach-O
///
pub trait NopFlip: Flip {}
