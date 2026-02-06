//!
//! Is a document folder.
//!
//! This module defines the hierarchy of the document tree.
//!

pub mod short_example1 {
    // 1. How to convert between a byte representation and
    //    a number
    #![doc = include_doc!("short_ex1_number.md")]
    use crate::*;
}

pub mod short_example2 {
    // 2. How to convert between a byte representation and
    //    an array of numbers
    #![doc = include_doc!("short_ex2_array.md")]
    use crate::*;
}

pub mod short_example3 {
    // 3. How to convert between a byte representation and
    //    a value of a `struct` type
    #![doc = include_doc!("short_ex3_struct.md")]
    use crate::*;
}

pub mod long_example1 {
    // 1. How to convert between a byte representation of the UDP
    //    header and a value of a `struct` type
    #![doc = include_doc!("long_ex1_struct.md")]
    use crate::*;
}

pub mod long_example2 {
    // 2. How to convert between a byte representation and a value of
    //    a nested `struct` type
    #![doc = include_doc!("long_ex2_nested.md")]
    use crate::*;
}

pub mod long_example3 {
    // 3. How to convert between a byte representation and a value of
    //    an array of a `struct` type
    #![doc = include_doc!("long_ex3_arrayed.md")]
    use crate::*;
}

pub mod long_example4 {
    // 4. How to utilize struct `std::io::Cursor` and a mutable fat
    //    pointer of `&mut [u8]`
    #![doc = include_doc!("long_ex4_std_io.md")]
    use crate::*;
}

pub mod summary1 {
    // 1. Enum Type: `Endian`
    #![doc = include_doc!("summary1_types.md")]
    use crate::*;
}

pub mod summary2 {
    // 2. Traits as Bounds: `Cast`, `Flip` and `NopFlip`
    #![doc = include_doc!("summary2_bounds.md")]
    use crate::*;
    use core::marker::PhantomData;
}

pub mod summary3 {
    // 3. Traits to Cast and Flip: `DecastMem` and `EncastMem`/
    //    `DecastIO` and `EncastIO`
    #![doc = include_doc!("summary3_traits.md")]
    use crate::*;
}

pub mod summary4 {
    // 4. Crate Features: `alloc` and `std`
    #![doc = include_doc!("summary4_features.md")]
    use crate::*;
}

pub mod summary5 {
    // 5. Planned Releases: Version 0.2 and 0.3
    #![doc = include_doc!("summary5_plans.md")]
    use crate::*;
}
