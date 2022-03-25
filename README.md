# Castflip

Castflip is a Rust library for encoding and decoding numeric
variables, arrays and structures in little-endian and big-endian.

* Documentation: <https://docs.rs/castflip/>
* License: <https://spdx.org/licenses/MIT-0.html>
* How to use: Add the following line to Cargo.toml:

```
[dependencies]
castflip = { version = "0.1" }
```

Here we show an example below.  For more examples and documentation,
please see <https://docs.rs/castflip/>.

In this crate, the term `encast` means decoding a number of bytes to
one or more values, the term `decast` means encoding one or more
variables to a number of bytes, and the term `endian-flip` means
flipping the endianness of value(s).

# Example 1

In the example below, method `encastf` decodes bytes in `bytes1` in
big-endian ([`BE`]) to variable `udp_hdr2` of type `UdpHdr`.  Then,
method `decastf` encodes the resulting value in `udp_hdr2` to bytes in
big-endian ([`BE`]) and stores them in `bytes3`.

```rust
use castflip::{Cast, Flip, EncastMem, DecastMem, BE};

#[repr(C)]
#[derive(Cast, Flip)]
struct UdpHdr {     // UDP: https://www.rfc-editor.org/rfc/rfc768.txt
    sport:  u16,    // UDP Source Port
    dport:  u16,    // UDP Destination Port
    len:    u16,    // UDP Length
    sum:    u16,    // UDP Checksum
}

// Input data: UDP header (8 bytes)
let bytes1: [u8; 8] = [0xC3, 0xC9, 0x00, 0x35, 0x00, 0x32, 0x82, 0x3F];

// Decode bytes `bytes1` to variable `udp_hdr2`.
let udp_hdr2 = bytes1.encastf::<UdpHdr>(BE)?;  // BE = Big-Endian

// Encode the resulting UDP header `udp_hdr2` to bytes `bytes3`.
let mut bytes3 = [0_u8; 8];
let size3 = bytes3.decastf::<UdpHdr>(&udp_hdr2, BE)?;

// Check the results (udp_hdr2)
assert_eq!(udp_hdr2.sport, 0xC3C9); // = 50121
assert_eq!(udp_hdr2.dport, 0x0035); // = 53 (DNS)
assert_eq!(udp_hdr2.len,   0x0032); // = 50
assert_eq!(udp_hdr2.sum,   0x823F);

// Check the results (bytes3)
assert_eq!(size3, 8);
assert_eq!(bytes3, bytes1);
```

In the example above, `#[derive(`[`Cast`]`)]` makes the value of
`UdpHdr` `encast`able / `decast`able, and `#[derive(`[`Flip`]`)]`
makes the value of `UdpHdr` `endian-flip`pable.

Trait [`EncastMem`] provides methods to `encast` from memory, and
trait [`DecastMem`] provides methods to `decast` to memory.  The
generic type parameters of their methods can be omitted where the Rust
compiler can infer them.  Their methods whose names end with 'f' flip
the endianness of the results.  The endianness of bytes is specified
in their argument.  [`BE`] is an alias of [`Endian`]`::Big`, which
means Big-Endian.

In addition to these traits, this crate has trait [`EncastIO`]
providing methods to `encast` from I/O, and trait [`DecastIO`]
providing methods to `decast` to I/O.

Note: [UDP] is one of the fundamental protocols in the internet
protocol suite.

[UDP]: https://en.wikipedia.org/wiki/User_Datagram_Protocol

For more examples and documentation, please see
<https://docs.rs/castflip/>.


[`Endian`]: https://docs.rs/castflip/latest/castflip/enum.Endian.html
[`NE`]: https://docs.rs/castflip/latest/castflip/constant.NE.html
[`SE`]: https://docs.rs/castflip/latest/castflip/constant.SE.html
[`BE`]: https://docs.rs/castflip/latest/castflip/constant.BE.html
[`LE`]: https://docs.rs/castflip/latest/castflip/constant.LE.html

[`Cast`]: https://docs.rs/castflip/latest/castflip/trait.Cast.html
[`Flip`]: https://docs.rs/castflip/latest/castflip/trait.Flip.html
[`NopFlip`]: https://docs.rs/castflip/latest/castflip/trait.NopFlip.html

[`EncastMem`]: https://docs.rs/castflip/latest/castflip/trait.EncastMem.html
[`DecastMem`]: https://docs.rs/castflip/latest/castflip/trait.DecastMem.html
[`EncastIO`]: https://docs.rs/castflip/latest/castflip/trait.EncastIO.html
[`DecastIO`]: https://docs.rs/castflip/latest/castflip/trait.DecastIO.html
