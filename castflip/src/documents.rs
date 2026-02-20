#![doc = include_doc!("documents.md")]

use crate::include_doc;

pub mod short_example1 {
    // 1. How to convert between bytes and a number
    #![doc = include_doc!("short_ex1_number.md")]
    use crate::*;
}

pub mod short_example2 {
    // 2. How to convert between bytes and an array of numbers
    #![doc = include_doc!("short_ex2_number_array.md")]
    use crate::*;
}

pub mod short_example3 {
    // 3. How to convert between bytes and a `struct`
    #![doc = include_doc!("short_ex3_struct.md")]
    use crate::*;
}

pub mod short_example4 {
    // 4. How to convert between bytes and an array of a `struct`s
    #![doc = include_doc!("short_ex4_struct_array.md")]
    use crate::*;
}

pub mod long_example1 {
    // 1. How to convert between bytes and a `struct` (The UDP header)
    #![doc = include_doc!("long_ex1_struct.md")]
    use crate::*;
}

pub mod long_example2 {
    // 2. How to convert between bytes and a nested `struct`
    #![doc = include_doc!("long_ex2_nested.md")]
    use crate::*;
}

pub mod long_example3 {
    // 3. How to convert between bytes and an array of `struct`s
    #![doc = include_doc!("long_ex3_arrayed.md")]
    use crate::*;
}

pub mod long_example3_1 {
    // 3.1. As a value of type `[T; N]`
    #![doc = include_doc!("long_ex3_1_an_array.md")]
    use crate::*;
}

pub mod long_example3_2 {
    // 3.2. As consecutive values of type `T`
    #![doc = include_doc!("long_ex3_2_consecutive_values.md")]
    use crate::*;
}

pub mod long_example3_3 {
    // 3.3. As an element of slice `[[T; N]]`
    #![doc = include_doc!("long_ex3_3_an_element_of_slice.md")]
    use crate::*;
}

pub mod long_example4 {
    // 4. How to convert between bytes and a `struct` (The UDP header)
    //    using `std::io`
    #![doc = include_doc!("long_ex4_std_io.md")]
    use crate::*;
}

pub mod long_example4_1 {
    // 4.1. Through a byte stream provided by struct [`std::io::Cursor`]
    #![doc = include_doc!("long_ex4_1_cursor.md")]
    use crate::*;
}

pub mod long_example4_2 {
    // 4.2. Through a byte stream provided by a mutable byte slice `&mut [u8]`
    #![doc = include_doc!("long_ex4_2_slice.md")]
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
    // 3. Traits to Encast and Decast: `EncastMem` and `DecastMem` /
    //    `EncastIO` and `DecastIO`
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
    use core::marker::PhantomData;
}
