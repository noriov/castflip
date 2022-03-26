//!
//! Defines experimental traits and historical traits.
//!
//! Experimental traits (in used)
//!
//! - [`AsifBytes`] converts a reference to a variable or a slice into
//!   a phantom reference to a slice of bytes without copying data.
//!   Although it is useful in certain situations, its use cases will
//!   be limited because its methods are considered unsafe.
//!
//! - [`FlipUnsized`] provides method `flip_var` for slices and
//!   tuples.
//!
//! Historical traits (not used)
//!
//! - [`Deslice`], [`Enslice`] and [`Reslice`] are general-purpose
//!   ancestors of [`AsifBytes`].  Some of their methods were used
//!   internally in this crate.  They convert a reference to a
//!   variable or a slice into a phantom reference to a slice or a
//!   variable of the specified type without copying data.
//!
//! - [`EncastArg`] and [`DecastArg`] provide some older versions of
//!   APIs of this crate.  If no good use case can be found, they may
//!   be removed in a future release.
//!
//! - [`AsBytes`] is an older version of [`AsifBytes`].  It is
//!   deprecated because one of its method names is conflicted with
//!   the one of `mem::MaybeUninit`.  In order to support method
//!   `as_bytes_mut` for `mem::MaybeUninit`, and to avoid possible
//!   other name conflict, [`AsBytes`] is renamed to [`AsifBytes`].
//!   It will be removed in a future release.
//!


// Modules in files in the same directory.
#[doc(hidden)] pub mod as_bytes;
#[doc(hidden)] pub mod asif_bytes;
#[doc(hidden)] pub mod decast_arg;
#[doc(hidden)] pub mod deslice;
#[doc(hidden)] pub mod encast_arg;
#[doc(hidden)] pub mod enslice;
#[doc(hidden)] pub mod flip_unsized;
#[doc(hidden)] pub mod reslice;

// Experimental but internally used trait.
#[allow(deprecated)] #[doc(inline)] pub use self::as_bytes::AsBytes;
#[doc(inline)] pub use self::asif_bytes::AsifBytes;

// Experimental but historical traits.
#[doc(inline)] pub use self::deslice::Deslice;
#[doc(inline)] pub use self::enslice::Enslice;
#[doc(inline)] pub use self::reslice::Reslice;

// Historical traits.
#[doc(inline)] pub use self::decast_arg::DecastArg;
#[doc(inline)] pub use self::encast_arg::EncastArg;
#[doc(inline)] pub use self::flip_unsized::FlipUnsized;
