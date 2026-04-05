How to convert between bytes and an array of numbers

The examples below show how to convert between a byte representation
of an array type consisting of a primitive numeric type and a value of
the array type with endianness handling.

# 1. Converting Bytes to an Array of Numbers

The example below *encast*s[^encast] a byte representation of an array
of `u16` in big-endian ([`BE`]) as a value of the array type in
native-endian by using method [`EncastMem::encastf`].

```rust
# fn main() {
use castflip::{BE, EncastMem};

let in_bytes: [u8; 4] = [0x12, 0x34, 0x56, 0x78]; // in Big-Endian (`BE`)
let out_numbers: [u16; 2] = in_bytes.encastf(BE).unwrap();
assert_eq!(out_numbers, [0x1234, 0x5678]);
# }
```

In the case where an input byte representation is in native-endian,
method [`EncastMem::encast`] can be used instead.

# 2. Converting an Array of Numbers to Bytes

The example below *decast*s[^decast] a value of an array of `u16` in
native-endian as a byte representation of the array type in
little-endian ([`LE`]) by using method [`DecastMem::decastf`].

```rust
# fn main() {
use castflip::{DecastMem, LE};

let in_numbers: [u16; 2] = [0x8765, 0x4321];
let mut out_bytes: [u8; 4] = [0; 4];
out_bytes.decastf(&in_numbers, LE).unwrap();
assert_eq!(out_bytes, [0x65, 0x87, 0x21, 0x43]); // in Little-Endian (`LE`)
# }
```

In the case where an output byte representation is in native-endian,
method [`DecastMem::decast`] can be used instead.

[^encast]: In this crate, to *encast* means to cast a byte
representation of a type as a value of the type.

[^decast]: In this crate, to *decast* means to cast a value of a type
as a byte representation of the type.
