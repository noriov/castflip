//!
//! Defines experimental traits and historical traits.
//!
//! - [`AsBytes`] converts a reference to a variable or a slice into a
//!   phantom reference to bytes without copying data.  Although it is
//!   useful in certain situations, its use cases will be limited
//!   because its methods are considered unsafe.  It is used
//!   internally in this crate.
//!
//! - [`Deslice`], [`Enslice`] and [`Reslice`] are general-purpose
//!   ancestors of [`AsBytes`].  Some of their methods were used
//!   internally in this crate.  They convert a reference to a
//!   variable or a slice into a phantom reference to a slice or a
//!   variable of the specified type without copying data.
//!
//! - [`EncastArg`], [`DecastArg`] and [`FlipUnsized`] provides some
//!   older versions of APIs of this crate.  If no good use case can
//!   be found, they may be removed in a future release.
//!


// Modules in files in the same directory.
#[doc(hidden)] pub mod as_bytes;
#[doc(hidden)] pub mod decast_arg;
#[doc(hidden)] pub mod deslice;
#[doc(hidden)] pub mod encast_arg;
#[doc(hidden)] pub mod enslice;
#[doc(hidden)] pub mod flip_unsized;
#[doc(hidden)] pub mod reslice;

// Experimental but internally used trait.
#[doc(inline)] pub use self::as_bytes::AsBytes;

// Experimental but historical traits.
#[doc(inline)] pub use self::deslice::Deslice;
#[doc(inline)] pub use self::enslice::Enslice;
#[doc(inline)] pub use self::reslice::Reslice;

// Historical traits.
#[doc(inline)] pub use self::decast_arg::DecastArg;
#[doc(inline)] pub use self::encast_arg::EncastArg;
#[doc(inline)] pub use self::flip_unsized::FlipUnsized;
