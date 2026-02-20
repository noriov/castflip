Provides methods that encast[^encast] one or more byte representations
of a type read from reader `self` using trait [`std::io::Read`] as one
or more values of the type and return the resulting values.

[^encast]: In this crate, to *encast* means to cast a byte
representation of a type as a value of the type.

This trait is blanketly implemented for those types that implement
trait [`std::io::Read`].

Note that the last letter of the trait name is an uppercase "`O`" (not
an lowercase "`o`").

# Implementation for The Types Implementing Trait [`std::io::Read`].

The methods of this trait encast one or more byte representations of a
type read from reader `self` using trait [`std::io::Read`] as one or
more values of the type, flip the endianness of the values as
required, then return the resulting values in [`Ok`] if successful.
On failure, they return an error value of struct [`std::io::Error`] in
[`Err`].

The number of the bytes that can be read from reader `self` must be
larger than or equal to the number of the bytes in the byte
representations of the type.  If this condition is satisfied, the
required number of the bytes is read from reader `self` and is
encasted as values.  The remaining bytes are not read.

# Method Naming Convention

* The names of all methods start with `encast`.

* The methods whose names end with `f` flips the endianness of the
  resulting values to the native-endianness on the assumption that the
  endianness of the source bytes is correctly specified by parameter
  `endian`.

* The methods whose names end with `` or `f` return the resulting
  value.

* The methods whose names end with `s` or `sf` write the resulting
  values to the slice specified by parameter `slice`.

* The methods whose names end with `v` or `vf` return the resulting
  values in struct `Vec<T>`.

For details, see the description of each method.

# Example

The example below encasts a byte representation of the UDP[^UDP]
header in big-endian as a value of struct `UdpHdr` in native-endian.

[^UDP]: The User Datagram Protocol ([UDP]) is one of the fundamental
protocols of the Internet protocol suite.  It is defined in [RFC768].
It is exhcanged in big-endian on the Internet.

- Step 1: Struct `UdpHdr` is defined.
    - It implements trait [`Cast`] by applying both attribute
      `#[`[`derive(Cast)`]`]` and attribute `#[`[`repr(C)`]`]` to it.
    - It implements trait [`Flip`] by applying attribute
      `#[`[`derive(Flip)`]`]` to it.

- Step 2: Method[`EncastIO::encastf`] encasts a byte
  representation of the UDP header in big-endian ([`BE`]) as a value
  of struct `UdpHdr` in native-endian.

The input byte representation is read using struct [`std::io::Cursor`]
which wraps an in-memory buffer and provides it through trait
[`std::io::Read`].

```rust
# fn main() {
use castflip::{BE, Cast, EncastIO, Flip};
use std::io::Cursor;

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
// (`BE`) read from variable `in_cursor` as a value of struct `UdpHdr` in
// native-endian and save it to variable `out_hdr`.
//
// Because the UDP header is 8 bytes, only the first 8 bytes are read
// and remaining 8 bytes are not read.
//

// Input: The UDP header (8 bytes) + part of the DNS header (8 bytes)
let in_bytes: [u8; 16] = [
    0xc3, 0xc9, 0x00, 0x35, 0x00, 0x32, 0x82, 0x3f,
    0x1a, 0xd1, 0x01, 0x20, 0x00, 0x01, 0x00, 0x00,
];

// Encast a byte representation in big-endian (`BE`) as a value.
let mut in_cursor = Cursor::new(in_bytes);
let out_hdr: UdpHdr = in_cursor.encastf(BE).unwrap();

// Check if the current position of variable `in_cursor` is as expected.
assert_eq!(in_cursor.position(), 8); // The size of the UDP header is 8.

// Check if all fields in variable `out_hdr` are as expected.
assert_eq!(out_hdr.sport, 0xc3c9);  // = 50121 (Ephemeral Port)
assert_eq!(out_hdr.dport, 0x0035);  // = 53 (DNS Port)
assert_eq!(out_hdr.len,   0x0032);  // = 50 (Length in Bytes)
assert_eq!(out_hdr.sum,   0x823f);  // = 0x823f (Checksum)
# }
```

[`derive(Cast)`]: ./derive.Cast.html
[`derive(Flip)`]: ./derive.Flip.html

[`repr(C)`]: https://doc.rust-lang.org/reference/type-layout.html#the-c-representation
[RFC768]: https://www.rfc-editor.org/rfc/rfc768.txt
[UDP]: https://en.wikipedia.org/wiki/User_Datagram_Protocol
