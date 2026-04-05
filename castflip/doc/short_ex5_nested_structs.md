How to convert between bytes and nested `struct`s

This crate provides methods to convert between a byte representation
of a nested type consiting of any combination of regular `struct`
types (with named fields), tuple `struct` types (with unnamed fields)
and `union` types and a value of the nested type with endianness
handling.

For the sake of simplicity, the examples below convert only a nested
type of regular `struct` types.

# 1. Converting Bytes to Nested `Struct`s

The example below *encast*s[^encast] a byte representation of struct
`Point` in big-endian ([`BE`]) as a value of the type in native-endian
by using method [`EncastMem::encastf`].

```rust
# fn main() {
use castflip::{BE, Cast, EncastMem, Flip};

#[repr(C)]
#[derive(Cast, Flip)]
struct Point {
    coord: Coord,
    color: u32,
}

#[repr(C)]
#[derive(Cast, Flip)]
struct Coord {
    x: u16,
    y: u16,
}

let in_bytes: [u8; 8] = [
    0x12, 0x34, 0x56, 0x78,
    0x00, 0xab, 0xcd, 0xef,
]; // in Big-Endian (`BE`)

let out_point: Point = in_bytes.encastf(BE).unwrap();
assert_eq!(out_point.coord.x, 0x1234);
assert_eq!(out_point.coord.y, 0x5678);
assert_eq!(out_point.color, 0xabcdef);
# }
```

In the case where an input byte representation is in native-endian,
method [`EncastMem::encast`] can be used instead.

# 2. Converting Nested `Struct`s to Bytes

The example below *decast*s[^decast] a value of struct `Point` in
native-endian as a byte representation of the type in big-endian
([`BE`]) by using method [`DecastMem::decastf`].

```rust
# fn main() {
use castflip::{Cast, DecastMem, Flip, LE};

#[repr(C)]
#[derive(Cast, Flip)]
struct Point {
    coord: Coord,
    color: u32,
}

#[repr(C)]
#[derive(Cast, Flip)]
struct Coord {
    x: u16,
    y: u16,
}

let in_point = Point {
    coord: Coord { x: 0x8765, y: 0x4321 },
    color: 0xfedcba,
};

let mut out_bytes: [u8; 8] = [0; 8];
out_bytes.decastf(&in_point, LE).unwrap();
assert_eq!(out_bytes, [
    0x65, 0x87, 0x21, 0x43,
    0xba, 0xdc, 0xfe, 0x00,
]); // in Little-Endian (`LE`)
# }
```

In the case where an output byte representation is in native-endian,
method [`DecastMem::decast`] can be used instead.

[^encast]: In this crate, to *encast* means to cast a byte
representation of a type as a value of the type.

[^decast]: In this crate, to *decast* means to cast a value of a type
as a byte representation of the type.
