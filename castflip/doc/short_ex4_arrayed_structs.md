How to convert between bytes and an array of `struct`s

This crate provides methods to convert between a byte representation
of an array type consisting of a regular `struct` type (with named
fields), a tuple `struct` type (with unnamed fields) or a `union` type
and a value of the array type with endianness handling.

For the sake of simplicity, the examples below convert only an array
of a regular `struct` type.

# 1. Converting Bytes to an array of `Struct`s

The example below *encast*s[^encast] a byte representation of an array
of struct `Coord` in big-endian ([`BE`]) as a value of an array type
of the type in native-endian by using method [`EncastMem::encastf`].

```rust
# fn main() {
use castflip::{BE, Cast, EncastMem, Flip};

#[repr(C)]
#[derive(Cast, Flip)]
struct Coord {
    x: u16,
    y: u16,
}

let in_bytes: [u8; 8] = [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0]; // in Big-Endian (`BE`)
let out_coords: [Coord; 2] = in_bytes.encastf(BE).unwrap();
assert_eq!(out_coords[0].x, 0x1234);
assert_eq!(out_coords[0].y, 0x5678);
assert_eq!(out_coords[1].x, 0x9abc);
assert_eq!(out_coords[1].y, 0xdef0);
# }
```

In the case where an input byte representation is in native-endian,
method [`EncastMem::encast`] can be used instead.

# 2. Converting an array of `Struct`s to Bytes

The example below *decast*s[^decast] a value of an array of struct
`Coord` in native-endian as a byte representation of an array type of
the type in big-endian ([`BE`]) by using method
[`DecastMem::decastf`].

```rust
# fn main() {
use castflip::{Cast, DecastMem, Flip, LE};

#[repr(C)]
#[derive(Cast, Flip)]
struct Coord {
    x: u16,
    y: u16,
}

let in_coords = [Coord {x: 0x8765, y: 0x4321}, Coord {x: 0x0fed, y: 0xcba9}];
let mut out_bytes: [u8; 8] = [0; 8];
out_bytes.decastf(&in_coords, LE).unwrap();
assert_eq!(out_bytes, [0x65, 0x87, 0x21, 0x43, 0xed, 0x0f, 0xa9, 0xcb]); // in Little-Endian (`LE`)
# }
```

In the case where an output byte representation is in native-endian,
method [`DecastMem::decast`] can be used instead.

[^encast]: In this crate, to *encast* means to cast a byte
representation of a type as a value of the type.

[^decast]: In this crate, to *decast* means to cast a value of a type
as a byte representation of the type.
