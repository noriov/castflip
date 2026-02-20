Crate Features: `alloc` and `std`

# Crate Features

The following two crate features are defined in this crate.

- `alloc`\
  Enables methods that use struct [`Vec`]`<T>`.  If this feature is
  enabled, this crate imports crate [`alloc`].  If feature `std` is
  enabled, this feature is also enabled.

- `std` (enabled by default)\
  Enables trait [`EncastIO`] and trait [`DecastIO`].  If this feature
  is enabled, this crate imports crate [`std::io`] and feature `alloc`
  is enabled.

By default, feature `std` is enabled.

# How to Use This Crate on a `no_std` Environment

## Without Memory Allocator

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

## With Memory Allocator

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
