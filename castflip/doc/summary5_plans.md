Planned Releases: Version 0.2 and 0.3

# Planned Releases

The following releases are planned.  Preparations are in progress.

- Version 0.2

    1. Trait [`Cast`] and trait [`Flip`] will be changed to `unsafe`.
    2. Trait [`Cast`] and trait [`Flip`] will be implemented for unit
       type [`()`] and struct [`PhantomData`] by default.
    3. Generic parameters for `struct` types and `union` types will be
       supported.
    4. Unused experimental traits will be removed.

- Version 0.3

    1. The API will be redesigned.
    2. The implementation will be overhauled.
    3. The test suite will be redesigned.
    4. Experimental traits will be replaced.
    5. For backward compatibility, the API of the castflip crate
       version 0.2 will be supported in the default configuration.

[`()`]: https://doc.rust-lang.org/core/primitive.unit.html
