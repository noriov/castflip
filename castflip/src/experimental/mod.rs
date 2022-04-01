//!
//! Defines experimental traits and historical traits.
//!
//! An experimental trait.  It is used internally in this crate.
//!
//! - [`AsifBytes`] converts a reference to a variable or a slice into
//!   a phantom reference to a slice of bytes without copying data.
//!
//! Historical traits.  Because they are not used in recent releases,
//! they may be removed in a future release.
//!
//! - [`EncastArg`] and [`DecastArg`] provide some older versions of
//!   APIs of this crate.
//!
//! - [`FlipUnsized`] provides method `flip_var` for a slice and
//!   a tuple.
//!
//! - [`Deslice`], [`Enslice`] and [`Reslice`] are general-purpose
//!   ancestors of [`AsifBytes`].  They convert a reference to a
//!   variable or a slice into a phantom reference to a slice or a
//!   variable of the specified type without copying data.  Although
//!   it is useful in certain situations, its use cases will be
//!   limited because its methods are considered unsafe.
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
