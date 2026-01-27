Arrayed Example: An array of a `struct` type is encasted[^encast] and
decasted[^decast] with its endianness flipped as required.

[^encast]: In this crate, to *encast* means to cast a byte
representation of a type as a value of the type.

[^decast]: In this crate, to *decast* means to cast a value of a type
as a byte representation of the type.

From the API's point of view, an array of type `[T; N]` can be viewed
as:

1. [A value of type `[T; N]`](#1-an-array-as-a-value-of-type-t-n)
2. [Consecutive values of type `T`
   ](#2-an-array-as-consecutive-values-of-type-t), or
3. [An element of slice `[[T; N]]`](#3-an-array-as-an-element-of-slice-t-n)

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
  representation of type `[T; N]` as a values of the type and returns
  the resulting value in `Ok([T; N])`.

- Step 3: Method [`DecastMem::decastf`] decasts a value of
  type `[T; N]` as a byte representation of the type and saves the
  resulting bytes to `self`.

## Source Code

```rust
use castflip::{BE, Cast, DecastMem, EncastMem, Flip};

//
// Step 1: Define struct `Pair` and test data.
//
#[repr(C)]            // to make it possible to apply #[derive(Cast)]
#[derive(Cast, Flip)] // to implement trait Cast and trait Flip
#[derive(Debug, PartialEq)] // to use assert_eq!
struct Pair (u16, u16); // 4 bytes (in total)

// Test data: A sample value of `[Pair; 3]`
const VALUE1: [Pair; 3] = [ // 12 bytes (in total)
    Pair(0x2021, 0x2223),   //  4 bytes
    Pair(0x2425, 0x2627),   //  4 bytes
    Pair(0x2829, 0x2a2b),   //  4 bytes
];

// A sample byte representation of the value above (12 bytes in big-endian)
const BYTES1: [u8; 12] = [
    0x20, 0x21, 0x22, 0x23,  // = Pair(0x2021, 0x2223)
    0x24, 0x25, 0x26, 0x27,  // = Pair(0x2425, 0x2627)
    0x28, 0x29, 0x2a, 0x2b,  // = Pair(0x2829, 0x2a2b)
];

fn my_main() -> Option<()> {
    //
    // Step 2: Method encastf (1) encasts a byte representation of
    // type [Pair; 3] at the head of const `BYTES1` as a value of the type,
    // (2) flips the endianness of three pairs of 16-bit unsigned integers
    // in the value from the big-endianness to the native-endianness, and
    // (3) returns the resulting value in Ok([Pair; 3]) which is saved
    // to variable `value2`.
    //
    // In the following method call, (a) generic argument `[Pair; 3]`
    // can be omitted because it can be infered by the Rust compiler,
    // and (b) argument `BE` stands for the Big-Endianness.
    //
    let value2: [Pair; 3] = BYTES1.encastf::<[Pair; 3]>(BE)?;

    // Check if the content of variable `value2` is as expected.
    assert_eq!(value2, VALUE1);

    //
    // Step 3: Method decastf (1) decasts a value in const
    // `VALUE1` of type [Pair; 3] as a byte representation of the type,
    // (2) flips the endianness of three pairs of 16-bit unsigned integers
    // in the bytes from the native-endianness to the big-endianness,
    // (3) saves the resulting bytes to `self`, i.e., variable `bytes3`,
    // and (4) returns the number of the resulting bytes in Ok(usize)
    // which is saved to variable `size3`.
    //
    // In the following method call, (a) generic argument `[Pair; 3]`
    // can be omitted because it can be infered by the Rust compiler,
    // and (b) argument `BE` stands for the Big-Endianness.
    //
    let mut bytes3 = [0_u8; 12];
    let size3 = bytes3.decastf::<[Pair; 3]>(&VALUE1, BE)?;

    // Check if the value in variable `size3` is as expected.
    assert_eq!(size3, 12); // The size of type `[Pair; 3]` is 12.

    // Check if the content of variable `bytes3` is as expected.
    assert_eq!(bytes3, BYTES1);

    Some(())
}

fn main() { my_main(); }
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
  representation of consecutive values of type `T` as the values of
  type `T`, saves the resulting values to the specified slice, and
  returns the number of the source bytes in `Ok(usize)`.

- Step 3: Method [`EncastMem::encastvf`] encasts a byte
  representation of consecutive values of type `T` as the values of
  type `T` and returns the resulting values in `Ok(Vec<T>)`.

- Step 4: Method [`DecastMem::decastsf`] decasts a
  slice of type `T` as a byte representation of consecutive values of
  type `T` and saves the resulting bytes to `self`.

## Source Code

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

// The byte representation of the array above (12 bytes in big-endian)
const BYTES1: [u8; 12] = [
    0x20, 0x21, 0x22, 0x23,  // = Pair(0x2021, 0x2223)
    0x24, 0x25, 0x26, 0x27,  // = Pair(0x2425, 0x2627)
    0x28, 0x29, 0x2a, 0x2b,  // = Pair(0x2829, 0x2a2b)
];

fn my_main() -> Option<()> {
    //
    // Step 2: Method encastsf (1) encasts a byte representation
    // of consecutive three values of struct `Pair` at the head of const
    // `BYTES1` as three values of the type, (2) flips the endianness of
    // three pairs of 16-bit unsigned integers in the values from
    // the big-endianness to the native-endianness, (3) saves resulting
    // three values to variable `array2`, and (4) returns the number of
    // the source bytes in Ok(usize) which is saved to variable `size2`.
    //
    // In the following method call, (a) generic arguments `Pair`
    // can be omitted because it can be infered by the Rust compiler,
    // and (b) argument `BE` stands for the Big-Endianness.
    //
    let mut array2: [Pair; 3] = Default::default();
    let size2 = BYTES1.encastsf::<Pair>(&mut array2, BE)?;

    // Check if the value in variable `size2` is as expected.
    assert_eq!(size2, 12); // The size of type `[Pair; 3]` is 12.

    // Check if the content of variable `array2` is as expected.
    assert_eq!(array2, ARRAY1);

    //
    // Step 3: Method encastvf (1) encasts a byte representation
    // of consecutive three values of struct `Pair` at the head of const
    // `BYTES1` as three values of the type, (2) flips the endianness of
    // three pairs of 16-bit unsigned integers in the values from
    // the big-endianness to the native-endianness, and (3) returns the
    // resulting values in Ok(Vec<Pair>) which are saved to variable `vec3`.
    //
    // In the following method call, (a) generic arguments `Pair`
    // can be omitted because it can be infered by the Rust compiler,
    // and (b) argument `BE` stands for the Big-Endianness.
    //
    let vec3: Vec<Pair> = BYTES1.encastvf::<Pair>(3, BE)?;

    // Check if the content of variable `vec3` is as expected.
    assert_eq!(&vec3, &ARRAY1);

    //
    // Step 4: Method decastsf (1) decasts consecutive three
    // values of struct `Pair` stored in `ARRAY1` as a byte representation
    // of consecutive three values of struct `Pair`, (2) flips the endianness
    // of three pairs of 16-bit unsigned integers in the bytes from the
    // native-endianness to the big-endianness, (3) saves the resulting bytes
    // to `self`, i.e., variable `bytes4`, and (4) returns the number of the
    // resulting bytes in Ok(usize) which is saved to variable `size4`.
    //
    // In the following method call, (a) generic arguments `Pair`
    // can be omitted because it can be infered by the Rust compiler,
    // and (b) argument `BE` stands for the Big-Endianness.
    //
    let mut bytes4 = [0_u8; 12]; // The size of three struct `Pair`s is 12.
    let size4 = bytes4.decastsf::<Pair>(&ARRAY1, BE)?;

    // Check if the value in variable `size4` is as expected.
    assert_eq!(size4, 12); // The size of three struct `Pair`s is 12.

    // Check if the content of variable `bytes4` is as expected.
    assert_eq!(bytes4, BYTES1);

    Some(())
}

fn main() { my_main(); }
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
  representation of type `[T; N]` as a values of the type, saves the
  resulting value to the specified slice, and returns the number of
  the source bytes in `Ok(usize)`.

- Step 3: [`EncastMem::encastvf`] encasts a byte
  representation of type `[T; N]` as a values of the type and returns
  the resulting values in `Ok(Vec<[T; N]>)`.

- Step 4: Method [`DecastMem::decastsf`] decasts a
  value of type `[T; N]` as a byte representation of the type and
  saves the resulting bytes to `self`.

## Source Code

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

// Test data: A sample value of `[Pair; 3]`
const VALUE1: [Pair; 3] = [ // 12 bytes (in total)
    Pair(0x2021, 0x2223),   //  4 bytes
    Pair(0x2425, 0x2627),   //  4 bytes
    Pair(0x2829, 0x2a2b),   //  4 bytes
];

// The byte representation of the value above (12 bytes in big-endian)
const BYTES1: [u8; 12] = [
    0x20, 0x21, 0x22, 0x23,  // = Pair(0x2021, 0x2223)
    0x24, 0x25, 0x26, 0x27,  // = Pair(0x2425, 0x2627)
    0x28, 0x29, 0x2a, 0x2b,  // = Pair(0x2829, 0x2a2b)
];

fn my_main() -> Option<()> {
    //
    // Step 2: Method encastsf (1) encasts a byte representation
    // of type `[T; N]` at the head of const `BYTES1` as a values of the type,
    // (2) flips the endianness of three pairs of 16-bit unsigned integers in
    // the values from the big-endianness to the native-endianness, (3) saves
    // resulting three values to variable `array2`, and (4) returns the number
    // of the source bytes in Ok(usize) which is saved to variable `size2`.
    //
    // In the following method call, (a) generic arguments `Pair`
    // can be omitted because it can be infered by the Rust compiler,
    // and (b) argument `BE` stands for the Big-Endianness.
    //
    let mut array2: [[Pair; 3]; 1] = Default::default();
    let size2 = BYTES1.encastsf::<[Pair; 3]>(&mut array2, BE)?;

    // Check if the value in variable `size2` is as expected.
    assert_eq!(size2, 12); // The size of type `[Pair; 3]` is 12.

    // Check if the content of variable `array2` is as expected.
    assert_eq!(array2, [VALUE1]);

    //
    // Step 3: Method encastvf (1) encasts a byte representation of
    // type [Pair; 3] at the head of const `BYTES1` as a value of the type,
    // (2) flips the endianness of three pairs of 16-bit unsigned integers
    // in the value from the big-endianness to the native-endianness, and
    // (3) returns the resulting value in Ok(Vec<[Pair; 3]>) which is saved
    // to variable `vec3`.
    //
    // In the following method call, (a) generic argument `[Pair; 3]`
    // can be omitted because it can be infered by the Rust compiler,
    // and (b) argument `BE` stands for the Big-Endianness.
    //
    let vec3: Vec<[Pair; 3]> = BYTES1.encastvf::<[Pair; 3]>(1, BE)?;

    // Check if the content of variable `vec3` is as expected.
    assert_eq!(&vec3, &[VALUE1]);

    //
    // Step 4: Method decastsf (1) decasts a value  of type
    // `[Pair; 3]` stored in `&[VALUE1]` as a byte representation of the type,
    // (2) flips the endianness of three pairs of 16-bit unsignedintegers in
    // the bytes from the native-endianness to the big-endianness, (3) saves
    // the resulting bytes to `self`, i.e., variable `bytes4`, and (4) returns
    // the number of the resulting bytes in Ok(usize) which is saved to
    // variable `size4`.
    //
    // In the following method call, (a) generic argument `[Pair; 3]`
    // can be omitted because it can be infered by the Rust compiler,
    // and (b) argument `BE` stands for the Big-Endianness.
    //
    let mut bytes4= [0_u8; 12]; // The size of type `[Pair; 3]` is 12.
    let size4 = bytes4.decastsf::<[Pair; 3]>(&[VALUE1], BE)?;

    // Check if the value in variable `size4` is as expected.
    assert_eq!(size4, 12); // The size of type `[Pair; 3]` is 12.

    // Check if the content of variable `bytes4` is as expected.
    assert_eq!(bytes4, BYTES1);

    Some(())
}

fn main() { my_main(); }
```

[`derive(Cast)`]: ../../derive.Cast.html
[`derive(Flip)`]: ../../derive.Flip.html

[`allocator_api`]: https://doc.rust-lang.org/beta/unstable-book/library-features/allocator-api.html
[`repr(C)`]: https://doc.rust-lang.org/reference/type-layout.html#the-c-representation
