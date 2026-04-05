Crate castflip_derive provides three derive macros to implement the
basic traits of [crate castflip] for their supported types.

# Crate castflip

Crate castflip is a Rust library for encoding and decoding numeric
variables, arrays and structures.  It provides methods to convert
between a byte representation of a fixed format and a value of a Rust
type with endianness handling.

More precisely, crate castflip provides several traits

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

[crate castflip]: https://docs.rs/castflip/0.1
[documentation]: https://docs.rs/castflip/0.1
