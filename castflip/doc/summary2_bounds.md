Traits as Bounds: `Cast`, `Flip` and `NopFlip`

# Fundamental Bounds : `Cast` and `Flip`

Trait [`Cast`] and trait [`Flip`] are the fundamental traits of this
crate.

1. Trait [`Cast`] : The values of those types that implement trait
   [`Cast`] can be encasted and decasted by using the methods of this
   crate.  Trait [`Cast`] is implemented for

   - all primitive numeric types,
   - array types whose elements' types implement trait [`Cast`],
   - `struct` types and `union` types whose all fields' types implement
     trait [`Cast`] and whose type definitions are annotated with both
     attribute `#[`[`derive(Cast)`]`]` and attribute
     `#[`[`repr(C)`]`]`, and
   - `struct` types with no field and whose type definitions are
     annotated with attribute `#[`[`derive(Cast)`]`]`.

2. Trait [`Flip`] : The endianness of the values of those types that
   implement trait [`Flip`] can be flipped by using the methods of
   this crate.  Trait [`Flip`] is implemented for

   - all primitive numeric types,
   - array types whose elements' types implement trait [`Flip`],
   - `struct` types whose all fields' types implement trait [`Flip`] or
     with no field, and whose type definitions are annotated with
     attribute `#[`[`derive(Flip)`]`]`, and
   - `struct` types and `union` types whose all fields' types
     implement trait [`Flip`] and whose type definitions are annotated
     with attribute `#[`[`derive(NopFlip)`]`]`[^NopFlip].

Fore more information, see the document of each trait or derive macro.

# Supplementary Bounds : `NopFlip`

The following trait is a subtrait of trait [`Flip`].

3. Trait [`NopFlip`] : Those types that implement trait [`NopFlip`]
   also implement trait [`Flip`] whose methods do nothing.  It is
   implemented for

   - `struct` types and `union` types whose all fields' types
     implement trait [`Flip`] and whose type definitions are annotated
     with attribute `#[`[`derive(NopFlip)`]`]`[^NopFlip].

Fore more information, see the document of the trait or the derive macro.

# Derive Macros : `Cast`, `Flip` and `NopFlip`

As described above, by applying following derive macros to a `struct`
type or a `union` type, the type implements a trait as listed below.

- By applying `#[`[`derive(Cast)`]`]` and `#[`[`repr(C)`]`]` to a
  type, the type implements trait [`Cast`].

- By applying `#[`[`derive(Flip)`]`]` to a type, the type implements
  trait [`Flip`].

- By applying `#[`[`derive(NopFlip)`]`]` to a type, the type
  implements trait [`Flip`] and trait [`NopFlip`].

[^NopFlip]: The types annotated with attribute `#[`[`derive(NopFlip)`]`]`
implement both trait [`Flip`] with NOP (No OPeration) methods and
marker trait [`NopFlip`].

[`derive(Cast)`]: ../../derive.Cast.html
[`derive(Flip)`]: ../../derive.Flip.html
[`derive(NopFlip)`]: ../../derive.NopFlip.html

[`repr(C)`]: https://doc.rust-lang.org/reference/type-layout.html#the-c-representation
