///
/// Defines types whose values can be `encast`ed and `decast`ed.
///
/// Note: In this crate, the term `encast` means decoding a number of
/// bytes to one or more values, the term `decast` means encoding one
/// or more variables to a number of bytes, and the term `endian-flip`
/// means flipping the endianness of value(s).
///
/// # Description
///
/// A type is `Cast` if
/// - it is a numeric type, i.e.,
///   * i8, i16, i32, i64, i128, isize,
///   * u8, u16, u32, u64, u128, usize,
///   * f32, or f64,
/// - it is an array type whose element is `Cast`, or
/// - it is a struct type or a union type whose all members are `Cast`
///   and implements `Cast` by declaring `#[derive(Cast)]`.
///   (Note: `#[repr(C)]` must also be declared)
///
/// As you may have noticed, all types implementing trait `Cast` can
/// be duplicated simply by copying bits.  But trait `Cast` is not a
/// subtrait of trait `Copy`.  The reasons why trait `Cast` is defined
/// independently from trait `Copy` are (1) to exclude pointers, (2)
/// to avoid unexpected copying, and (3) to enjoy the benefits of
/// Rust's ownership system.  Therefore, if a struct type or a union
/// type need to be `Copy`, `#[derive(Clone, Copy)]` should also be
/// declared.
///
/// Trait `Cast` has no method.  It is purely defined as a trait
/// bound.
///
/// # Example
///
/// In the example below, `#[derive(Cast)]` makes the values of
/// `ElfIdHdr` `encast`able / `decast`able so that methods `encast`
/// and method `decast` can work.  Note that `#[repr(C)]` must also be
/// declared.
///
/// ```
/// # fn main() { test(); }
/// # fn test() -> Option<()> {
/// use castflip::{Cast, EncastMem, DecastMem};
///
/// #[repr(C)]
/// #[derive(Cast)]
/// struct ElfIdHdr {
///     magic:    [u8; 4],  //00-03: Magic Number 0x7F "ELF"
///     class:    u8,       //04   : File Class
///     encoding: u8,       //05   : Data Encoding
///     version:  u8,       //06   : Version (should be 1)
///     os_abi:   u8,       //07   : OS and ABI
///     abi_ver:  u8,       //08   : ABI Version
///     pad:      [u8; 7],  //09-0F: Padding (should be 0)
/// }
///
/// // Input data: ELF Identification (16 bytes)
/// let bytes1: [u8; 16] = [0x7F, 0x45, 0x4C, 0x46, 0x02, 0x01, 0x01, 0x00,
///                         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
///
/// // Decode bytes `bytes1` to variable `id_hdr2`.
/// let id_hdr2: ElfIdHdr = bytes1.encast()?;
///
/// // Encode the resulting `id_hdr2` to bytes `bytes3`.
/// let mut bytes3 = [0_u8; 16];
/// let size3 = bytes3.decast(&id_hdr2)?;
///
/// // Check the results (id_hdr2)
/// assert_eq!(id_hdr2.magic, *b"\x7FELF"); // Magic Number: 7F 45 4C 46
/// assert_eq!(id_hdr2.class, 2);           // File Class: 64-bit
/// assert_eq!(id_hdr2.encoding, 1);        // Encoding: Little-Endian
/// assert_eq!(id_hdr2.version, 1);         // Version 1
/// assert_eq!(id_hdr2.os_abi, 0);          // OS/ABI: unspecified
/// assert_eq!(id_hdr2.abi_ver, 0);         // ABI Ver: unspecified
/// assert_eq!(id_hdr2.pad, [0_u8; 7]);     // Padding
///
/// // Check the results (bytes3)
/// assert_eq!(size3, 16);
/// assert_eq!(bytes3, bytes1);
/// # Some(())
/// # }
/// ```
///
/// In the example above, method `encast` decodes bytes in `bytes1` to
/// variable `id_hdr` of type `ElfIdHdr`, and method `decast` encodes
/// the resulting value in `id_hdr` to bytes and stores them in
/// `bytes3`.  Their endiannesses are not flipped.
///
/// Note: [ELF] is a common standard file format for executable files.
/// ELF Identification is the first 16 bytes of the ELF header.
///
/// [ELF]: https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
///
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
