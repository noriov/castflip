Traits to Cast and Flip: `DecastMem` and `EncastMem`/ `DecastIO` and `EncastIO`

# Traits for Bytes in Memory : `DecastMem` and `EncastMem`

The following two traits are defined to decast values as in-memory
byte representations or to encast in-memory byte representations as
values.

- Trait [`DecastMem`]\
  Provides the `decast` method family whose methods decast one or more
  values of a type specified by a parameter as one or more byte
  representations of the type, flip the endianness of the byte
  representations as required, save the resulting byte representations
  at the head of `self`, then return the number of the bytes in the
  byte representations in [`Some`]`(usize)`.

- Trait [`EncastMem`]\
  Provides the `encast` method family whose methods encast one or more
  byte representations of a type at the head of `self` as one or more
  values of the type, flip the endianness of the values as required,
  then return the resulting values in [`Some`].

The methods of these traits return [`Option`].  On failure, they return
[`None`].

Fore more information, see the document of each trait.

# Traits to Read and Write Bytes using [`std::io`] : `DecastIO` and `EncastIO`

The following two traits are defined to decast using
[`std::io::Write`] or to encast using [`std::io::Read`].

- Trait [`DecastIO`]\
  Provides the `decast` method family whose methods decast one or more
  values of a type specified by a parameter as one or more byte
  representations of the type, flip the endianness of the byte
  representations as required, write the resulting byte
  representations to writer `self` using trait [`std::io::Write`],
  then return the number of the bytes in the byte representations in
  [`Ok`]`(usize)`.

- Trait [`EncastIO`]\
  Provides the `encast` method family whose methods encast one or more
  byte representations of a type read from reader `self` using trait
  [`std::io::Read`] as one or more values of the type, flip the
  endianness of the values as required, then return the resulting
  values in [`Ok`].

The methods of these traits return [`Result`].  On failure, they return
an error value of struct [`std::io::Error`] in [`Err`].

Fore more information, see the document of each trait.

# The `decast` method family

The `decast` method family contains four methods.

* Method `decast` decasts a value of type `T` specified by a parameter
  as a byte representation of type `T` without fliping its endianness
  and writes the resulting bytes to `self`.

* Method `decastf` decasts a value of type `T` specified by a parameter
  as a byte representation of type `T` with its endianness flipped as
  required and writes the resulting bytes to `self`.

* Method `decasts` decasts values of type `T` specified by a parameter
  as byte representations of type `T` without fliping its endianness
  and writes the resulting bytes to `self`.

* Method `decastsf` decasts values of type `T` specified by a parameter
  as byte representations of type `T` with its endianness flipped as
  required and writes the resulting bytes to `self`.

# The `encast` method family

The `encast` method family contains six methods.

* Method `encast` encasts a byte representation of type `T` as a value
  of type `T` without fliping its endianness and returns the resulting
  value.

* Method `encastf` encasts a byte representation of type `T` as a value
  of type `T` with its endianness flipped as required and returns the
  resulting value.

* Method `encasts` encasts byte representations of type `T` as values
  of type `T` without fliping its endianness and writes the resulting
  values to the slice specified by a parameter.

* Method `encastsf` encasts byte representations of type `T` as values
  of type `T` with its endianness flipped as required and writes the
  resulting values to the slice specified by a parameter.

* Method `encastv` encasts byte representations of type `T` as values
  of type `T` without fliping its endianness and returns the resulting
  values in struct `Vec<T>`.

* Method `encastvf` encasts byte representations of type `T` as values
  of type `T` with its endianness flipped as required and returns the
  resulting values in struct `Vec<T>`.
