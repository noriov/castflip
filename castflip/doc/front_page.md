Crate castflip is a Rust library for encoding and decoding numeric
variables, arrays and structures in little-endian and big-endian.
It provides methods to convert between a byte representation of a
format and a value of a Rust type with endianness handling.

# Introduction

Crate castflip provides several traits

- to *encast* a byte representation of a type as a value of the type,
- to *decast* a value of a type as a byte representation of the type, and
- to flip the endianness of a value of a type as required.

The supported types include

1. primitive numeric types, and
2. array types, `struct` types and `union` types consisting of the
   supported types.

# A Simple Example

The example below encasts a byte representation of the UDP[^UDP]
header in big-endian as a value of a `struct` type in native-endian.

[^UDP]: The User Datagram Protocol ([UDP]) is one of the fundamental
protocols of the Internet protocol suite.  It is defined in [RFC768].
It is exhcanged in big-endian on the Internet.

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

# Dependencies

To use crate castflip version 0.1, add the following lines to your
`Cargo.toml`:

```toml
[dependencies]
castflip = "0.1"
```

# Documents

The documents below describes the API of the castflip crate version
0.1.

Short Examples as a Quick Start Guide:

1. [How to convert between bytes and a number
   ](./documents/short_example1/index.html)
2. [How to convert between bytes and an array of numbers
   ](./documents/short_example2/index.html)
3. [How to convert between bytes and a `struct`
   ](./documents/short_example3/index.html)
4. [How to convert between bytes and an array of `struct`s
   ](./documents/short_example4/index.html)

Long Examples with Explanations:

1. [How to convert between bytes and a `struct` (The UDP header)
   ](./documents/long_example1/index.html)
2. [How to convert between bytes and a nested `struct`
   ](./documents/long_example2/index.html)
3. [How to convert between bytes and an array of `struct`s
   ](./documents/long_example3/index.html)
    1. [As a value of type `[T; N]`
       ](./documents/long_example3_1/index.html)
    2. [As consecutive values of type `T`
       ](./documents/long_example3_2/index.html)
    3. [As an element of slice `[[T; N]]`
       ](./documents/long_example3_3/index.html)
4. [How to convert between bytes and a `struct` (The UDP header)
   using `std::io`](./documents/long_example4/index.html)
    1. [Through a byte stream provided by struct `std::io::Cursor`
       ](./documents/long_example4_1/index.html)
    2. [Through a byte stream provided by a mutable byte slice `&mut [u8]`
       ](./documents/long_example4_2/index.html)

Summaries of Types, Traits, Crate Features, etc.:

1. [Enum Type: `Endian`](./documents/summary1/index.html)
2. [Traits as Bounds: `Cast`, `Flip` and `NopFlip`
   ](./documents/summary2/index.html)
3. [Traits to Encast and Decast: `EncastMem` and `DecastMem` /
   `EncastIO` and `DecastIO`](./documents/summary3/index.html)
4. [Crate Features: `alloc` and `std`](./documents/summary4/index.html)
5. [Planned Releases: Version 0.2 and 0.3](./documents/summary5/index.html)

[RFC768]: https://www.rfc-editor.org/rfc/rfc768.txt
[UDP]: https://en.wikipedia.org/wiki/User_Datagram_Protocol
