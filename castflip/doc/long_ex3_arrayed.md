How to convert between a byte representation and a value of an array
of a `struct` type

From the API's point of view, an array of type `[T; N]` can be viewed
as:

1. [A value of type `[T; N]`](#1-an-array-as-a-value-of-type-t-n)
2. [Consecutive values of type `T`
   ](#2-an-array-as-consecutive-values-of-type-t), or
3. [An element of slice `[[T; N]]`](#3-an-array-as-an-element-of-slice-t-n)

The examples below encast[^encast] a byte representation of those
types as a value of the type then decast[^decast] a value of the type
as a byte representation of the type.

[^encast]: In this crate, to *encast* means to cast a byte
representation of a type as a value of the type.

[^decast]: In this crate, to *decast* means to cast a value of a type
as a byte representation of the type.

Both regular `struct` types which have named fields and tuple `struct`
types which have unnamed fields can be converted in the same way.

# 1. An Array As A Value of Type `[T; N]`

The example below encasts a byte representation of type `[T; N]` in
big-endian as a value of type `[T; N]` in native-endian then decasts a
value of type `[T; N]` in native-endian as a byte representation of
type `[T; N]` in big-endian.

## Outline

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

## Source Code

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

# 2. An Array As Consecutive Values of Type `T`

The example below encasts consecutive byte representations of type `T`
in big-endian as values of type `T` in native-endian in two different
ways then decasts values of type `T` in native-endian as consecutive
byte representations of type `T` in big-endian.

## Outline

- Step 1: A tuple `struct` type is defined.  It is aliased to `T` here.
  - It implements trait [`Cast`] by applying both attribute
    `#[`[`derive(Cast)`]`]` and attribute `#[`[`repr(C)`]`]` to it.
  - It implements trait [`Flip`] by applying attribute
    `#[`[`derive(Flip)`]`]` to it.

- Step 2: Method [`EncastMem::encastsf`] encasts a byte
  representation of consecutive values of type `T` in big-endian
  ([`BE`]) as the values of type `T` in native-endian, saves the
  resulting values to the specified slice, and returns the mutable
  slice of the resulting values in [`Ok`]`(&mut [T])`,

- Step 3: Method [`EncastMem::encastvf`] encasts a byte
  representation of consecutive values of type `T` in big-endian
  ([`BE`]) as the values of type `T` in native-endian and returns the
  resulting values in `Ok(Vec<T>)`.

- Step 4: Method [`DecastMem::decastsf`] decasts a
  slice of type `T` in native-endian as a byte representation of
  consecutive values of type `T` in big-endian ([`BE`]) and saves the
  resulting bytes to `self`.

## Source Code

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

# 3. An Array As An Element of Slice `[[T; N]]`

The example below encasts a byte representation of type `[[T; N]]` in
big-endian as a value of type `[[T; N]]` in native-endian in two
different ways then decasts a value of type `[[T; N]]` in
native-endian as a byte representation of type `[[T; N]]` in
big-endian.

## Outline

- Step 1: A tuple `struct` type is defined.  It is aliased to `T` here.
  - It implements trait [`Cast`] by applying both attribute
    `#[`[`derive(Cast)`]`]` and attribute `#[`[`repr(C)`]`]` to it.
  - It implements trait [`Flip`] by applying attribute
    `#[`[`derive(Flip)`]`]` to it.

- Step 2: Method [`EncastMem::encastsf`] encasts a byte
  representation of type `[T; N]` in big-endian ([`BE`]) as a values
  of the type in native-endian, saves the resulting value to the
  specified slice, and returns the mutable slice of the resulting
  value in [`Ok`]`(&mut [T])`,

- Step 3: Method [`EncastMem::encastvf`] encasts a byte
  representation of type `[T; N]` in big-endian ([`BE`]) as a values
  of the type in native-endian and returns the resulting values in
  `Ok(Vec<[T; N]>)`.

- Step 4: Method [`DecastMem::decastsf`] decasts a
  value of type `[T; N]` in big-endian ([`BE`]) as a byte
  representation of the type in native-endian and saves the resulting
  bytes to `self`.

## Source Code

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

// Test data: A sample value of `[Point; 3]`
const VALUE1: [Point; 3] = [ // 12 bytes (in total)
    Point(0x2021, 0x2223),   //  4 bytes
    Point(0x2425, 0x2627),   //  4 bytes
    Point(0x2829, 0x2a2b),   //  4 bytes
];

// The byte representation of the value above (12 bytes in big-endian)
const BYTES1: [u8; 12] = [
    0x20, 0x21, 0x22, 0x23,  // = Point(0x2021, 0x2223)
    0x24, 0x25, 0x26, 0x27,  // = Point(0x2425, 0x2627)
    0x28, 0x29, 0x2a, 0x2b,  // = Point(0x2829, 0x2a2b)
];

fn main() {
    //
    // Step 2: Method encastsf (1) encasts a byte representation
    // of type `[T; N]` at the head of const `BYTES1` as a values of the type,
    // (2) flips the endianness of three pairs of 16-bit unsigned integers in
    // the values from the big-endianness (`BE`) to the native-endianness,
    // (3) saves resulting three values to variable `array2`, and (4) returns
    // the number of the source bytes in Ok(usize) which is saved to variable
    // `size2`.
    //
    // In the following method call, generic arguments `Point`
    // can be omitted because it can be infered by the Rust compiler.
    //
    let mut array2: [[Point; 3]; 1] = Default::default();
    let size2 = BYTES1.encastsf::<[Point; 3]>(&mut array2, BE).unwrap();

    // Check if the value in variable `size2` is as expected.
    assert_eq!(size2, 12); // The size of type `[Point; 3]` is 12.

    // Check if the content of variable `array2` is as expected.
    assert_eq!(array2, [VALUE1]);

    //
    // Step 3: Method encastvf (1) encasts a byte representation of
    // type [Point; 3] at the head of const `BYTES1` as a value of the type,
    // (2) flips the endianness of three pairs of 16-bit unsigned integers
    // in the value from the big-endianness (`BE`) to the native-endianness,
    // and (3) returns the resulting value in Ok(Vec<[Point; 3]>) which is
    // saved to variable `vec3`.
    //
    // In the following method call, generic argument `[Point; 3]`
    // can be omitted because it can be infered by the Rust compiler.
    //
    let vec3: Vec<[Point; 3]> = BYTES1.encastvf::<[Point; 3]>(1, BE).unwrap();

    // Check if the content of variable `vec3` is as expected.
    assert_eq!(&vec3, &[VALUE1]);

    //
    // Step 4: Method decastsf (1) decasts a value  of type
    // `[Point; 3]` stored in `&[VALUE1]` as a byte representation of the type,
    // (2) flips the endianness of three pairs of 16-bit unsignedintegers in
    // the bytes from the native-endianness to the big-endianness (`BE`),
    // (3) saves the resulting bytes to `self`, i.e., variable `bytes4`, and
    // (4) returns the number of the resulting bytes in Ok(usize) which is
    // saved to variable `size4`.
    //
    // In the following method call, generic argument `[Point; 3]`
    // can be omitted because it can be infered by the Rust compiler.
    //
    let mut bytes4= [0_u8; 12]; // The size of type `[Point; 3]` is 12.
    let size4 = bytes4.decastsf::<[Point; 3]>(&[VALUE1], BE).unwrap();

    // Check if the value in variable `size4` is as expected.
    assert_eq!(size4, 12); // The size of type `[Point; 3]` is 12.

    // Check if the content of variable `bytes4` is as expected.
    assert_eq!(bytes4, BYTES1);
}
```

[`derive(Cast)`]: ../../derive.Cast.html
[`derive(Flip)`]: ../../derive.Flip.html

[`allocator_api`]: https://doc.rust-lang.org/beta/unstable-book/library-features/allocator-api.html
[`repr(C)`]: https://doc.rust-lang.org/reference/type-layout.html#the-c-representation
