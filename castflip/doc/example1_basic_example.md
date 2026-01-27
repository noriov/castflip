Basic Example: A `struct` type is encasted[^encast] and
decasted[^decast] with its endianness flipped as required.

[^encast]: In this crate, to *encast* means to cast a byte
representation of a type as a value of the type.

[^decast]: In this crate, to *decast* means to cast a value of a type
as a byte representation of the type.

The example below encasts a byte representation of the UDP[^UDP]
header in big-endian as a value of struct `UdpHdr` in native-endian
then decasts a value of struct `UdpHdr` in native-endian as a byte
representation of the UDP header in big-endian.

[^UDP]: The User Datagram Protocol ([UDP]) is one of the fundamental
protocols of the Internet protocol suite.  It is defined in [RFC768].
It is exhcanged in big-endian on the Internet.

# Outline

- Step 1: Struct `UdpHdr` is defined.
  - It implements trait [`Cast`] by applying both attribute
    `#[`[`derive(Cast)`]`]` and attribute `#[`[`repr(C)`]`]` to it.
  - It implements trait [`Flip`] by applying attribute
    `#[`[`derive(Flip)`]`]` to it.

- Step 2: Method [`EncastMem::encastf`] encasts a byte
  representation of the UDP header in big-endian as a value of struct
  `UdpHdr` in native-endian.

- Step 3: Method [`DecastMem::decastf`] decasts a value of
  struct `UdpHdr` in native-endian as a byte representation of the UDP
  header in big-endian.

# Source Code

```rust
use castflip::{BE, Cast, DecastMem, EncastMem, Flip};

//
// Step 1: Define struct `UdpHdr` and test data.
//
#[repr(C)]            // to make it possible to apply #[derive(Cast)]
#[derive(Cast, Flip)] // to implement trait Cast and trait Flip
#[derive(Debug, PartialEq)] // to use assert_eq!
struct UdpHdr {  // UDP: See https://www.rfc-editor.org/rfc/rfc768.txt
    sport: u16,  // UDP Source Port
    dport: u16,  // UDP Destination Port
    len:   u16,  // UDP Length in Bytes (header plus data)
    sum:   u16,  // UDP Checksum
}

// Test data: A sample UDP header
const UDP_HDR1: UdpHdr = UdpHdr {
    sport: 0xc3c9,  // = 50121 (Ephemeral Port)
    dport: 0x0035,  // = 53 (DNS Port)
    len:   0x0032,  // = 50 (Length in Bytes)
    sum:   0x823f,  // = 0x823f (Checksum)
};

// Test data: The UDP header (8 bytes) + part of the DNS header (8 bytes)
const BYTES1: [u8; 16] = [
    0xc3, 0xc9, 0x00, 0x35, 0x00, 0x32, 0x82, 0x3f,
    0x1a, 0xd1, 0x01, 0x20, 0x00, 0x01, 0x00, 0x00,
];

fn my_main() -> Option<()> {
    //
    // Step 2: Method encastf (1) encasts a byte representation of
    // the UDP header at the head of const `BYTES1` as a value of struct
    // `UdpHdr`, (2) flips the endianness of four 16-bit unsigned integers
    // in the value from the big-endianness to the native-endianness, and
    // (3) returns the resulting value in Ok(UdpHdr) which is saved to
    // variable `udp_hdr2`.
    //
    // In the following method call, (a) generic argument `UdpHdr`
    // can be omitted because it can be infered by the Rust compiler,
    // and (b) argument `BE` stands for the Big-Endianness.
    //
    let udp_hdr2: UdpHdr = BYTES1.encastf::<UdpHdr>(BE)?;

    // Check if the content of variable `udp_hdr2` is as expected.
    assert_eq!(udp_hdr2, UDP_HDR1);

    //
    // Step 3: Method decastf (1) decasts a value of struct `UdpHdr`
    // stored in const `UDP_HDR1` as a byte representation of the UDP header,
    // (2) flips the endianness of four 16-bit unsigned integers in the bytes
    // from the native-endianness to the big-endianness, (3) saves the
    // resulting bytes at the head of `self`, i.e., at the head of variable
    // `bytes3`, and (4) returns the number of the resulting bytes in
    // Ok(usize) which is saved to variable `size3`.
    //
    // In the following method call, (a) generic argument `UdpHdr`
    // can be omitted because it can be infered by the Rust compiler,
    // and (b) argument `BE` stands for the Big-Endianness.
    //
    let mut bytes3 = [0_u8; 16];
    let size3 = bytes3.decastf::<UdpHdr>(&UDP_HDR1, BE)?;

    // Check if the value in variable `size3` is as expected.
    assert_eq!(size3, 8); // The size of the UDP header is 8.

    // Check if the content of variable `bytes3` is as expected.
    assert_eq!(bytes3[0..8], BYTES1[0..8]); // The UDP header is stored.
    assert_eq!(bytes3[8..16], [0_u8; 8]);   // Zero because not changed.

    Some(())
}

fn main() { my_main(); }
```

[`derive(Cast)`]: ../../derive.Cast.html
[`derive(Flip)`]: ../../derive.Flip.html

[`repr(C)`]: https://doc.rust-lang.org/reference/type-layout.html#the-c-representation
[RFC768]: https://www.rfc-editor.org/rfc/rfc768.txt
[UDP]: https://en.wikipedia.org/wiki/User_Datagram_Protocol
