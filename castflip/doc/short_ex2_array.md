How to convert between a byte representation and an array of numbers

# 1. Converting a Byte Representation to an Array of Numbers

The example below *encast*s[^encast] a byte representation of type
`[u16; 2]` in big-endian ([`BE`]) as a value of type `[u16; 2]` in
native-endian by using method [`EncastMem::encastf`].

```rust
# fn main() {
use castflip::{BE, EncastMem};

let in_bytes: [u8; 4] = [0x12, 0x34, 0x56, 0x78];
let out_value: [u16; 2] = in_bytes.encastf(BE).unwrap();
assert_eq!(out_value, [0x1234, 0x5678]);
# }
```

In the case where an input byte representation is in native-endian,
method [`EncastMem::encast`] can be used instead.

# 2. Converting an Array of Numbers to a Byte Representation

The example below *decast*s[^decast] a value of type `[u16; 2]` in
native-endian as a byte representation of type `[u16; 2]` in
little-endian ([`LE`]) by using method
[`DecastMem::decastf`].

```rust
# fn main() {
use castflip::{DecastMem, LE};

let in_value: [u16; 2] = [0x8765, 0x4321];
let mut out_bytes: [u8; 4] = [0; 4];
out_bytes.decastf(&in_value, LE).unwrap();
assert_eq!(out_bytes, [0x65, 0x87, 0x21, 0x43]);
# }
```

In the case where an output byte representation is in native-endian,
method [`DecastMem::decast`] can be used instead.

[^encast]: In this crate, to *encast* means to cast a byte
representation of a type as a value of the type.

[^decast]: In this crate, to *decast* means to cast a value of a type
as a byte representation of the type.
