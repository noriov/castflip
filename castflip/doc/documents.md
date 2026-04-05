The documents directory

# Short Examples as a Quick Start Guide:

1. [How to convert between bytes and a number
   ](./short_example1/index.html)
2. [How to convert between bytes and an array of numbers
   ](./short_example2/index.html)
3. [How to convert between bytes and a `struct`
   ](./short_example3/index.html)
4. [How to convert between bytes and an array of `struct`s
   ](./short_example4/index.html)
5. [How to convert between bytes and nested `struct`s
   ](./short_example5/index.html)

# Long Examples with Explanations:

1. [How to convert between bytes and a `struct` (of the UDP header)
   ](./long_example1/index.html)
2. [How to convert between bytes and nested `struct`s
   ](./long_example2/index.html)
3. [How to convert between bytes and an array of type `[T; N]`
   ](./long_example3/index.html)
    1. [As a value of type `[T; N]`](./long_example3_1/index.html)
    2. [As N-consecutive values of type `T`](./long_example3_2/index.html)
    3. [As an element of slice `[[T; N]]`](./long_example3_3/index.html)
4. [How to convert between bytes and a `struct` (of the UDP header)
   using `std::io`](./long_example4/index.html)
    1. [Through a byte stream provided by struct `std::io::Cursor`
       ](./long_example4_1/index.html)
    2. [Through a byte stream provided by a mutable byte slice `&mut [u8]`
       ](./long_example4_2/index.html)

# Summaries of Types, Traits, Crate Features, etc.:

1. [Enum Type: `Endian`](./summary1/index.html)
2. [Traits as Bounds: `Cast`, `Flip` and `NopFlip`](./summary2/index.html)
3. [Traits to Encast and Decast: `EncastMem` and `DecastMem` /
   `EncastIO` and `DecastIO`](./summary3/index.html)
4. [Crate Features: `alloc` and `std`](./summary4/index.html)
5. [Planned Releases: Version 0.2 and 0.3](./summary5/index.html)
