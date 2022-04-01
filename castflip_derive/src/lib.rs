//!
//! This crate provides three derive macros for crate `castflip`.
//!
//! Please refer to the documentation of `castflip`
//! at <https://docs.rs/castflip/> for more information.
//!


mod cast;
mod flip;
mod nop_flip;

use proc_macro::TokenStream;


///
/// Declares that the succeeding struct or union type is `encast`able
/// / `decast`able.
///
/// Please refer to the description of trait [`Cast`] in the
/// documentation of `castflip` at <https://docs.rs/castflip/>
///
/// [`Cast`]: https://docs.rs/castflip/latest/castflip/trait.Cast.html
///
#[proc_macro_derive(Cast)]
pub fn cast_derive(input: TokenStream) -> TokenStream {
    cast::proc_tokens(input)
}


///
/// Declares that the succeeding struct type is `endian-flip`pable.
///
/// Please refer to the description of trait [`Flip`] in the
/// documentation of `castflip` at <https://docs.rs/castflip/>
///
/// [`Flip`]: https://docs.rs/castflip/latest/castflip/trait.Flip.html
///
#[proc_macro_derive(Flip)]
pub fn flip_derive(input: TokenStream) -> TokenStream {
    flip::proc_tokens(input)
}


///
/// Declares that the succeeding struct or union type is is marked as
/// `endian-flip`pable but the implemented operation is Nop (No
/// operation).
///
/// Please refer to the description of trait [`NopFlip`] in the
/// documentation of `castflip` at <https://docs.rs/castflip/>
///
/// [`NopFlip`]: https://docs.rs/castflip/latest/castflip/trait.NopFlip.html
///
#[proc_macro_derive(NopFlip)]
pub fn nop_flip_derive(input: TokenStream) -> TokenStream {
    nop_flip::proc_tokens(input)
}
