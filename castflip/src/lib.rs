/*!

This crate provides methods for encoding and decoding numeric
variables, arrays and structures in little-endian and big-endian.

In this crate, the term `encast` means decoding a number of bytes to
one or more values, the term `decast` means encoding one or more
variables to a number of bytes, and the term `endian-flip` means
flipping the endianness of value(s).

# Example 1

In the example below, method `encastf` decodes bytes in `bytes1` in
big-endian ([`BE`]) to variable `udp_hdr2` of type `UdpHdr`.  Then,
method `decastf` encodes the resulting value in `udp_hdr2` to bytes in
big-endian ([`BE`]) and stores them in `bytes3`.

```
# fn main() { test(); }
# fn test() -> Option<()> {
use castflip::{Cast, Flip, EncastMem, DecastMem, BE};

#[repr(C)]
#[derive(Cast, Flip)]
struct UdpHdr {     // UDP: https://www.rfc-editor.org/rfc/rfc768.txt
    sport:  u16,    // UDP Source Port
    dport:  u16,    // UDP Destination Port
    len:    u16,    // UDP Length
    sum:    u16,    // UDP Checksum
}

// Input data: UDP header (8 bytes)
let bytes1: [u8; 8] = [0xC3, 0xC9, 0x00, 0x35, 0x00, 0x32, 0x82, 0x3F];

// Decode bytes `bytes1` to variable `udp_hdr2`.
let udp_hdr2 = bytes1.encastf::<UdpHdr>(BE)?;  // BE = Big-Endian

// Encode the resulting UDP header `udp_hdr2` to bytes `bytes3`.
let mut bytes3 = [0_u8; 8];
let size3 = bytes3.decastf::<UdpHdr>(&udp_hdr2, BE)?;

// Check the results (udp_hdr2)
assert_eq!(udp_hdr2.sport, 0xC3C9); // = 50121
assert_eq!(udp_hdr2.dport, 0x0035); // = 53 (DNS)
assert_eq!(udp_hdr2.len,   0x0032); // = 50
assert_eq!(udp_hdr2.sum,   0x823F);

// Check the results (bytes3)
assert_eq!(size3, 8);
assert_eq!(bytes3, bytes1);
# Some(())
# }
```

In the example above, `#[derive(`[`Cast`]`)]` makes the value of
`UdpHdr` `encast`able / `decast`able, and `#[derive(`[`Flip`]`)]`
makes the value of `UdpHdr` `endian-flip`pable.

Trait [`EncastMem`] provides methods to `encast` from memory, and
trait [`DecastMem`] provides methods to `decast` to memory.  The
generic type parameters of their methods can be omitted where the Rust
compiler can infer them.  Their methods whose names end with 'f' flip
the endianness of the results.  The endianness of bytes is specified
in their argument.  [`BE`] is an alias of [`Endian`]`::Big`, which
means Big-Endian.

Note: [UDP] is one of the fundamental protocols in the internet
protocol suite.

[UDP]: https://en.wikipedia.org/wiki/User_Datagram_Protocol

# Example 2

Arrays, structures and unions can be nested as in the example below.

```
# use std::io::Result;
# fn main() { test(); }
# fn test() -> Result<()> {
use std::io::Cursor;
use castflip::{Cast, Flip, NopFlip, EncastIO, DecastIO, LE};

#[repr(C)]
#[derive(Cast, Flip)]
struct StructA {    // 8 bytes (total)
    x: [u8; 2],     // 2 bytes
    y: u16,         // 2 bytes
    z: u32,         // 4 bytes
}

#[repr(C)]
#[derive(Cast, NopFlip)]
union UnionB {      // 4 bytes (largest)
    u: [u8; 4],     // 4 bytes
    v: [u16; 2],    // 4 bytes
    w: u32,         // 4 bytes
}

#[repr(C)]
#[derive(Cast, Flip)]
struct StructC {    // 16 bytes (total)
    a: StructA,     //  8 bytes
    b: UnionB,      //  4 bytes
    f: f32,         //  4 bytes
}

// Input data (16 bytes)
let bytes1: [u8; 16] = [0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
                        0x18, 0x19, 0x1A, 0x1B, 0x00, 0x00, 0x48, 0x41];
let mut input1 = Cursor::new(bytes1);

// Decode input `input1` to variable `var2_c` of type `StructC`.
let var2_c: StructC = input1.encastf(LE)?;  // LE = Little-Endian

// Encode variable `var2_c` to bytes and write them to `output3`.
let mut output3 = Cursor::new(vec![0_u8; 16]);
let size3 = output3.decastf(&var2_c, LE)?;

// Check the results (StructA in StructC)
assert_eq!(var2_c.a.x, [0x10, 0x11]);  // x: [u8; 2],
assert_eq!(var2_c.a.y, 0x1312);        // y: u16,
assert_eq!(var2_c.a.z, 0x17161514);    // z: u32,

// Check the results (UnionB in StructC)
unsafe {
    assert_eq!(var2_c.b.u, [0x18, 0x19, 0x1A, 0x1B]);  // u: [u8; 4],
    if cfg!(target_endian = "little") {
        assert_eq!(var2_c.b.v, [0x1918, 0x1B1A]);      // v: [u16; 2],
        assert_eq!(var2_c.b.w, 0x1B1A1918);            // w: u32,
    } else if cfg!(target_endian = "big") {
        assert_eq!(var2_c.b.v, [0x1819, 0x1A1B]);      // v: [16; 2],
        assert_eq!(var2_c.b.w, 0x18191A1B);            // w: u32,
    }
}

// Check the result (f32 in StructC)
assert_eq!(var2_c.f, 12.5_f32);  // f: f32,

// Check the results (output3)
assert_eq!(size3, 16);
assert_eq!(&output3.into_inner(), &bytes1[..]);
# Ok(())
# }
```

In the example above, `#[derive(`[`Cast`]`)]` makes all types
`encast`able / `decast`able, `#[derive(`[`Flip`]`)]` makes `StructA`
and `StructC` `endian-flip`pable, and `#[derive(`[`NopFlip`]`)]` marks
`UnionB` as `endian-flip`pable but the implemented operation is Nop
(No operation).

Note that [`io::Cursor`] wraps an in-memory buffer and provides it
through [`io::Read`] and [`io::Write`].

Trait [`EncastIO`] provides methods to `encast` from [`io::Read`], and
trait [`DecastIO`] provides methods to `decast` to [`io::Write`].  The
type of the value(s) can be explicitly specified as the generic type
parameter of their methods or implicitly specified so that the Rust
compiler can infer.  Their methods whose names end with 'f' flip the
endianness of the results.  The endianness of bytes is specified in
their argument.  [`LE`] is an alias of [`Endian`]`::Little`, which
means Little-Endian.

# Example 3

A series of structured binary data can be `encast`ed / `decast`ed as
in the example below.

```
# fn main() { test(); }
# fn test() -> Option<()> {
use castflip::{Cast, Flip, EncastMem, DecastMem, BE};

#[repr(C)]
#[derive(Cast, Flip, PartialEq, Debug)]
struct Pair (u16, u16); // 4 bytes

// Input data.
let bytes1: [u8; 12] = [0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27,
                        0x28, 0x29, 0x2A, 0x2B];

// Decode bytes `bytes1` to variable `vec2`.
let vec2: Vec<Pair> = bytes1.encastvf(3, BE)?;  // 3 pairs, BE = Big-Endian

// Encode variable `vec2` to bytes `bytes3`.
let mut bytes3 = [0_u8; 12];
let size3 = bytes3.decastvf(&vec2, BE)?;

// Check the results (vec2)
assert_eq!(vec2, vec![Pair(0x2021, 0x2223), Pair(0x2425, 0x2627),
                      Pair(0x2829, 0x2A2B)]);

// Check the results (bytes3)
assert_eq!(size3, 12);
assert_eq!(bytes3, bytes1);
# Some(())
# }
```

In the example above, `#[derive(`[`Cast`]`)]` makes the values of
`Pair` `encast`able / `decast`able, and `#[derive(`[`Flip`]`)]` makes
the values of `Pair` `endian-flip`pable.

Trait [`EncastMem`] provides methods to `encast` from memory, and
trait [`DecastMem`] provides methods to `decast` to memory.  The type
of the value(s) can be explicitly specified as the generic type
parameter of their methods or implicitly specified so that the Rust
compiler can infer.  The methods whose name contain 'v' (= vector)
`encast` / `decast` a series of structured binary data, and the
methods whose names end with 'f' flip the endianness of the results.
The number of elements is specified in the argument of method
`encastvf`.  The endianness of bytes is specified in their argument.
[`BE`] is an alias of [`Endian`]`::Big`, which means Big-Endian.

Note: The reason why `#[derive(PartialEq, Debug)]` is declared is that
assert_eq! requires them.  This crate works without them.

# Summary

This crate provides methods for encoding and decoding numeric
variables, arrays and structures in little-endian and big-endian.
They can be nested and arranged in a series.

Two types of endianness is defined in enum [`Endian`]: relative endian
(Native or Swapped) and absolute endian (Little or Big).

## List of traits to mark characteristics of types

* [`Cast`] marks types as `encast`able / `decast`able
* [`Flip`] marks types as `endian-flip`pable
* [`NopFlip`] marks types as `endian-flip`pable but the implemented
  operation is Nop (No operation)

They can be implemented by declaring `#[derive(`[`Cast`]`)]`,
`#[derive(`[`Flip`]`)]` and `#[derive(`[`NopFlip`]`)]` respectively if
required conditions are met.

## List of methods defined in trait [`EncastMem`] and trait [`EncastIO`]

* `encast` decodes a required number of bytes to a value.
* `encastf` decodes a required number of bytes to a value with
  `endian-flip`ping.
* `encastv` decodes a required number of bytes to a vector of value(s).
* `encastvf` decodes a required number of bytes to a vector of value(s)
  with `endian-flip`ping.

The methods defined in trait [`EncastMem`] returns `Option` and
the methods defined in trait [`EncastIO`] returns `io::Result`.
If successful, they return the resulting value(s).
The endianness of the bytes in `self` is specified in the arguments of
`encastf` and `encastvf`.  The number of elements in the resulting
vector is specified in the arguments of `encastv` and `encastvf`.

## List of methods defined in trait [`DecastMem`] and trait [`DecastIO`]

* `decast` encodes a variable to a number of bytes.
* `decastf` encodes a variable to a number of bytes with `endian-flip`ping.
* `decastv` encodes a slice of variable(s) to a number of bytes.
* `decastvf` encodes a slice of variable(s) to a number of bytes
  with `endian-flip`ping.

The methods defined in trait [`DecastMem`] returns `Option` and
the methods defined in trait [`DecastIO`] returns `io::Result`.
If successful, they return the number of resulting bytes.
The endianness of resulting bytes is specified in the arguments of
`decastf` and `decastvf`.

## Notes on trait `EncastIO` and trait `DecastIO`

[`EncastIO`] and [`DecastIO`] provide methods to `encast` / `decast`
through [`io::Read`] and [`io::Write`].  Remember that [`io::Read`]
and [`io::Write`] are implemented also for `&[u8]`.  Additionally,
[`io::Cursor`] wraps an in-memory buffer and provides it through
[`io::Read`] and [`io::Write`].  Therefore, [`EncastIO`] and
[`DecastIO`] would enable to write a program code that can `encast`
from and `decast` to file, network and memory.

For more information, please see the description of each trait and
enum listed below.

 */


// Modules in files in the same directory.
#[doc(hidden)] pub mod cast;
#[doc(hidden)] pub mod decast_io;
#[doc(hidden)] pub mod decast_mem;
#[doc(hidden)] pub mod encast_io;
#[doc(hidden)] pub mod encast_mem;
#[doc(hidden)] pub mod endian;
#[doc(hidden)] pub mod flip;
#[doc(hidden)] pub mod nop_flip;

// A module in a subdirectory.
pub mod experimental;


// Enum Endian and aliases of variants.
#[doc(inline)] pub use self::endian::{Endian, NE, SE, LE, BE};

// Trait Cast, trait Flip, and related derive macros.
#[doc(inline)] pub use self::cast::Cast;
#[doc(inline)] pub use self::flip::Flip;
#[doc(inline)] pub use self::nop_flip::NopFlip;
#[doc(hidden)] pub use castflip_derive::{Cast, Flip, NopFlip};

// Traits to encast and decast on memory.
#[doc(inline)] pub use self::encast_mem::EncastMem;
#[doc(inline)] pub use self::decast_mem::DecastMem;

// Traits to encast and decast through I/O.
#[doc(inline)] pub use self::encast_io::EncastIO;
#[doc(inline)] pub use self::decast_io::DecastIO;


// A module used in document comment.
#[cfg(doc)] use std::io;

// Make sure that this crate supports the endianness of the target system.
#[cfg(not(any(target_endian = "little", target_endian = "big")))]
core::compile_error!("Unsupported endian");
