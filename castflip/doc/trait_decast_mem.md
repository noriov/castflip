Provides methods that decast[^decast] one or more values of a type
specified by a parameter as one or more byte representations of the
type and save the resulting byte representations to the memory at the
head of `self`.

[^decast]: In this crate, to *decast* means to cast a value of a type
as a byte representation of the type.

This trait is implemented for `[u8]`.

# Implementation for `[u8]`

The methods of this trait decast one or more values of a type
specified by a parameter as one or more byte representations of the
type, flip the endianness of the byte representations as required,
save the resulting byte representations to the memory at the head of
`self`, then return the number of the bytes in the byte
representations in [`Some`]`(usize)` if successful.  On failure, they
return [`None`]

The number of the bytes in `self` must be larger than or equal to the
number of the bytes in the byte representations of the type.  If this
condition is satisfied, the resulting byte representations are saved
to the memory at the head of `self`.  The remaining bytes in `self`
are unchanged.

# Method Naming Convention

* The names of all methods start with `decast`.

* The methods whose names end with `f` flips the endianness of the
  resulting byte representations to the endian specified by parameter
  `endian`.

* The methods whose names end with `` or `f` decast a value specified
  by parameter `value`.

* The methods whose names end with `s` or `sf` decast values in the
  slice specified by parameter `slice`.

* The methods whose names end with `v` or `vf` are equivalent to the
  methods whose names end with `s` or `sf` respectively.

For details, see the description of each method.

# Example

The example below decasts a value of struct `UdpHdr` in native-endian
as a byte representation of the UDP[^UDP] header in big-endian.

[^UDP]: The User Datagram Protocol ([UDP]) is one of the fundamental
protocols of the Internet protocol suite.  It is defined in [RFC768].
It is exhcanged in big-endian on the Internet.

- Step 1: Struct `UdpHdr` is defined.
  - It implements trait [`Cast`] by applying both attribute
    `#[`[`derive(Cast)`]`]` and attribute `#[`[`repr(C)`]`]` to it.
  - It implements trait [`Flip`] by applying attribute
    `#[`[`derive(Flip)`]`]` to it.

- Step 2: Method [`DecastMem::decastf`] decasts a value of
  struct `UdpHdr` in native-endian as a byte representation of the
  UDP[^UDP] header in big-endian.

```rust
use castflip::{BE, Cast, DecastMem, Flip};

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

// Test data: A sample UDP header
const UDP_HDR1: UdpHdr = UdpHdr {
    sport: 0xc3c9,  // = 50121 (Ephemeral Port)
    dport: 0x0035,  // = 53 (DNS Port)
    len:   0x0032,  // = 50 (Length in Bytes)
    sum:   0x823f,  // = 0x823f (Checksum)
};

// The byte representation of the UDP header above (8 bytes in big-endian)
const BYTES1: [u8; 8] = [0xc3, 0xc9, 0x00, 0x35, 0x00, 0x32, 0x82, 0x3f];

fn my_main() -> Option<()> {
    //
    // Step 2: Decast a value of struct `UdpHdr` in native-endian
    // stored in const `UDP_HDR1` as a byte representation of the
    // UDP header in big-endian and save it to variable `bytes2`.
    //
    // Because the UDP header is 8 bytes, only the first 8 bytes of
    // variable `bytes2` are filled with data and remaining 8 bytes
    // are unchanged.
    //
    let mut bytes2 = [0_u8; 16];
    let size2 = bytes2.decastf(&UDP_HDR1, BE)?; // BE = Big-Endian

    // Check if the value in variable `size2` is as expected.
    assert_eq!(size2, 8); // The size of the UDP header is 8.

    // Check if the content of variable `bytes2` is as expected.
    assert_eq!(bytes2[0..8], BYTES1[0..8]); // The UDP header is stored.
    assert_eq!(bytes2[8..16], [0_u8; 8]);   // Zero because not changed.

    Some(())
}

fn main() { my_main(); }
```

[`derive(Cast)`]: ./derive.Cast.html
[`derive(Flip)`]: ./derive.Flip.html

[`repr(C)`]: https://doc.rust-lang.org/reference/type-layout.html#the-c-representation
[RFC768]: https://www.rfc-editor.org/rfc/rfc768.txt
[UDP]: https://en.wikipedia.org/wiki/User_Datagram_Protocol
