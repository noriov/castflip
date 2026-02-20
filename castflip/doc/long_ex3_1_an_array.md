How to convert between bytes and a value of type `[T; N]`

The example below encasts a byte representation of type `[T; N]` in
big-endian as a value of the type in native-endian, then decasts a
value of type `[T; N]` in native-endian as a byte representation of
the type in big-endian.

# Outline

- Step 1: A tuple `struct` type is defined.  It is aliased to `T` here.
    - It implements trait [`Cast`] by applying both attribute
      `#[`[`derive(Cast)`]`]` and attribute `#[`[`repr(C)`]`]` to it.
    - It implements trait [`Flip`] by applying attribute
      `#[`[`derive(Flip)`]`]` to it.

- Step 2: Method [`EncastMem::encastf`] encasts a byte
  representation of type `[T; N]` in big-endian ([`BE`]) as a values
  of the type in native-endian and returns the resulting value in
  `Ok([T; N])`.

- Step 3: Method [`DecastMem::decastf`] decasts a value of
  type `[T; N]` in native-endian as a byte representation of the type
  in big-endian ([`BE`]) and saves the resulting bytes to `self`.

# Source Code

```rust
use castflip::{BE, Cast, DecastMem, EncastMem, Flip};

//
// Step 1: Define struct `Point` and test data.
//
#[repr(C)]            // to make it possible to apply #[derive(Cast)]
#[derive(Cast, Flip)] // to implement trait Cast and trait Flip
#[derive(Debug, PartialEq)] // to use assert_eq!
struct Point (u16, u16); // 4 bytes (in total)

// Test data: A sample value of `[Point; 3]`
const VALUE1: [Point; 3] = [ // 12 bytes (in total)
    Point(0x2021, 0x2223),   //  4 bytes
    Point(0x2425, 0x2627),   //  4 bytes
    Point(0x2829, 0x2a2b),   //  4 bytes
];

// A sample byte representation of the value above (12 bytes in big-endian)
const BYTES1: [u8; 12] = [
    0x20, 0x21, 0x22, 0x23,  // = Point(0x2021, 0x2223)
    0x24, 0x25, 0x26, 0x27,  // = Point(0x2425, 0x2627)
    0x28, 0x29, 0x2a, 0x2b,  // = Point(0x2829, 0x2a2b)
];

fn main() {
    //
    // Step 2: Method encastf (1) encasts a byte representation of
    // type [Point; 3] at the head of const `BYTES1` as a value of the type,
    // (2) flips the endianness of three pairs of 16-bit unsigned integers
    // in the value from the big-endianness (`BE`) to the native-endianness,
    // and (3) returns the resulting value in Ok([Point; 3]) which is saved
    // to variable `value2`.
    //
    // In the following method call, generic argument `[Point; 3]`
    // can be omitted because it can be infered by the Rust compiler.
    //
    let value2: [Point; 3] = BYTES1.encastf::<[Point; 3]>(BE).unwrap();

    // Check if the content of variable `value2` is as expected.
    assert_eq!(value2, VALUE1);

    //
    // Step 3: Method decastf (1) decasts a value in const
    // `VALUE1` of type [Point; 3] as a byte representation of the type,
    // (2) flips the endianness of three pairs of 16-bit unsigned integers
    // in the bytes from the native-endianness to the big-endianness (`BE`),
    // (3) saves the resulting bytes to `self`, i.e., variable `bytes3`,
    // and (4) returns the number of the resulting bytes in Ok(usize)
    // which is saved to variable `size3`.
    //
    // In the following method call, generic argument `[Point; 3]`
    // can be omitted because it can be infered by the Rust compiler.
    //
    let mut bytes3 = [0_u8; 12]; // The size of type `[Point; 3]` is 12.
    let size3 = bytes3.decastf::<[Point; 3]>(&VALUE1, BE).unwrap();

    // Check if the value in variable `size3` is as expected.
    assert_eq!(size3, 12); // The size of type `[Point; 3]` is 12.

    // Check if the content of variable `bytes3` is as expected.
    assert_eq!(bytes3, BYTES1);
}
```

[`derive(Cast)`]: ../../derive.Cast.html
[`derive(Flip)`]: ../../derive.Flip.html

[`repr(C)`]: https://doc.rust-lang.org/reference/type-layout.html#the-c-representation
