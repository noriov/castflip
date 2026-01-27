I/O Example: A `struct` type is encasted[^encast] and
decasted[^decast] with its endianness flipped as required using trait
[`std::io::Read`] and trait [`std::io::Write`].

[^encast]: In this crate, to *encast* means to cast a byte
representation of a type as a value of the type.

[^decast]: In this crate, to *decast* means to cast a value of a type
as a byte representation of the type.

The following two examples are shown.

1. [A byte stream provided through struct `std::io::Cursor`](#1-a-byte-stream-provided-through-struct-stdiocursor)
2. [A byte stream provided through a byte slice using `std::io`](#2-a-byte-stream-provided-through-a-byte-slice-using-stdio)

# 1. A Byte Stream Provided Through Struct [`std::io::Cursor`]

The example below encasts a byte representation of the UDP[^UDP]
header in big-endian read from struct [`std::io::Cursor`] as a value
of struct `UdpHdr` in native-endian then decasts a value of struct
`UdpHdr` in native-endian as a byte representation of the UDP[^UDP]
header in big-endian which is written to struct [`std::io::Cursor`].

[^UDP]: The User Datagram Protocol ([UDP]) is one of the fundamental
protocols of the Internet protocol suite.  It is defined in [RFC768].
It is exhcanged in big-endian on the Internet.

Note that struct [`std::io::Cursor`] wraps an in-memory buffer and
provides the buffer using trait [`std::io::Read`] and trait
[`std::io::Write`].

## Outline

- Step 1: Struct `UdpHdr` is defined.
  - It implements trait [`Cast`] by applying both attribute
    `#[`[`derive(Cast)`]`]` and attribute `#[`[`repr(C)`]`]` to it.
  - It implements trait [`Flip`] by applying attribute
    `#[`[`derive(Flip)`]`]` to it.

- Step 2: Method [`EncastIO::encastf`] encasts a byte
  representation of the UDP header in big-endian read from struct
  [`std::io::Cursor`] as a value of struct `UdpHdr` in native-endian.

- Step 3: Method [`DecastIO::decastf`] decasts a value of
  struct `UdpHdr` in native-endian as a byte representation of the UDP
  header in big-endian and writes it to struct [`std::io::Cursor`].

- Step 4: Method [`EncastIO::encastf`] encasts a byte
  representation of the UDP header in big-endian read from struct
  [`std::io::Cursor`] as four 16-bit unsigned integers in
  native-endian.

- Step 5: Method [`DecastIO::decastf`] decasts four 16-bit
  unsigned integers in native-endian as a byte representation of the
  UDP header in big-endian and writes it to struct [`std::io::Cursor`].

## Source Code

```rust
use castflip::{BE, Cast, DecastIO, EncastIO, Flip};
use std::io::Cursor;

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

fn main() -> std::io::Result<()> {
    //
    // Step 2: Method encastf (1) encasts a byte representation of
    // the UDP header read from struct `Cursor` as a value of struct
    // `UdpHdr`, (2) flips the endianness of four 16-bit unsigned integers
    // in the value from the big-endianness to the native-endianness, and
    // (3) returns the resulting value in Ok(UdpHdr) which is saved to
    // variable `udp_hdr2`.
    //
    // In the following method call, (a) generic argument `UdpHdr`
    // can be omitted because it can be infered by the Rust compiler,
    // and (b) argument `BE` stands for the Big-Endianness.
    //
    let mut input2 = Cursor::new(BYTES1);
    let udp_hdr2: UdpHdr = input2.encastf::<UdpHdr>(BE)?;

    // Check if the current position of struct `Cursor` is as expected.
    assert_eq!(input2.position(), 8); // The size of the UDP header is 8.

    // Check if the content of variable `udp_hdr2` is as expected.
    assert_eq!(udp_hdr2, UDP_HDR1);

    //
    // Step 3: Method decastf (1) decasts a value of struct `UdpHdr`
    // stored in const `UDP_HDR1` as a byte representation of the UDP header,
    // (2) flips the endianness of four 16-bit unsigned integers in the bytes
    // from the native-endianness to the big-endianness, (3) writes the
    // resulting bytes to struct `Cursor`, and (4) returns the number
    // of the resulting bytes in Ok(usize) which is saved to variable `size3`.
    //
    // In the following method call, (a) generic argument `UdpHdr`
    // can be omitted because it can be infered by the Rust compiler,
    // and (b) argument `BE` stands for the Big-Endianness.
    //
    let mut output3 = Cursor::new(vec![0_u8; 16]);
    let size3 = output3.decastf::<UdpHdr>(&UDP_HDR1, BE)?;

    // Check if the current position of struct `Cursor` is as expected.
    assert_eq!(output3.position(), 8); // The size of the UDP header is 8.

    // Check if the value in variable `size3` is as expected.
    assert_eq!(size3, 8); // The size of the UDP header is 8.

    // Check if the content of struct `Cursor` is as expected.
    let bytes3 = output3.into_inner();
    assert_eq!(bytes3[0..8], BYTES1[0..8]); // The UDP header is stored.
    assert_eq!(bytes3[8..16], [0_u8; 8]);   // Zero because not changed.

    //
    // Step 4: Encast a byte representation of the UDP header in big-endian
    // read from struct `Cursor` as four 16-bit unsigned integers in
    // native-endian and save them in variable `sport4`, `dport4`, `len4`
    // and `sum4`.
    //
    // In the following method call, (a) generic argument `u16`
    // can be omitted because it can be infered by the Rust compiler,
    // and (b) argument `BE` stands for the Big-Endianness.
    //
    let mut input4 = Cursor::new(BYTES1);
    let sport4: u16 = input4.encastf::<u16>(BE)?;
    let dport4: u16 = input4.encastf::<u16>(BE)?;
    let len4:   u16 = input4.encastf::<u16>(BE)?;
    let sum4:   u16 = input4.encastf::<u16>(BE)?;

    // Check if the current position of struct `Cursor` is as expected.
    assert_eq!(input4.position(), 8); // The size of the UDP header is 8.

    // Check if the contents of variable `sport4`, `dport4`, `len4` and `sum4`
    // are as expected.
    assert_eq!(sport4, UDP_HDR1.sport); // = 50121 (Ephemeral Port)
    assert_eq!(dport4, UDP_HDR1.dport); // = 53 (DNS Port)
    assert_eq!(len4,   UDP_HDR1.len);   // = 50 (Length in Bytes)
    assert_eq!(sum4,   UDP_HDR1.sum);   // = 0x823f (Checksum)

    //
    // Step 5: Decast four 16-bit unsigned integers in native-endian in
    // variables `sport4`, `dport4`, `len4` and `sum4` as a byte representation
    // of the UDP header in big-endian and write it to struct `Cursor`.
    //
    // In the following method call, (a) generic argument `u16`
    // can be omitted because it can be infered by the Rust compiler,
    // and (b) argument `BE` stands for the Big-Endianness.
    //
    let mut output5 = Cursor::new(vec![0_u8; 16]);
    let sport_size5 = output5.decastf::<u16>(&sport4, BE)?;
    let dport_size5 = output5.decastf::<u16>(&dport4, BE)?;
    let len_size5   = output5.decastf::<u16>(&len4,   BE)?;
    let sum_size5   = output5.decastf::<u16>(&sum4,   BE)?;

    // Check if the current position of struct `Cursor` is as expected.
    assert_eq!(output5.position(), 8); // The size of the UDP header is 8.

    // Check if the number of written bytes in variables `sport_size5`,
    // `dport_size5`, `len_size5` and `sum_size5` are as expected.
    assert_eq!(sport_size5, 2); // The size of u16 is 2.
    assert_eq!(dport_size5, 2); // The size of u16 is 2.
    assert_eq!(len_size5,   2); // The size of u16 is 2.
    assert_eq!(sum_size5,   2); // The size of u16 is 2.

    // Check if the content of struct `Cursor` is as expected.
    let bytes5 = output5.into_inner();
    assert_eq!(bytes5[0..8], BYTES1[0..8]); // The UDP header is stored.
    assert_eq!(bytes5[8..16], [0_u8; 8]);   // Zero because not changed.

    Ok(())
}
```

# 2. A Byte Stream Provided Through A Byte Slice Using [`std::io`]

Because `&[u8]` implements trait [`std::io::Read`], the methods of
trait [`EncastIO`] can encast in-memory byte representations as values
through a slice of type `&[u8]`.  Because `&mut [u8]` implements trait
[`std::io::Write`], the methods of trait [`DecastIO`] can decast
values as in-memory byte representations through a slice of type `&mut
[u8]`.

The example below is almost the same with [the first
example](#1-a-byte-stream-provided-through-struct-stdiocursor) except
that it uses a slice instead of struct [`std::io::Cursor`].

## Source Code

```rust
use castflip::{BE, Cast, DecastIO, EncastIO, Flip};

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

fn main() -> std::io::Result<()> {
    //
    // Step 2: Method encastf (1) encasts a byte representation of
    // the UDP header at the head of variable `slice2` as a value of struct
    // `UdpHdr`, (2) flips the endianness of four 16-bit unsigned integers
    // in the value from the big-endianness to the native-endianness, and
    // (3) returns the resulting value in Ok(UdpHdr) which is saved to
    // variable `udp_hdr2`.
    //
    // In the following method call, (a) generic argument `UdpHdr`
    // can be omitted because it can be infered by the Rust compiler,
    // and (b) argument `BE` stands for the Big-Endianness.
    //
    let mut slice2 = &BYTES1[..];
    let udp_hdr2: UdpHdr = slice2.encastf::<UdpHdr>(BE)?;

    // Check if the content of variable `slice2` is as expected.
    assert_eq!(slice2.len(), 8); // The size of the UDP header is 8.
    assert_eq!(slice2, &BYTES1[8..]);

    // Check if the content of variable `udp_hdr2` is as expected.
    assert_eq!(udp_hdr2, UDP_HDR1);

    //
    // Step 3: Method decastf (1) decasts a value of struct `UdpHdr`
    // stored in const `UDP_HDR1` as a byte representation of the UDP header,
    // (2) flips the endianness of four 16-bit unsigned integers in the bytes
    // from the native-endianness to the big-endianness, (3) saves the
    // resulting bytes at the head of `self`, i.e., at the head of variable
    // `slice3`, and (4) returns the number of the resulting bytes in
    // Ok(usize) which is saved to variable `size3`.
    //
    // In the following method call, (a) generic argument `UdpHdr`
    // can be omitted because it can be infered by the Rust compiler,
    // and (b) argument `BE` stands for the Big-Endianness.
    //
    let mut bytes3 = [0_u8; 16];
    let mut slice3 = &mut bytes3[..];
    let size3 = slice3.decastf::<UdpHdr>(&UDP_HDR1, BE)?;

    // Check if the content of variable `slice3` is as expected.
    assert_eq!(slice3.len(), 8); // The size of the UDP header is 8.
    assert_eq!(slice3, &[0_u8; 8]);

    // Check if the value in variable `size3` is as expected.
    assert_eq!(size3, 8); // The size of the UDP header is 8.

    // Check if the content of variable `bytes3` is as expected.
    assert_eq!(bytes3[0..8], BYTES1[0..8]); // The UDP header is stored.
    assert_eq!(bytes3[8..16], [0_u8; 8]);   // Zero because not changed.

    //
    // Step 4: Encast a byte representation of the UDP header in big-endian
    // at the head of variable `slice2` as four 16-bit unsigned integers in
    // native-endian and save them in variable `sport4`, `dport4`, `len4`
    // and `sum4`.
    //
    // In the following method call, (a) generic argument `u16`
    // can be omitted because it can be infered by the Rust compiler,
    // and (b) argument `BE` stands for the Big-Endianness.
    //
    let mut slice4 = &BYTES1[..];
    let sport4: u16 = slice4.encastf::<u16>(BE)?;
    let dport4: u16 = slice4.encastf::<u16>(BE)?;
    let len4:   u16 = slice4.encastf::<u16>(BE)?;
    let sum4:   u16 = slice4.encastf::<u16>(BE)?;

    // Check if the content of variable `slice4` is as expected.
    assert_eq!(slice4.len(), 8); // The size of the UDP header is 8.
    assert_eq!(slice4, &BYTES1[8..]);

    // Check if the contents of variable `sport4`, `dport4`, `len4` and `sum4`
    // are as expected.
    assert_eq!(sport4, UDP_HDR1.sport); // = 50121 (Ephemeral Port)
    assert_eq!(dport4, UDP_HDR1.dport); // = 53 (DNS Port)
    assert_eq!(len4,   UDP_HDR1.len);   // = 50 (Length in Bytes)
    assert_eq!(sum4,   UDP_HDR1.sum);   // = 0x823f (Checksum)

    //
    // Step 5: Decast four 16-bit unsigned integers in native-endian in
    // variables `sport4`, `dport4`, `len4` and `sum4` as a byte representation
    // of the UDP header in big-endian and write it to variable `slice5`.
    //
    // In the following method call, (a) generic argument `u16`
    // can be omitted because it can be infered by the Rust compiler,
    // and (b) argument `BE` stands for the Big-Endianness.
    //
    let mut bytes5 = [0_u8; 16];
    let mut slice5 = &mut bytes5[..];
    let sport_size5 = slice5.decastf::<u16>(&sport4, BE)?;
    let dport_size5 = slice5.decastf::<u16>(&dport4, BE)?;
    let len_size5   = slice5.decastf::<u16>(&len4,   BE)?;
    let sum_size5   = slice5.decastf::<u16>(&sum4,   BE)?;

    // Check if the content of variable `slice5` is as expected.
    assert_eq!(slice5.len(), 8); // The size of the UDP header is 8.
    assert_eq!(slice5, &[0_u8; 8]);

    // Check if the number of written bytes in variables `sport_size5`,
    // `dport_size5`, `len_size5` and `sum_size5` are as expected.
    assert_eq!(sport_size5, 2); // The size of u16 is 2.
    assert_eq!(dport_size5, 2); // The size of u16 is 2.
    assert_eq!(len_size5,   2); // The size of u16 is 2.
    assert_eq!(sum_size5,   2); // The size of u16 is 2.

    // Check if the content of variable `bytes5` is as expected.
    assert_eq!(bytes5[0..8], BYTES1[0..8]); // The UDP header is stored.
    assert_eq!(bytes5[8..16], [0_u8; 8]);   // Zero because not changed.

    Ok(())
}
```

[`derive(Cast)`]: ../../derive.Cast.html
[`derive(Flip)`]: ../../derive.Flip.html

[`repr(C)`]: https://doc.rust-lang.org/reference/type-layout.html#the-c-representation
[RFC768]: https://www.rfc-editor.org/rfc/rfc768.txt
[UDP]: https://en.wikipedia.org/wiki/User_Datagram_Protocol
