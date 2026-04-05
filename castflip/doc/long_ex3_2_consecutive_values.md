How to convert between bytes and N-consecutive values of type `T`

The example below encasts N-consecutive byte representations of type
`T` in big-endian as N-consecutive values of the type in native-endian
in two different ways, then decasts N-consecutive values of type `T`
in native-endian as N-consecutive byte representations of the type in
big-endian.

# Outline

- Step 1: A tuple `struct` type is defined.  It is aliased to `T` here.
    - It implements trait [`Cast`] by applying both attribute
      `#[`[`derive(Cast)`]`]` and attribute `#[`[`repr(C)`]`]` to it.
    - It implements trait [`Flip`] by applying attribute
      `#[`[`derive(Flip)`]`]` to it.

- Step 2: Method [`EncastMem::encastsf`] encasts
  N-consecutive byte representations of type `T` in big-endian
  ([`BE`]) at the head of parameter `self` as N-consecutive values of
  type `T` in native-endian, saves the N-consecutive values to the
  specified mutable N-element slice of type `T`, and returns the
  number of the source bytes in [`Ok`]`(usize)`.

- Step 3: Method [`EncastMem::encastvf`] encasts
  N-consecutive byte representations of type `T` in big-endian
  ([`BE`]) at the head of parameter `self` as N-consecutive values of
  type `T` in native-endian, and returns the N-consecutive values in
  [`Ok`]`(Vec<T>)`,

- Step 4: Method [`DecastMem::decastsf`] decasts a slice
  containing N-consecutive values of type `T` in native-endian in a
  parameter as N-consecutive byte representations of type `T` in
  big-endian ([`BE`]), saves the bytes to `self`, and returns the
  number of the bytes in [`Ok`]`(usize)`.

# Source Code

```rust
use castflip::{BE, Cast, DecastMem, EncastMem, Flip};

//
// Step 1: Define struct `Pair` and test data.
//
#[repr(C)]            // to make it possible to apply #[derive(Cast)]
#[derive(Cast, Flip)] // to implement trait Cast and trait Flip
#[derive(Default)]    // to give this type a default value
#[derive(Debug, PartialEq)] // to use assert_eq!
struct Pair (u16, u16); // 4 bytes (in total)

// Test data: Sample arrayed values of struct Pair.
const ARRAY1: [Pair; 3] = [ // 12 bytes (in total)
    Pair(0x2021, 0x2223),   //  4 bytes
    Pair(0x2425, 0x2627),   //  4 bytes
    Pair(0x2829, 0x2a2b),   //  4 bytes
];

static SLICE1: &[Pair] = &ARRAY1;

// The byte representation of the array above (12 bytes in big-endian)
const BYTES1: [u8; 12] = [
    0x20, 0x21, 0x22, 0x23,  // = Pair(0x2021, 0x2223)
    0x24, 0x25, 0x26, 0x27,  // = Pair(0x2425, 0x2627)
    0x28, 0x29, 0x2a, 0x2b,  // = Pair(0x2829, 0x2a2b)
];

fn main() {
    //
    // Step 2: Method encastsf (1) encasts a byte
    // representation of three consecutive values of struct `Pair`
    // at the head of const `BYTES1` as three consecutive values of
    // the type, (2) flips the endianness of three pairs of 16-bit
    // unsigned integers in the values from the big-endianness (`BE`)
    // to the native-endianness, (3) saves the resulting three
    // consecutive values to variable `array2`, and (4) returns
    // the number of the source bytes in Ok(usize) which is saved
    // to variable `size2`.
    //
    // In the following method call, generic arguments `Pair`
    // can be omitted because it can be infered by the Rust compiler.
    //
    let mut array2: [Pair; 3] = Default::default();
    let size2 = BYTES1.encastsf::<Pair>(&mut array2, BE).unwrap();

    // Check if the value in variable `size2` is as expected.
    assert_eq!(size2, 12); // The size of type `[Pair; 3]` is 12.

    // Check if the content of variable `array2` is as expected.
    assert_eq!(array2, ARRAY1);

    //
    // Step 3: Method encastvf (1) encasts a byte
    // representation of three consecutive values of struct `Pair`
    // at the head of const `BYTES1` as three consecutive values of
    // the type, (2) flips the endianness of three pairs of 16-bit
    // unsigned integers in the values from the big-endianness (`BE`)
    // to the native-endianness, and (3) returns the resulting three
    // consecutive values in Ok(Vec<Pair>) which are saved to variable
    // `vec3`.
    //
    // In the following method call, generic arguments `Pair`
    // can be omitted because it can be infered by the Rust compiler.
    //
    let vec3: Vec<Pair> = BYTES1.encastvf::<Pair>(3, BE).unwrap();

    // Check if the content of variable `vec3` is as expected.
    assert_eq!(&vec3, SLICE1);

    //
    // Step 4: Method decastsf (1) decasts three consecutive
    // values of struct `Pair` in static `SLICE1` as three consecutive
    // byte representations of struct `Pair`, (2) flips the endianness
    // of three pairs of 16-bit unsigned integers in the byte
    // representations from the native-endianness to the big-endianness
    // (`BE`), (3) saves the resulting bytes to  variable `bytes4`,
    // and (4) returns the number of the resulting bytes in Ok(usize)
    // which is saved to variable `size4`.
    //
    // In the following method call, generic arguments `Pair`
    // can be omitted because it can be infered by the Rust compiler.
    //
    let mut bytes4 = [0_u8; 12]; // The size of three struct `Pair`s is 12.
    let size4 = bytes4.decastsf::<Pair>(SLICE1, BE).unwrap();

    // Check if the value in variable `size4` is as expected.
    assert_eq!(size4, 12); // The size of three struct `Pair`s is 12.

    // Check if the content of variable `bytes4` is as expected.
    assert_eq!(bytes4, BYTES1);
}
```

[`derive(Cast)`]: ../../derive.Cast.html
[`derive(Flip)`]: ../../derive.Flip.html

[`repr(C)`]: https://doc.rust-lang.org/reference/type-layout.html#the-c-representation
