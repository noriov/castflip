#![doc = include_str!("front_page.md")]


use proc_macro::TokenStream;


mod cast;
mod flip;
mod nop_flip;


///
/// Implements trait [`Cast`] for a `struct` type or a `union` type.
///
/// It must be applied together with attribute `#[`[`repr(C)`]`]`.
///
/// For detailed information, see the document of trait [`Cast`].
///
/// [`Cast`]: https://docs.rs/castflip/0.1/castflip/trait.Cast.html
/// [`repr(C)`]: https://doc.rust-lang.org/reference/type-layout.html#the-c-representation
///
#[proc_macro_derive(Cast)]
pub fn cast_derive(input: TokenStream) -> TokenStream {
    cast::proc_tokens(input)
}


///
/// Implements trait [`Flip`] for a `struct` type.
///
/// For detailed information, see the document of trait [`Flip`].
///
/// [`Flip`]: https://docs.rs/castflip/0.1/castflip/trait.Flip.html
///
#[proc_macro_derive(Flip)]
pub fn flip_derive(input: TokenStream) -> TokenStream {
    flip::proc_tokens(input)
}


///
/// Implements trait [`NopFlip`] for a `struct` type or a `union` type.
///
/// For detailed information, see the document of trait [`NopFlip`].
///
/// [`NopFlip`]: https://docs.rs/castflip/0.1/castflip/trait.NopFlip.html
///
#[proc_macro_derive(NopFlip)]
pub fn nop_flip_derive(input: TokenStream) -> TokenStream {
    nop_flip::proc_tokens(input)
}
