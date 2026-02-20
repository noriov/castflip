How to convert between bytes and a `struct` (The UDP header)
using `std::io`

The examples below use platform independent types that implement both
trait [`std::io::Read`] and trait [`std::io::Write`] so that they can
run on any platform.  Needless to say, the steps in the examples can
be applied to those types that implement trait [`std::io::Read`]
and/or trait [`std::io::Write`] such as the types for file I/O and
network I/O.

1. [Through a byte stream provided by struct `std::io::Cursor`
   ](../long_example4_1/index.html)
2. [Through a byte stream provided by a mutable byte slice `&mut [u8]`
   ](../long_example4_2/index.html)
