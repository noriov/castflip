Marks types whose values can be encasted[^encast] and
decasted[^decast] by the methods of this crate.

[^encast]: In this crate, to *encast* means to cast a byte
representation of a type as a value of the type.

[^decast]: In this crate, to *decast* means to cast a value of a type
as a byte representation of the type.

# Description

The values of types that implement Trait [`Cast`] can be encasted and
decasted by the methods of this crate.

Trait [`Cast`] is implemented for

- all primitive numeric types, i.e.,
    * `i8`, `i16`, `i32`, `i64`, `i128`, `isize`,
    * `u8`, `u16`, `u32`, `u64`, `u128`, `usize`,
    * `f32`, and `f64`,
- array types whose elements' types implement trait [`Cast`],
- `struct` types and `union` types whose all fields' types implement
  trait [`Cast`] and whose type definitions are annotated with both
  attribute `#[`[`derive(Cast)`]`]` and attribute
  `#[`[`repr(C)`]`]`, and
- `struct` types with no field and whose type definitions are
  annotated with attribute `#[`[`derive(Cast)`]`]`.

Trait [`Cast`] has no method.  It is defined as a marker.

# Safety

You must not implement trait [`Cast`] manually unless you know what
you are doing.

The recommended way to implement trait [`Cast`] for a `struct` type or
a `union` type is to apply both attribute `#[`[`derive(Cast)`]`]` and
attribute `#[`[`repr(C)`]`]` to the type.  If the type has no field,
attribute `#[`[`repr(C)`]`]` can be omitted.

Note that the internal specification of attribute
`#[`[`derive(Cast)`]`]` may be revised in a future release.

# Comparison With Trait `Copy`

As you may have noticed by reading [the Description
section](#description) above, all types that implement trait [`Cast`]
can be duplicated simply by copying bits.  However, trait [`Cast`] is
not a subtrait of trait [`Copy`].  The reasons why trait [`Cast`] is
defined independently from trait [`Copy`] are (1) to exclude pointers,
and (2) to avoid unexpected copy operation.  Therefore, when a
`struct` type or a `union` type needs to implement both trait [`Cast`]
and trait [`Copy`], you must implement both traits explicitly.

When the methods of this crate encast one or more byte representations
of a type that implement trait [`Cast`] as one or more values of the
type, or the methods of this crate decast one or more values of a type
that implement trait [`Cast`] as one or more byte representations of
the type, the methods copy the bits between the values and the byte
representations by the functions provided by module [`core::ptr`]
instead of by the Rust assignment expressions because (1) the type may
not implement trait [`Copy`], and (2) the byte representations of the
type may not be placed on their natural alignment.

# Comparison With Trait `Flip`

The set of types that can implement trait [`Cast`] is equal to
the set of types that can implement trait [`Flip`].

But there is a difference between derive macros that support them;
attribute `#[`[`derive(Cast)`]`]` supports a `union` type because the
value of a `union` type can be duplicated by copying bits, while
attribute `#[`[`derive(Flip)`]`]` does not support a `union` type
because there is no common way to flip the endianness of a `union`
type.

In order to distinguish the difference properly, trait [`Cast`] is
defined independently from trait [`Flip`].

# Example

The example below encasts a byte representation of the ELF[^ELF]
Identification header as a value of struct `ElfIdHdr`.

[^ELF]: The Executable and Linkable Format ([ELF]) is the primary
executable file format in many operating systems including Linux.  The
[ELF] Identification header is the first 16 bytes of the [ELF] header.

- Step 1: Struct `ElfIdHdr` is defined.
    - It implements trait [`Cast`] by applying both attribute
      `#[`[`derive(Cast)`]`]` and attribute `#[`[`repr(C)`]`]` to it.

- Step 2: Method [`EncastMem::encastf`] encasts a byte representation
  of the ELF Identification header as a value of struct `ElfIdHdr`.

```rust
# fn main() {
use castflip::{Cast, EncastMem};

//
// Step 1: Define struct `ElfIdHdr`.
// Because it has no multi-byte field, #[derive(Flip)] is not necessary.
//
#[repr(C)]      // to make it possible to apply #[derive(Cast)]
#[derive(Cast)] // to implement trait Cast
struct ElfIdHdr { // The ELF Identification header
    magic:      [u8; 4],  //00-03: Magic Number 0x7f "ELF"
    class:      u8,       //04   : File Class (1 = 32-bit, 2 = 64-bit)
    data:       u8,       //05   : Data Encoding (1 = little, 2 = big endian)
    version:    u8,       //06   : ELF Version (should be 1)
    osabi:      u8,       //07   : OS and ABI
    abiversion: u8,       //08   : ABI Version
    pad:        [u8; 7],  //09-0f: Padding (should be 0)
}

//
// Step 2: Encast a byte representation of the ELF Identification header
// stored in variable `in_bytes` as a value of struct `ElfIdHdr` and
// save it to variable `out_hdr`.
//

// Input: A sample byte representation of the ELF Identification header
let in_bytes: [u8; 16] = [
    0x7f, 0x45, 0x4c, 0x46, 0x02, 0x01, 0x01, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

// Encast a byte representation as a value.
let out_hdr: ElfIdHdr = in_bytes.encast().unwrap();

// Check if all fields in variable `out_hdr` are as expected.
assert_eq!(out_hdr.magic, *b"\x7fELF");  // Magic Number: 7f 45 4c 46
assert_eq!(out_hdr.class, 2);            // File Class: 64-bit
assert_eq!(out_hdr.data, 1);             // Data Encoding: Little-Endian
assert_eq!(out_hdr.version, 1);          // ELF Version: 1
assert_eq!(out_hdr.osabi, 0);            // OS and ABI: unspecified
assert_eq!(out_hdr.abiversion, 0);       // ABI Version: unspecified
assert_eq!(out_hdr.pad, [0_u8; 7]);      // Padding
# }
```

[`derive(Cast)`]: ./derive.Cast.html
[`derive(Flip)`]: ./derive.Flip.html

[ELF]: https://en.wikipedia.org/wiki/Executable_and_Linkable_Format
[`repr(C)`]: https://doc.rust-lang.org/reference/type-layout.html#the-c-representation
