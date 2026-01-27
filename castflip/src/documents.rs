//!
//! Is a document folder.
//!
//! This module defines the hierarchy of the document tree.
//!

pub mod example1 {
    // 1. Basic Example: A `struct` type is encasted and decasted
    //    with its endianness flipped as required.
    #![doc = include_doc!("example1_basic_example.md")]
    use crate::*;
}

pub mod example2 {
    // 2. Nested Example: A nested `struct` type is encasted and decasted
    //    with its endianness flipped as required.
    #![doc = include_doc!("example2_nested_example.md")]
    use crate::*;
}

pub mod example3 {
    // 3. Arrayed Example: An array of a `struct` type is encasted and decasted
    //    with its endianness flipped as required.
    #![doc = include_doc!("example3_arrayed_example.md")]
    use crate::*;
}

pub mod example4 {
    // 4. I/O Example: A `struct` type is encasted and decasted
    //    with its endianness flipped as required
    //    using trait [`std::io::Read`] and trait [`std::io::Write`].
    #![doc = include_doc!("example4_using_std_io.md")]
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
