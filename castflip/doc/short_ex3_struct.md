How to convert between a byte representation and a value of a `struct`
type

Both regular `struct` types which have named fields and tuple `struct`
types which have unnamed fields can be converted.

# 1. Converting a Byte Representation of a Struct Type to a Value

The example below *encast*s[^encast] a byte representation of struct
`Coord` in big-endian ([`BE`]) as a value of struct `Coord` in
native-endian by using method [`EncastMem::encastf`].

```rust
# fn main() {
use castflip::{BE, Cast, EncastMem, Flip};

#[repr(C)]
#[derive(Cast, Flip)]
struct Coord {
    x: u16,
    y: u16,
}

let in_bytes: [u8; 4] = [0x12, 0x34, 0x56, 0x78];
let out_coord: Coord = in_bytes.encastf(BE).unwrap();
assert_eq!(out_coord.x, 0x1234);
assert_eq!(out_coord.y, 0x5678);
# }
```

In the case where an input byte representation is in native-endian,
method [`EncastMem::encast`] can be used instead.

# 2. Converting a Value of a Struct Type to a Byte Representation

The example below *decast*s[^decast] a value of struct `Coord` in
native-endian as a byte representation of struct `Coord` in big-endian
([`BE`]) by using method [`DecastMem::decastf`].

```rust
# fn main() {
use castflip::{BE, Cast, DecastMem, Flip};

#[repr(C)]
#[derive(Cast, Flip)]
struct Coord {
    x: u16,
    y: u16,
}

let in_coord = Coord { x: 0x08765, y: 0x4321 };
let mut out_bytes: [u8; 4] = [0; 4];
out_bytes.decastf(&in_coord, BE).unwrap();
assert_eq!(out_bytes, [0x87, 0x65, 0x43, 0x21]);
# }
```

In the case where an output byte representation is in native-endian,
method [`DecastMem::decast`] can be used instead.

[^encast]: In this crate, to *encast* means to cast a byte
representation of a type as a value of the type.

[^decast]: In this crate, to *decast* means to cast a value of a type
as a byte representation of the type.
