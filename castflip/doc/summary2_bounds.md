Traits as Bounds: `Cast`, `Flip` and `NopFlip`

# Fundamental Bounds : `Cast` and `Flip`

Trait [`Cast`] and trait [`Flip`] are the fundamental traits of this
crate.

1. Trait [`Cast`] : The values of types that implement trait [`Cast`]
   can be encasted[^encast] and decasted[^decast] by the methods of
   this crate.  Trait [`Cast`] is implemented for

    - all primitive numeric types,
    - array types whose elements' types implement trait [`Cast`],
    - `struct` types and `union` types whose all fields' types implement
      trait [`Cast`] and whose type definitions are annotated with both
      attribute `#[`[`derive(Cast)`]`]` and attribute
      `#[`[`repr(C)`]`]`, and
    - `struct` types with no field and whose type definitions are
      annotated with attribute `#[`[`derive(Cast)`]`]`.

2. Trait [`Flip`] : The endiannesses of the values of types that
   implement trait [`Flip`] can be flipped by the methods of this
   crate.  Trait [`Flip`] is implemented for

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

3. Trait [`NopFlip`] : Types that implement trait [`NopFlip`] also
   implement trait [`Flip`] whose methods do nothing.  Trait
   [`NopFlip`] is implemented for

    - `struct` types and `union` types whose all fields' types
      implement trait [`Flip`] or with no field and whose type
      definitions are annotated with attribute
      `#[`[`derive(NopFlip)`]`]`[^NopFlip].

Fore more information, see the document of the trait or the derive macro.

# Derive Macros : `Cast`, `Flip` and `NopFlip`

By applying the following attributes to a `struct` type or a `union`
type, the type implements the corresponding trait or traits as listed
below.

- By applying `#[`[`derive(Cast)`]`]` and `#[`[`repr(C)`]`]` to a
  type, the type implements trait [`Cast`].

- By applying `#[`[`derive(Flip)`]`]` to a type, the type implements
  trait [`Flip`].

- By applying `#[`[`derive(NopFlip)`]`]` to a type, the type
  implements both trait [`Flip`] whose methods do nothing and trait
  [`NopFlip`].

[^encast]: In this crate, to *encast* means to cast a byte
representation of a type as a value of the type.

[^decast]: In this crate, to *decast* means to cast a value of a type
as a byte representation of the type.

[^NopFlip]: The types annotated with attribute `#[`[`derive(NopFlip)`]`]`
implement both trait [`Flip`] with NOP (No OPeration) methods and
marker trait [`NopFlip`].

[`derive(Cast)`]: ../../derive.Cast.html
[`derive(Flip)`]: ../../derive.Flip.html
[`derive(NopFlip)`]: ../../derive.NopFlip.html

[`repr(C)`]: https://doc.rust-lang.org/reference/type-layout.html#the-c-representation
