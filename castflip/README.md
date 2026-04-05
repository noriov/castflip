# Crate castflip

Crate castflip is a Rust library for encoding and decoding numeric
variables, arrays and structures.  It provides methods to convert
between a byte representation of a fixed binary format and a value of
a Rust type with endianness handling.

# Introduction

Crate castflip provides several traits

- to *encast* a byte representation of a type as a value of the type
  with endianness handling,
- to *decast* a value of a type as a byte representation of the type
  with endianness handling, and
- to be used as bounds to determine which methods can be applied.

The supported types include

1. primitive numeric types, and
2. array types, `struct` types and `union` types consisting of the
   supported types.

For more information, please see the [documentation] of crate castflip.
It includes a number of examples and summaries as well as the detailed
descriptions of its types and its traits.

# A Simple Example

The example below encasts a byte representation of the UDP
header in big-endian as a value of struct `UdpHdr` in native-endian.

```rust
use castflip::{BE, Cast, EncastMem, Flip};

//
// Step 1: Define struct `UdpHdr` (The UDP header) and test data.
//
#[repr(C)]            // to make it possible to apply #[derive(Cast)]
#[derive(Cast, Flip)] // to implement trait Cast and trait Flip
struct UdpHdr {  // UDP: See https://www.rfc-editor.org/rfc/rfc768.txt
    sport: u16,  // UDP Source Port
    dport: u16,  // UDP Destination Port
    len:   u16,  // UDP Length in Bytes (header plus data)
    sum:   u16,  // UDP Checksum
}

// Input: A sample byte representation of the UDP header (8 bytes)
let in_bytes: [u8; 8] = [0xc3, 0xc9, 0x00, 0x35, 0x00, 0x32, 0x82, 0x3f];

//
// Step 2: Encast a byte representation of the UDP header in big-endian
// (`BE`) at the head of variable `in_bytes` as a value of struct
// `UdpHdr` in native-endian and save it to variable `out_hdr`.
//
let out_hdr: UdpHdr = in_bytes.encastf(BE).unwrap();

// Check if all fields in variable `out_hdr` are as expected.
assert_eq!(out_hdr.sport, 0xc3c9);  // = 50121 (Ephemeral Port)
assert_eq!(out_hdr.dport, 0x0035);  // = 53 (DNS Port)
assert_eq!(out_hdr.len,   0x0032);  // = 50 (Length in Bytes)
assert_eq!(out_hdr.sum,   0x823f);  // = 0x823f (Checksum)
```

To use crate castflip version 0.1, add the following lines to your
`Cargo.toml`:

```toml
[dependencies]
castflip = "0.1"
```

[documentation]: https://docs.rs/castflip/0.1
