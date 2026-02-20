Marks types whose values' endiannesses can be flipped by the methods
of this crate.

# Description

The endiannesses of the values of types that implement trait [`Flip`]
can be flipped by the methods of this crate.

Trait [`Flip`] is implemented for

- all primitive numeric types, i.e.,
    * `i8`, `i16`, `i32`, `i64`, `i128`, `isize`,
    * `u8`, `u16`, `u32`, `u64`, `u128`, `usize`,
    * `f32`, and `f64`,
- array types whose elements' types implement trait [`Flip`],
- `struct` types whose all fields' types implement trait [`Flip`] or
  with no field, and whose type definitions are annotated with
  attribute `#[`[`derive(Flip)`]`]`, and
- `struct` types and `union` types whose all fields' types
  implement trait [`Flip`] and whose type definitions are annotated
  with attribute `#[`[`derive(NopFlip)`]`]`[^NopFlip].

[^NopFlip]: The types annotated with attribute `#[`[`derive(NopFlip)`]`]`
implement both trait [`Flip`] with NOP (No OPeration) methods and
marker trait [`NopFlip`].

The methods of trait [`Flip`] are designed mainly for internal use.
They are called by the methods of this crate to flip the endiannesses
of their resulting values or byte representations.  However, if they
satisfy your requirements, you may call them manually to flip the
endiannesses of your values.

# Safety

You must not implement trait [`Flip`] manually unless you know what
you are doing.

The recommended way to implement trait [`Flip`] for a struct type is
to apply attribute `#[`[`derive(Flip)`]`]` to the type.

Note that the internal specification of attribute
`#[`[`derive(Flip)`]`]` may be revised in a future release.

# Method Naming Convention

Each method name consits of two parts.

The first part determines the result handling.  The methods whose
names start with

1. "`flip_val`" return the value in `self`, and

2. "`flip_var`" manipulate the value in `self`.

The second part determines the endianness handling.  The methods whose
names end with

1. "" (no suffix) flip the endianness of the value if parameter
   `endian` is not equivalent to the endianness of the target system,
   and

2. "`_swapped`" reverse the endianness, i.e., the endianness of the
   resulting value is reversed from the endianness of the source
   value.

# Comparison With Trait `Copy`

As you may have noticed by reading [the Description
section](#description) above, all types that implement trait [`Flip`]
can be duplicated simply by copying bits.  However, trait [`Flip`] is
not a subtrait of trait [`Copy`].  The reasons why trait [`Flip`] is
defined independently from trait [`Copy`] are (1) to exclude pointers,
and (2) to avoid unexpected copy operation.  Therefore, when a
`struct` type or a `union` type needs to implement both trait [`Flip`]
and trait [`Copy`], you must implement both traits explicitly.

When the methods of this crate flip the endiannesses of one or more
values of a type that implement trait [`Flip`], the methods read and
write the bits of the values by the functions provided by module
[`core::ptr`] instead of by the Rust assignment expressions because
the type may not implement trait [`Copy`].

# Comparison With Trait `Cast`

The set of types that can implement trait [`Flip`] is equal to
the set of types that can implement trait [`Cast`].

But there is a difference between derive macros that support them;
attribute `#[`[`derive(Flip)`]`]` does not support a `union` type
because there is no common way to flip the endianness of a `union`
type, while attribute `#[`[`derive(Cast)`]`]` supports a `union` type
because the value of a `union` type can be duplicated by copying bits.

In order to distinguish the difference properly, trait [`Flip`] is
defined independently from trait [`Cast`].

# `union` types and trait `Flip`

Because attribute `#[`[`derive(Flip)`]`]` cannot be applied to a
`union` type, if a `struct` type contains a field of a `union` type,
attribute `#[`[`derive(Flip)`]`]` cannot be applied to the `struct`
type unless the `union` type manually implements trait [`Flip`].  In
order to simplify such manual work, attribute
`#[`[`derive(NopFlip)`]`]` is introduced.

Attribute `#[`[`derive(NopFlip)`]`]` can be applied to a `struct` type
or a `union` type if its all fields implement [`Flip`] or it has no
field.  By applying the attribute to such type, it implements trait
[`Flip`] whose methods do nothing (NOP = No OPeration).  What you
should do manually is to flip the endiannesses of its values or its
byte representations where necessary.  In addition, it also implements
trait [`NopFlip`].

For more information, see the description of trait [`NopFlip`].

# Example 1

The example below encasts a byte representation of the UDP[^UDP]
header in big-endian ([`BE`]) as a value of struct `UdpHdr` in
native-endian.

[^UDP]: The User Datagram Protocol ([UDP]) is one of the fundamental
protocols of the Internet protocol suite.  It is defined in [RFC768].
It is exhcanged in big-endian on the Internet.

- Step 1: Struct `UdpHdr` is defined.
    - It implements trait [`Cast`] by applying both attribute
      `#[`[`derive(Cast)`]`]` and attribute `#[`[`repr(C)`]`]` to it.
    - It implements trait [`Flip`] by applying attribute
      `#[`[`derive(Flip)`]`]` to it.

- Step 2: Method [`EncastMem::encastf`] encasts a byte
  representation of the UDP header in big-endian as a value of struct
  `UdpHdr` in native-endian.

```rust
# fn main() {
use castflip::{BE, Cast, EncastMem, Flip};

//
// Step 1: Define struct `UdpHdr`.
//
#[repr(C)]            // to make it possible to apply #[derive(Cast)]
#[derive(Cast, Flip)] // to implement trait Cast and trait Flip
struct UdpHdr {  // UDP: See https://www.rfc-editor.org/rfc/rfc768.txt
    sport: u16,  // UDP Source Port
    dport: u16,  // UDP Destination Port
    len:   u16,  // UDP Length in Bytes (header plus data)
    sum:   u16,  // UDP Checksum
}

//
// Step 2: Encast a byte representation of the UDP header in big-endian
// (`BE`) stored in variable `in_bytes` as a value of struct `UdpHdr` in
// native-endian and save it to variable `out_hdr`.
//

// Input: A sample byte representation of the UDP header (8 bytes)
let in_bytes: [u8; 8] = [0xc3, 0xc9, 0x00, 0x35, 0x00, 0x32, 0x82, 0x3f];

// Encast a byte representation in big-endian (`BE`) as a value.
let out_hdr: UdpHdr = in_bytes.encastf(BE).unwrap();

// Check if all fields in variable `out_hdr` are as expected.
assert_eq!(out_hdr.sport, 0xc3c9);  // = 50121 (Ephemeral Port)
assert_eq!(out_hdr.dport, 0x0035);  // = 53 (DNS Port)
assert_eq!(out_hdr.len,   0x0032);  // = 50 (Length in Bytes)
assert_eq!(out_hdr.sum,   0x823f);  // = 0x823f (Checksum)
# }
```

# Example 2

In the example below, the endiannesses of a couple of values are
flipped by using the mothods of trait [`Flip`].

```rust
# fn main() {
use castflip::{BE, Flip, LE};

//
// Step 1: Read the value of variable `int0` in little-endian (`LE`)
// and save it to variable `int1` in native-endian.
//
let int0 = 0x1234_u16.to_le(); // `int0` is in little-endian.
let int1 = int0.flip_val(LE);  // `int1` is in native-endian.

// Check if the value in variable `int1` is as expected.
assert_eq!(int1, 0x1234);

//
// Step 2: Read the literal integer 0x5678 in big-endian (`BE`)
// and save it to variable `int2` in native-endian.
//
let int2 = 0x5678_u16.to_be().flip_val(BE); // `self` is in big-endian.

// Check if the value in variable `int2` is as expected.
assert_eq!(int2, 0x5678);

//
// Step 3: Flip the endianness of the value in variable `int3`
// from the little-endianness (`LE`) to the native-endianness.
//
let mut int3 = 0xabcd_u16.to_le(); // `int3` is in little-endian.
int3.flip_var(LE); // `int3` is in native-endian now.

// Check if the value in variable `int3` is as expected.
assert_eq!(int3, 0xabcd);
# }
```

[`derive(Cast)`]: ./derive.Cast.html
[`derive(Flip)`]: ./derive.Flip.html
[`derive(NopFlip)`]: ./derive.NopFlip.html

[`repr(C)`]: https://doc.rust-lang.org/reference/type-layout.html#the-c-representation
[RFC768]: https://www.rfc-editor.org/rfc/rfc768.txt
[UDP]: https://en.wikipedia.org/wiki/User_Datagram_Protocol
