Crate castflip_derive provides three derive macros for crate castflip.
They simplifies to implement the fundamental traits of crate castflip
for the supported types.

# Crate castflip

Crate castflip is a Rust library for encoding and decoding numeric
variables, arrays and structures in little-endian and big-endian.
It provides methods to convert between a byte representation of a
format and a value of a Rust type with automatic endian handling.

More precisely, crate castflip provides several traits

- to *encast* a byte representation of a type as a value of the type,
- to *decast* a value of a type as a byte representation of the type, and
- to *flip* the endianness of a value of a type as required.

The supported types include

1. primitive numeric types, and
2. array types, `struct` types and `union` types consisting of the
   supported types.

For more information, please see the [documentation] of crate castflip.
It includes some examples and summaries as well as the descriptions of
its types and its traits.

[documentation]: https://docs.rs/castflip/0.1/castflip/
