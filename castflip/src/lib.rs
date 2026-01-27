//
// Include the front page.
//
#![doc = include_doc!("front_page.md")]


//
// Declare lint levels and features.
//
#![warn(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]


// Make sure the endianness of the target system is supported by this crate.
#[cfg(not(any(target_endian = "little", target_endian = "big")))]
core::compile_error!(
    "The endianness of the target system is not supported by this crate."
);


//
// Import an external module.
//
#[cfg(feature = "alloc")]
extern crate alloc;


//
// Export the current API (version 0.1)
//
pub use self::{
    bounds::{Cast, Flip, NopFlip},
    enum_endian::{BE, Endian, LE, NE, SE},
    trait_decast_mem::DecastMem,
    trait_encast_mem::EncastMem,
};
pub use castflip_derive::{Cast, Flip, NopFlip};

#[cfg(feature = "std")]
pub use self::{
    trait_encast_io::EncastIO,
    trait_decast_io::DecastIO,
};


//
// Import local modules.
//
pub mod experimental;

mod bounds;
mod enum_endian;
mod trait_decast_mem;
mod trait_encast_mem;

#[cfg(feature = "std")]
mod trait_decast_io;
#[cfg(feature = "std")]
mod trait_encast_io;

#[cfg(doc)]
pub mod documents;
