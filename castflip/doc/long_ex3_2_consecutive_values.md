How to convert between bytes and consecutive values of type `T`

The example below encasts consecutive byte representations of type `T`
in big-endian as values of the type in native-endian in two different
ways, then decasts values of type `T` in native-endian as consecutive
byte representations of the type in big-endian.

# Outline

- Step 1: A tuple `struct` type is defined.  It is aliased to `T` here.
    - It implements trait [`Cast`] by applying both attribute
      `#[`[`derive(Cast)`]`]` and attribute `#[`[`repr(C)`]`]` to it.
    - It implements trait [`Flip`] by applying attribute
      `#[`[`derive(Flip)`]`]` to it.

- Step 2: Method [`EncastMem::encastsf`] encasts a byte
  representation of consecutive values of type `T` in big-endian
  ([`BE`]) as the values of type `T` in native-endian, saves the
  resulting values to the specified slice, and returns the number of
  the source bytes in [`Ok`]`(usize)`.

- Step 3: Method [`EncastMem::encastvf`] encasts a byte
  representation of consecutive values of type `T` in big-endian
  ([`BE`]) as the values of type `T` in native-endian and returns the
  resulting values in `Ok(Vec<T>)`.

- Step 4: Method [`DecastMem::decastsf`] decasts a
  slice of type `T` in native-endian as a byte representation of
  consecutive values of type `T` in big-endian ([`BE`]) and saves the
  resulting bytes to `self`.

# Source Code

```rust
use castflip::{BE, Cast, DecastMem, EncastMem, Flip};

//
// Step 1: Define struct `Point` and test data.
//
#[repr(C)]            // to make it possible to apply #[derive(Cast)]
#[derive(Cast, Flip)] // to implement trait Cast and trait Flip
#[derive(Default)]    // to give this type a default value
#[derive(Debug, PartialEq)] // to use assert_eq!
struct Point (u16, u16); // 4 bytes (in total)

// Test data: Sample arrayed values of struct Point.
const ARRAY1: [Point; 3] = [ // 12 bytes (in total)
    Point(0x2021, 0x2223),   //  4 bytes
    Point(0x2425, 0x2627),   //  4 bytes
    Point(0x2829, 0x2a2b),   //  4 bytes
];

// The byte representation of the array above (12 bytes in big-endian)
const BYTES1: [u8; 12] = [
    0x20, 0x21, 0x22, 0x23,  // = Point(0x2021, 0x2223)
    0x24, 0x25, 0x26, 0x27,  // = Point(0x2425, 0x2627)
    0x28, 0x29, 0x2a, 0x2b,  // = Point(0x2829, 0x2a2b)
];

fn main() {
    //
    // Step 2: Method encastsf (1) encasts a byte representation
    // of consecutive three values of struct `Point` at the head of const
    // `BYTES1` as three values of the type, (2) flips the endianness of
    // three pairs of 16-bit unsigned integers in the values from
    // the big-endianness (`BE`) to the native-endianness, (3) saves
    // resulting three values to variable `array2`, and (4) returns the
    // number of the source bytes in Ok(usize) which is saved to variable
    // `size2`.
    //
    // In the following method call, generic arguments `Point`
    // can be omitted because it can be infered by the Rust compiler.
    //
    let mut array2: [Point; 3] = Default::default();
    let size2 = BYTES1.encastsf::<Point>(&mut array2, BE).unwrap();

    // Check if the value in variable `size2` is as expected.
    assert_eq!(size2, 12); // The size of type `[Point; 3]` is 12.

    // Check if the content of variable `array2` is as expected.
    assert_eq!(array2, ARRAY1);

    //
    // Step 3: Method encastvf (1) encasts a byte representation
    // of consecutive three values of struct `Point` at the head of const
    // `BYTES1` as three values of the type, (2) flips the endianness of
    // three pairs of 16-bit unsigned integers in the values from the
    // big-endianness (`BE`) to the native-endianness, and (3) returns
    // the resulting values in Ok(Vec<Point>) which are saved to variable
    // `vec3`.
    //
    // In the following method call, generic arguments `Point`
    // can be omitted because it can be infered by the Rust compiler.
    //
    let vec3: Vec<Point> = BYTES1.encastvf::<Point>(3, BE).unwrap();

    // Check if the content of variable `vec3` is as expected.
    assert_eq!(&vec3, &ARRAY1);

    //
    // Step 4: Method decastsf (1) decasts consecutive three
    // values of struct `Point` stored in `ARRAY1` as a byte representation
    // of consecutive three values of struct `Point`, (2) flips the
    // endianness of three pairs of 16-bit unsigned integers in the bytes
    // from the native-endianness to the big-endianness (`BE`), (3) saves
    // the resulting bytes to `self`, i.e., variable `bytes4`, and
    // (4) returns the number of the resulting bytes in Ok(usize) which is
    // saved to variable `size4`.
    //
    // In the following method call, generic arguments `Point`
    // can be omitted because it can be infered by the Rust compiler.
    //
    let mut bytes4 = [0_u8; 12]; // The size of three struct `Point`s is 12.
    let size4 = bytes4.decastsf::<Point>(&ARRAY1, BE).unwrap();

    // Check if the value in variable `size4` is as expected.
    assert_eq!(size4, 12); // The size of three struct `Point`s is 12.

    // Check if the content of variable `bytes4` is as expected.
    assert_eq!(bytes4, BYTES1);
}
```

[`derive(Cast)`]: ../../derive.Cast.html
[`derive(Flip)`]: ../../derive.Flip.html

[`repr(C)`]: https://doc.rust-lang.org/reference/type-layout.html#the-c-representation
