use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Field, Fields, Ident,
	  punctuated::Punctuated, token::Comma};


pub fn proc_tokens(input: TokenStream) -> TokenStream {
    // Parse TokenStream into the Rust Abstract Syntax Tree (AST).
    let ast: DeriveInput = syn::parse(input).unwrap();

    match &ast.data {
	Data::Struct(data_struct) =>
	    match &data_struct.fields {
		Fields::Named(fields_named) =>
		    with_bounds(&ast.ident, &fields_named.named),
		Fields::Unnamed(fields_unnamed) =>
		    with_bounds(&ast.ident, &fields_unnamed.unnamed),
		Fields::Unit =>
		    without_bounds(&ast.ident),
	    },
	Data::Enum(_data_enum) =>
	    panic!("The derive macro NopFlip does not support enum."),
	Data::Union(data_union) =>
	    with_bounds(&ast.ident, &data_union.fields.named),
    }
}

fn with_bounds(ident: &Ident,
	       punctuated: &Punctuated<Field, Comma>) -> TokenStream {
    let field_type = punctuated.iter().map(|field| &field.ty);

    quote! {
	impl castflip::Flip for #ident
	where
	    #( #field_type: castflip::Flip, )*
	{
	    fn flip_val_swapped(&self) -> Self {
		unsafe {
		    ::core::ptr::read(self)
		}
	    }
	    fn flip_var_swapped(&mut self) {}
	}
	impl castflip::NopFlip for #ident {}
    }.into()
}

fn without_bounds(ident: &Ident) -> TokenStream {
    quote! {
	impl castflip::Flip for #ident
	{
	    fn flip_val_swapped(&self) -> Self {
		unsafe {
		    ::core::ptr::read(self)
		}
	    }
	    fn flip_var_swapped(&mut self) {}
	}
	impl castflip::NopFlip for #ident {}
    }.into()
}
