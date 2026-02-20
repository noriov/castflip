How to convert between bytes and a number

# 1. Converting Bytes to a Number

The example below *encast*s[^encast] a byte representation of `u32` in
big-endian ([`BE`]) as a value of `u32` in native-endian by using
method [`EncastMem::encastf`].

```rust
# fn main() {
use castflip::{BE, EncastMem};

let in_bytes: [u8; 4] = [0x12, 0x34, 0x56, 0x78];
let out_number: u32 = in_bytes.encastf(BE).unwrap();
assert_eq!(out_number, 0x12345678);
# }
```

In the case where an input byte representation is in native-endian,
method [`EncastMem::encast`] can be used instead.

# 2. Converting a Number to Bytes

The example below *decast*s[^decast] a value of `u32` in native-endian
as a byte representation of `u32` in little-endian ([`LE`]) by using
method [`DecastMem::decastf`].

```rust
# fn main() {
use castflip::{DecastMem, LE};

let in_number: u32 = 0x87654321;
let mut out_bytes: [u8; 4] = [0; 4];
out_bytes.decastf(&in_number, LE).unwrap();
assert_eq!(out_bytes, [0x21, 0x43, 0x65, 0x87]);
# }
```

In the case where an output byte representation is in native-endian,
method [`DecastMem::decast`] can be used instead.

[^encast]: In this crate, to *encast* means to cast a byte
representation of a type as a value of the type.

[^decast]: In this crate, to *decast* means to cast a value of a type
as a byte representation of the type.
