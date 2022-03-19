//!
//! This crate provides three derive macros for crate `castflip`.
//!
//! Please refer to the documentation of `castflip`.
//!


mod cast;
mod flip;
mod nop_flip;

use proc_macro::TokenStream;


///
/// Declares that the structure is `encast`able / `decast`able.
///
/// Please refer to the description of trait `Cast` in the
/// documentation of `castflip`.
///
#[proc_macro_derive(Cast)]
pub fn cast_derive(input: TokenStream) -> TokenStream {
    cast::proc_tokens(input)
}


///
/// Declares that the structure is endian-`Flip`pable.
///
/// Please refer to the description of trait `Flip` in the
/// documentation of `castflip`.
///
#[proc_macro_derive(Flip)]
pub fn flip_derive(input: TokenStream) -> TokenStream {
    flip::proc_tokens(input)
}


///
/// Declares that the structure is marked as endian-`Flip`pable but
/// the implemented operation is Nop (No operation).
///
/// Please refer to the description of trait `NopFlip` in the
/// documentation of `castflip`.
///
#[proc_macro_derive(NopFlip)]
pub fn nop_flip_derive(input: TokenStream) -> TokenStream {
    nop_flip::proc_tokens(input)
}
