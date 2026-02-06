Crate Features: `alloc` and `std`

# Crate Features

The following two crate features are defined in this crate.

- `alloc`\
  Makes it possible to use [`Vec`]`<T>` with this crate.

- `std` (enabled by default)\
  Makes it possible to use [`std::io`] with this crate.
  That is, it makes possible to use [`DecastIO`] and [`EncastIO`].\
  When this feature is enabled, crate feature `alloc` is also enabled.

By default, crate feature `std` is enabled.

# Notes on `#![no_std]`

To use this crate on a [`no_std`] environment without memory
allocator, add the following line to the dependencies in `Cargo.toml`.

```toml
default-features = false
```

Below is an example of the dependencies description.

```toml
[dependencies.castflip]
version = "0.1"
default-features = false
```

To use this crate on a [`no_std`] environment with memory allocator,
add the following lines to the dependencies in `Cargo.toml`.

```toml
default-features = false
features = ["alloc"]
```

Below is an example of the dependencies description.

```toml
[dependencies.castflip]
version = "0.1"
default-features = false
features = ["alloc"]
```

[`allocator_api`]: https://doc.rust-lang.org/beta/unstable-book/library-features/allocator-api.html
[`no_std`]: https://doc.rust-lang.org/reference/names/preludes.html#the-no_std-attribute
[`Vec`]: https://doc.rust-lang.org/stable/alloc/vec/struct.Vec.html
