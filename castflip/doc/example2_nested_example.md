Nested Example: A nested `struct` type is encasted[^encast] and
decasted[^decast] with its endianness flipped as required.

[^encast]: In this crate, to *encast* means to cast a byte
representation of a type as a value of the type.

[^decast]: In this crate, to *decast* means to cast a value of a type
as a byte representation of the type.

The example below encasts a byte representation of nested struct
`Container` in little-endian as a value of struct `Container` in
native-endian then decasts the value as a byte representation of
struct `Container` in little-endian.

# Outline

- Step 1: Struct `Container`, struct `FieldS` and union `FieldU` are
  defined.
  - They implement trait [`Cast`] by applying both attribute
    `#[`[`derive(Cast)`]`]` and attribute `#[`[`repr(C)`]`]` to them.
  - Struct `Container` and struct `FieldS` implement trait [`Flip`] by
    applying attribute `#[`[`derive(Flip)`]`]` to them.
  - Union `FieldU` implements both trait [`Flip`] whose methods do
    nothing and trait [`NopFlip`] by applying attribute
    `#[`[`derive(NopFlip)`]`]` to it.

- Step 2: Method [`EncastMem::encastf`] encasts a byte
  representation in little-endian as a value of struct `Container` in
  native-endian.

- Step 3: Method [`DecastMem::decastf`] decasts a value of
  struct `Container` in native-endian as a byte representation in
  little-endian.

# Source Code

```rust
use castflip::{Cast, DecastMem, EncastMem, Flip, LE, NopFlip};

//
// Step 1: Define nested struct `Container` and test data.
//
#[repr(C)]            // to make it possible to apply #[derive(Cast)]
#[derive(Cast, Flip)] // to implement trait Cast and trait Flip
struct Container {  // 16 bytes (in total)
    s: FieldS,      //  8 bytes
    u: FieldU,      //  4 bytes
    f: f32,         //  4 bytes
}

#[repr(C)]            // to make it possible to apply #[derive(Cast)]
#[derive(Cast, Flip)] // to implement trait Cast and trait Flip
struct FieldS {     // 8 bytes (in total)
    s1: [u8; 2],    // 2 bytes
    s2: u16,        // 2 bytes
    s3: u32,        // 4 bytes
}

#[repr(C)]               // to make it possible to apply #[derive(Cast)]
#[derive(Cast, NopFlip)] // to implement trait Cast, trait Flip & trait NopFlip
union FieldU {      // 4 bytes (the largest)
    u1: [u8; 4],    // 4 bytes
    u2: [u16; 2],   // 4 bytes
    u3: u32,        // 4 bytes
}

// Test data: A sample byte representation of struct `Container`
// (16 bytes in little-endian)
const BYTES1: [u8; 16] = [
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17,
    0x18, 0x19, 0x1a, 0x1b, 0x00, 0x00, 0x48, 0x41,
];

fn my_main() -> Option<()> {
    //
    // Step 2: Method encastf (1) encasts a byte representation of
    // struct `Container` at the head of const `BYTES1` as a value of the
    // type, (2) flips the endiannesses of field `s` of struct `FieldS` and
    // field `f` of f32 from the little-endianness to the native-endianness
    // (Note: field `u` of union `FieldU` is not flipped), and (3) returns
    // the resulting value in Ok(Container) which is saved to variable `con2`.
    //
    // In the following method call, (a) generic argument `Container`
    // can be omitted because it can be infered by the Rust compiler,
    // and (b) argument `LE` stands for the Little-Endianness.
    //
    let con2: Container = BYTES1.encastf::<Container>(LE)?;

    // Check if the values of all fields in variable `con2.s` (whose type
    // is struct `FieldS` in struct `Container`) are as expected.
    assert_eq!(con2.s.s1, [0x10, 0x11]);  // s1: [u8; 2],
    assert_eq!(con2.s.s2, 0x1312);        // s2: u16,
    assert_eq!(con2.s.s3, 0x17161514);    // s3: u32,

    // Check if the values of all fields in variable `con2.u` (whose type
    // is union `FieldU` in struct `Container`) are as expected.
    // Note that their endiannesses must not be changed.
    unsafe {
        assert_eq!(con2.u.u1, [0x18, 0x19, 0x1a, 0x1b]); // u1: [u8; 4],
        if cfg!(target_endian = "little") {
            assert_eq!(con2.u.u2, [0x1918, 0x1b1a]);     // u2: [u16; 2],
            assert_eq!(con2.u.u3, 0x1b1a1918);           // u3: u32,
        } else if cfg!(target_endian = "big") {
            assert_eq!(con2.u.u2, [0x1819, 0x1a1b]);     // u2: [16; 2],
            assert_eq!(con2.u.u3, 0x18191a1b);           // u3: u32,
#       } else {
#           panic!();
        }
    }

    // Check if the value of variable `con2.f` (whose type is f32 in
    // struct `Container`) is as expected.
    assert_eq!(con2.f, 12.5_f32);  // f: f32,

    //
    // Step 3: Method decastf (1) decasts the value of struct
    // `Container` saved in variable `con2` as a byte representation of the
    // type, (2) flips the endiannesses of field `s` of struct `FieldS` and
    // field `f` of f32 from the native-endianness to the little-endianness
    // (Note: field `u` of union `FieldU` is not flipped), (3) saves the
    // resulting bytes to variable `bytes3` and (4) returns the number of
    // the saved bytes in Ok(usize) which is saved in variable `size3`.
    //
    // In the following method call, (a) generic argument `Container`
    // can be omitted because it can be infered by the Rust compiler,
    // and (b) argument `LE` stands for the Little-Endianness.
    //
    let mut bytes3 = [0_u8; 16];
    let size3 = bytes3.decastf::<Container>(&con2, LE)?;

    // Check if the value in variable `size3` is as expected.
    assert_eq!(size3, 16); // The size of struct `Container` is 16.

    // Check if the content of variable `bytes3` is as expected.
    // The endianness of field `u` of union `FieldU` must not be changed.
    assert_eq!(bytes3, BYTES1);

    Some(())
}

fn main() { my_main(); }
```

[`derive(Cast)`]: ../../derive.Cast.html
[`derive(Flip)`]: ../../derive.Flip.html
[`derive(NopFlip)`]: ../../derive.NopFlip.html

[`repr(C)`]: https://doc.rust-lang.org/reference/type-layout.html#the-c-representation
