Types that implement trait [`Flip`] whose methods do nothing.

# Description

Those types that implement trait [`NopFlip`] also implement trait
[`Flip`] whose methods do nothing.  Note that NOP stands for "No
OPeration".

It is implemented for

- `struct` types and `union` types whose all fields' types implement
  trait [`Flip`] and whose type definitions are annotated with
  attribute `#[`[`derive(NopFlip)`]`]`[^NopFlip].

[^NopFlip]: The types annotated with attribute `#[`[`derive(NopFlip)`]`]`
implement both trait [`Flip`] with NOP (No OPeration) methods and
marker trait [`NopFlip`].

Trait [`NopFlip`] has no method.  It is a subtrait of trait [`Flip`].
It is defined as a marker.

# Safety

When you implement trait [`NopFlip`] for a type, you also need to
implement trait [`Flip`] whose methods do nothing for the type.  But
for safety reasons, you must not implement trait [`Flip`] manually
unless you know what you are doing.

The recommended way to implement both trait [`Flip`] whose methods do
nothing and trait [`NopFlip`] for a type is to apply attribute
`#[`[`derive(NopFlip)`]`]` to the type.

Note that the internal specification of attribute
`#[`[`derive(NopFlip)`]`]` may change in a future release.

# Background

Because there is no common way to flip the endianness of a value of a
`union` type, attribute `#[`[`derive(Flip)`]`]` does not support a
`union` type.  Therefore, if a `struct` type contains a field of a
`union` type, attribute `#[`[`derive(Flip)`]`]` cannot be applied to
the `struct` type without implementing trait [`Flip`] with custom-made
methods to the `union` type.

Attribute `#[`[`derive(NopFlip)`]`]` is introduced to simplify such
manual implementations.  By applying attribute
`#[`[`derive(NopFlip)`]`]` to a `struct` type or a `union` type, trait
[`Flip`] whose methods do nothing is implemented for the type so that
the type can be a member of a `struct` type that implement trait
[`Flip`].  You should manually flip the endianness of the value or the
byte representation of the type as required when encasting or
decasting.  By applying the attribute, marker trait [`NopFlip`] is
also implemented for the type.

Attribute `#[`[`derive(NopFlip)`]`]` is also useful for a `struct`
type.  For example, the [Mach-O] header and the [PCAP] format have a
multi-byte field to indicate both the endianness and the file type of
their files.  In such cases, it is often simple to read and write the
value of such multi-byte field without flipping its endianness.  By
defining a single-field `struct` type containing such multi-byte field
and applying attribute `#[`[`derive(NopFlip)`]`]` to the type, the
endianness of such multi-byte field is kept unchanged.

# Example 1

The example below encasts a byte representation of nested struct
`Container` in little-endian as a value of the type in native-endian.

- Step 1: Struct `Container`, struct `FieldS` and union `FieldU` are
  defined.
  - They implement trait [`Cast`] by applying both attribute
    `#[`[`derive(Cast)`]`]` and attribute `#[`[`repr(C)`]`]` to them.
  - Struct `Container` and struct `FieldS` implement trait [`Flip`] by
    applying attribute `#[`[`derive(Flip)`]`]` to them.
  - Union `FieldU` implements both trait [`Flip`] whose methods do
    nothing and trait [`NopFlip`] by applying attribute
    `#[`[`derive(NopFlip)`]`]` to it.

- Step 2: Method [`EncastMem::encastf`] encasts a byte
  representation of struct `Container` in little-endian as a value of
  the type in native-endian.

```rust
use castflip::{Cast, EncastMem, Flip, LE, NopFlip};

//
// Step 1: Define nested struct `Container` and test data.
//
#[repr(C)]            // to make it possible to apply #[derive(Cast)]
#[derive(Cast, Flip)] // to implement trait Cast and trait Flip
struct Container {  // 16 bytes (in total)
    s: FieldS,      //  8 bytes
    u: FieldU,      //  4 bytes
    f: f32,         //  4 bytes
}

#[repr(C)]            // to make it possible to apply #[derive(Cast)]
#[derive(Cast, Flip)] // to implement trait Cast and trait Flip
struct FieldS {     // 8 bytes (in total)
    s1: [u8; 2],    // 2 bytes
    s2: u16,        // 2 bytes
    s3: u32,        // 4 bytes
}

#[repr(C)]               // to make it possible to apply #[derive(Cast)]
#[derive(Cast, NopFlip)] // to implement trait Cast, trait Flip & trait NopFlip
union FieldU {      // 4 bytes (the largest)
    u1: [u8; 4],    // 4 bytes
    u2: [u16; 2],   // 4 bytes
    u3: u32,        // 4 bytes
}

// Test data: A sample byte representation of struct `Container`
// (16 bytes in little-endian)
const BYTES1: [u8; 16] = [
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
    0x18, 0x19, 0x1a, 0x1b, 0x00, 0x00, 0x48, 0x41,
];

fn my_main() -> Option<()> {
    //
    // Step 2: Encast a byte representation of struct `Container` in
    // little-endian stored in const `BYTES1` as a value of struct
    // `Container` in native-endian and save it to variable `con2`.
    //
    let con2: Container = BYTES1.encastf(LE)?; // LE = Little-Endian

    // Check if the values of all fields in variable `con2.s` (whose type
    // is struct `FieldS` in struct `Container`) are as expected.
    assert_eq!(con2.s.s1, [0x10, 0x11]);  // s1: [u8; 2],
    assert_eq!(con2.s.s2, 0x1312);        // s2: u16,
    assert_eq!(con2.s.s3, 0x17161514);    // s3: u32,

    // Check if the values of all fields in variable `con2.u` (whose type
    // is union `FieldU` in struct `Container`) are as expected.
    // Note that their endiannesses must not be changed.
    unsafe {
        assert_eq!(con2.u.u1, [0x18, 0x19, 0x1a, 0x1b]); // u1: [u8; 4],
        if cfg!(target_endian = "little") {
            assert_eq!(con2.u.u2, [0x1918, 0x1b1a]);     // u2: [u16; 2],
            assert_eq!(con2.u.u3, 0x1b1a1918);           // u3: u32,
        } else if cfg!(target_endian = "big") {
            assert_eq!(con2.u.u2, [0x1819, 0x1a1b]);     // u2: [16; 2],
            assert_eq!(con2.u.u3, 0x18191a1b);           // u3: u32,
#       } else {
#           panic!();
        }
    }

    // Check if the value of variable `con2.f` (whose type is f32 in
    // struct `Container`) is as expected.
    assert_eq!(con2.f, 12.5_f32);  // f: f32,

    Some(())
}

fn main() { my_main(); }
```

# Example 2

The example below encasts a byte representation of the Mach-O[^Mach-O]
fat header as a value of struct `FatHeader`.

[^Mach-O]: [Mach-O] is a file format for executable files.
It is used by most systems based on the Mach kernel such as macOS.

The header has two fields: magic number (4 bytes) and the number of
architectures in the file (4 bytes).  The endianness of the second
field and its successive headers is specified by the first field.  To
demonstrate how trait [`NopFlip`] works, the first 4 bytes (i.e.,
magic number) are read twice.

- Step 1: Struct `FatHeader` and struct `FatMagic` are defined.
  - They implement trait [`Cast`] by applying both attribute
    `#[`[`derive(Cast)`]`]` and attribute `#[`[`repr(C)`]`]` to them.
  - Struct `FatHeader` implements trait [`Flip`] by applying attribute
    `#[`[`derive(Flip)`]`]` to it.
  - struct `FatMagic` implements both trait [`Flip`] whose methods do
    nothing and trait [`NopFlip`] by applying attribute
    `#[`[`derive(NopFlip)`]`]` to it.

- Step 2: Method [`EncastMem::encast`] encasts the first 4 bytes of
  a byte representation of the Mach-O fat header as a value of struct
  `FatMagic` to determine the endianness of the byte representation.

- Step 3: Method [`EncastMem::encastf`] encasts a byte
  representation of the Mach-O fat header as a value of struct
  `FatHeader`.

```rust
use castflip::{Cast, EncastMem, Endian, Flip, NopFlip};

//
// Step 1: Define nested struct `FatHeader` and test data.
//
// struct fat_header and magic numbers are defined in <mach-o/fat.h> at
// /Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/usr/include/ on macOS.
//
#[repr(C)]            // to make it possible to apply #[derive(Cast)]
#[derive(Cast, Flip)] // to implement trait Cast and trait Flip
struct FatHeader {
    magic:     FatMagic,  // Magic number
    nfat_arch: u32,       // Number of architectures in this file.
}

#[repr(C)]               // to make it possible to apply #[derive(Cast)]
#[derive(Cast, NopFlip)] // to implement trait Cast, trait Flip & trait NopFlip
struct FatMagic {
    number: u32,  // Magic number indicating endianness and file type.
}

// The possible values of the magic number above.
const FAT_MAGIC32: u32 = 0xcafebabe; // Native Endian, 32-bit
const FAT_MAGIC64: u32 = 0xcafebabf; // Native Endian, 64-bit
const FAT_CIGAM32: u32 = 0xbebafeca; // Swapped Endian, 32-bit
const FAT_CIGAM64: u32 = 0xbfbafeca; // Swapped Endian, 64-bit

// Test data: A sample byte representation of the Mach-O fat header
// (8 bytes in big-endian)
const BYTES1: [u8; 8] = [0xca, 0xfe, 0xba, 0xbe, 0x00, 0x00, 0x00, 0x02];

fn my_main() -> Option<()> {
    //
    // Step 2: Encast the first 4 bytes of a byte representation of
    // the Mach-O fat header in `BYTES1` as a value of struct `FatMagic`
    // without flipping its endianness to determine the endianness of
    // const `BYTES1`.  The resulting endianness is saved to `endian2`.
    //
    let magic: FatMagic = BYTES1.encast()?;
    let endian2 = match magic.number {
        FAT_MAGIC32 | FAT_MAGIC64 => Endian::Native,
        FAT_CIGAM32 | FAT_CIGAM64 => Endian::Swapped,
        _ => panic!("Unsupported magic number: {:#x}", magic.number),
    };

    // Check if variable `endian2` is equivalent to the big-endianness.
    assert_eq!(endian2.absolute(), Endian::Big);

    //
    // Step 3: Encast a byte representation of the Mach-O fat header
    // in const `BYTES1` as a value of struct `FatHeader` and save it
    // to variable `fat_hdr3`.
    //
    let fat_hdr3: FatHeader = BYTES1.encastf(endian2)?;

    // Check if the value in variable `fat_hdr3.magic` is as expected.
    // The endianness of the value must be kept.
    if cfg!(target_endian = "little") {
        assert_eq!(fat_hdr3.magic.number, FAT_CIGAM32); // Swapped-Endian
    } else if cfg!(target_endian = "big") {
        assert_eq!(fat_hdr3.magic.number, FAT_MAGIC32); // Native-Endian
#     } else {
#       panic!();
    }

    // Check if the value in variable `fat_hdr3.nfat_arch` is as expected.
    // The endianness of the value must be the native-endianness.
    assert_eq!(fat_hdr3.nfat_arch, 2);

    Some(())
}

fn main() { my_main(); }
```

[`derive(Cast)`]: ./derive.Cast.html
[`derive(Flip)`]: ./derive.Flip.html
[`derive(NopFlip)`]: ./derive.NopFlip.html

[Mach-O]: https://en.wikipedia.org/wiki/Mach-O
[PCAP]: https://github.com/IETF-OPSAWG-WG/draft-ietf-opsawg-pcap
[`repr(C)`]: https://doc.rust-lang.org/reference/type-layout.html#the-c-representation
