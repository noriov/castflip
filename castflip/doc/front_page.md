This crate provides several traits

- to *encast* a byte representation of a type as a value of the type,
- to *decast* a value of a type as a byte representation of the type, and
- to *flip* the endianness of a value of a type as required.

The supported types include

1. primitive numeric types, and
2. array types, `struct` types and `union` types consisting of the
   supported types.

This document describes the API of the castflip crate version 0.1.

# Documents

Examples:

1. [Basic Example: A `struct` type is encasted and decasted
    with its endianness flipped as required.](./documents/example1/index.html)
2. [Nested Example: A nested `struct` type is encasted and decasted
    with its endianness flipped as required.](./documents/example2/index.html)
3. [Arrayed Example: An array of a `struct` type is encasted and decasted
    with its endianness flipped as required.](./documents/example3/index.html)
4. [I/O Example: A `struct` type is encasted and decasted
    with its endianness flipped as required using trait `std::io::Read` and
    trait `std::io::Write`.](./documents/example4/index.html)

A summary of types, traits and crate features:

1. [Enum Type: `Endian`](./documents/summary1/index.html)
2. [Traits as Bounds: `Cast`, `Flip` and `NopFlip`
   ](./documents/summary2/index.html)
3. [Traits to Cast and Flip: `DecastMem` and `EncastMem`/
    `DecastIO` and `EncastIO`](./documents/summary3/index.html)
4. [Crate Features: `alloc` and `std`](./documents/summary4/index.html)
5. [Planned Releases: Version 0.2 and 0.3](./documents/summary5/index.html)

# A Simple Example

In this example, a byte representation of the UDP[^UDP] header in
big-endian is encasted as a value of a `struct` type in native-endian.

[^UDP]: The User Datagram Protocol ([UDP]) is one of the fundamental
protocols of the Internet protocol suite.  It is defined in [RFC768].
It is exhcanged in big-endian on the Internet.

```rust
use castflip::{BE, Cast, EncastMem, Flip};

//
// Step 1: Define struct `UdpHdr` and test data.
//
#[repr(C)]            // to make it possible to apply #[derive(Cast)]
#[derive(Cast, Flip)] // to implement trait Cast and trait Flip
struct UdpHdr {  // UDP: See https://www.rfc-editor.org/rfc/rfc768.txt
    sport: u16,  // UDP Source Port
    dport: u16,  // UDP Destination Port
    len:   u16,  // UDP Length in Bytes (header plus data)
    sum:   u16,  // UDP Checksum
}

// Test data: A sample byte representation of the UDP header (8 bytes)
const BYTES1: [u8; 8] = [0xc3, 0xc9, 0x00, 0x35, 0x00, 0x32, 0x82, 0x3f];

fn my_main() -> Option<()> {
    //
    // Step 2: Encast a byte representation of the UDP header in
    // big-endian stored in const `BYTES1` as a value of struct
    // `UdpHdr` in native-endian and save it to variable `udp_hdr2`.
    //
    let udp_hdr2: UdpHdr = BYTES1.encastf(BE)?;

    // Check if all fields in variable `udp_hdr2` are as expected.
    assert_eq!(udp_hdr2.sport, 0xc3c9); // = 50121 (Ephemeral Port)
    assert_eq!(udp_hdr2.dport, 0x0035); // = 53 (DNS Port)
    assert_eq!(udp_hdr2.len,   0x0032); // = 50 (Length in Bytes)
    assert_eq!(udp_hdr2.sum,   0x823f); // = 0x823f (Checksum)

    Some(())
}

fn main() { my_main(); }
```

To use this crate, add the following lines to your `Cargo.toml`:

```toml
[dependencies.castflip]
version = "0.1"
```

[RFC768]: https://www.rfc-editor.org/rfc/rfc768.txt
[UDP]: https://en.wikipedia.org/wiki/User_Datagram_Protocol
