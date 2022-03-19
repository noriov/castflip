# Castflip

Castflip is a Rust library for encoding and decoding numeric
variables, arrays and structures in little-endian and big-endian.

In this crate, the term `encast` means decoding a number of bytes to
one or more values, the term `decast` means encoding one or more
variables to a number of bytes, and the term `endian-flip` means
flipping the endianness of value(s).

# Example 1

In the example below, method `encastf` decodes bytes in `bytes1` in
big-endian (`BE`) to variable `udp_hdr2` of type `UdpHdr`.  Then,
method `decastf` encodes the resulting value in `udp_hdr2` to bytes in
big-endian (`BE`) and stores them in `bytes3`.

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
let bytes3_size = bytes3.decastf::<UdpHdr>(&udp_hdr2, BE)?;

// Check the results.
assert_eq!(udp_hdr2.sport, 0xC3C9); // = 50121
assert_eq!(udp_hdr2.dport, 0x0035); // = 53 (DNS)
assert_eq!(udp_hdr2.len,   0x0032); // = 50
assert_eq!(udp_hdr2.sum,   0x823F);
assert_eq!(bytes3_size, 8);
assert_eq!(bytes3, bytes1);
```

In the example above, #[derive(`Cast`)] makes the values of `UdpHdr`
`encast`able / `decast`able, and #[derive(`Flip`)] makes the values
of `UdpHdr` `endian-flip`pable.

Trait `EncastMem` provides methods to `encast` from memory, and
trait `DecastMem` provides methods to `decast` to memory.  The
generic type parameters of their methods can be omitted where the Rust
compiler can infer them.  Their methods whose names end with 'f' flip
the endianness of the results.  The endianness of bytes is specified
by their argument.  `BE` is an alias of `Endian::Big`, which
means Big-Endian.

Note: [UDP] is one of the fundamental protocols in the internet
protocol suite.

[UDP]: https://en.wikipedia.org/wiki/User_Datagram_Protocol

# Example 2

Arrays, structures and unions can be nested as in the example below.

```rust
use std::io::Cursor;
use castflip::{Cast, Flip, NopFlip, EncastIO, DecastIO, LE};

#[repr(C)]
#[derive(Cast, Flip)]
struct TypeA {      // 8 bytes (total)
    x: [u8; 2],     // 2 bytes
    y: u16,         // 2 bytes
    z: u32,         // 4 bytes
}

#[repr(C)]
#[derive(Cast, NopFlip)]
union UnionB {      // 4 bytes (largest)
    u: [u8; 4],     // 4 bytes
    v: [u16; 2],    // 4 bytes
    w: u32,         // 4 bytes
}

#[repr(C)]
#[derive(Cast, Flip)]
struct TypeC {      // 16 bytes (total)
    a: TypeA,       //  8 bytes
    b: UnionB,      //  4 bytes
    f: f32,         //  4 bytes
}

// Input data (16 bytes)
let bytes1: [u8; 16] = [0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
                        0x18, 0x19, 0x1A, 0x1B, 0x00, 0x00, 0x48, 0x41];
let mut input1 = Cursor::new(bytes1);

// Decode input `input1` to variable `var2_c` of type `TypeC`.
let var2_c: TypeC = input1.encastf(LE)?;  // LE = Little-Endian

// Encode variable `var2_c` to bytes and write them to `output3`.
let mut output3 = Cursor::new(vec![0_u8; 16]);
output3.decastf(&var2_c, LE)?;

// Check the results (TypeA in TypeC)
assert_eq!(var2_c.a.x, [0x10_u8, 0x11]);
assert_eq!(var2_c.a.y, 0x1312);
assert_eq!(var2_c.a.z, 0x17161514);

// Check the results (UnionB in TypeC)
unsafe {
    assert_eq!(var2_c.b.u, [0x18_u8, 0x19, 0x1A, 0x1B]);
    if cfg!(target_endian = "little") {
        assert_eq!(var2_c.b.v, [0x1918_u16, 0x1B1A]);
        assert_eq!(var2_c.b.w, 0x1B1A1918);
    } else if cfg!(target_endian = "big") {
        assert_eq!(var2_c.b.v, [0x1819_u16, 0x1A1B]);
        assert_eq!(var2_c.b.w, 0x18191A1B);
    }
}

// Check the result (f32 in TypeC)
assert_eq!(var2_c.f, 12.5_f32);

// Check the result (output3)
assert_eq!(&output3.into_inner(), &bytes1[..]);
```

In the example above, #[derive(`Cast`)] makes all types
`encast`able / `decast`able, #[derive(`Flip`)] makes `TypeA`
and `TypeC` `endian-flip`pable, and #[derive(`NopFlip`) marks
`UnionB` as `endian-flip`pable but the implemented operation is
Nop (No operation).

Trait `EncastIO` provides methods to `encast` from I/O, and trait
`DecastIO` provides methods to `decast` to I/O.  The generic type
parameters of their methods can be omitted where the Rust compiler can
infer them as in the example above.  Their methods whose names end
with 'f' flip the endianness of the results.  The endianness of bytes
is specified by their argument.  `LE` is an alias of
`Endian::Little`, which means Little-Endian.

# Example 3

A series of structured binary data can be `encast`ed / `decast`ed as
in the example below.

```rust
use castflip::{Cast, Flip, EncastMem, DecastMem, BE};

#[repr(C)]
#[derive(Cast, Flip, PartialEq, Debug)]
struct Pair (u16, u16);

// Input data.
let bytes1: [u8; 16] = [0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27,
                        0x28, 0x29, 0x2A, 0x2B, 0x2C, 0x2D, 0x2E, 0x2F];

// Decode bytes `bytes1` to variable `vec2`.
let vec2: Vec<Pair> = bytes1.encastvf(4, BE)?;  // BE = Big-Endian

// Encode variable `vec2` to bytes `bytes3`.
let mut bytes3 = [0_u8; 16];
let bytes3_size = bytes3.decastvf(&vec2, BE)?;

// Check the results.
assert_eq!(vec2, vec![Pair(0x2021, 0x2223), Pair(0x2425, 0x2627),
                      Pair(0x2829, 0x2A2B), Pair(0x2C2D, 0x2E2F)]);
assert_eq!(bytes3_size, 16);
assert_eq!(bytes3, bytes1);
```

In the example above, #[derive(`Cast`)] makes the values of `Pair`
`encast`able / `decast`able, and #[derive(`Flip`)] makes the values
of `Pair` `endian-flip`pable.

Trait `EncastMem` provides methods to `encast` from memory, and
trait `DecastMem` provides methods to `decast` to memory.  The
generic type parameters of their methods can be omitted where the Rust
compiler can infer them as in the example above.  The methods whose
name contain 'v' (= vector) process a series of structured binary
data, and the methods whose names end with 'f' flip the endianness of
the results.  `BE` is an alias of `Endian::Big`, which means
Big-Endian.

Note: The reason why #[derive(PartialEq, Debug)] is declared is that
assert_eq! requires them.  This crate works without them.
