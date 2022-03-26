use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Ident};


pub fn proc_tokens(input: TokenStream) -> TokenStream {
    // Parse TokenStream into the Rust Abstract Syntax Tree (AST).
    let ast: DeriveInput = syn::parse(input).unwrap();

    match &ast.data {
	Data::Struct(data_struct) =>
	    match &data_struct.fields {
		Fields::Named(fields_named) =>
		    proc_named_struct(&ast.ident, fields_named),
		Fields::Unnamed(fields_unnamed) =>
		    proc_unnamed_struct(&ast.ident, fields_unnamed),
		Fields::Unit =>
		    proc_unit_struct(&ast.ident),
	    },
	Data::Enum(_data_enum) =>
	    panic!("Trait NopFlip cannot be implemented for enum."),
	Data::Union(data_union) =>
	    proc_union(&ast.ident, &data_union.fields),
    }
}

// e.g. struct Ident { field1: Type1, field2: Type2, ... }
fn proc_named_struct(ident: &Ident, fields: &FieldsNamed) -> TokenStream {
    let field_type = fields.named.iter().map(|field| &field.ty);

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

// e.g. struct Ident ( Type1, Type2, ... );
fn proc_unnamed_struct(ident: &Ident, fields: &FieldsUnnamed) -> TokenStream {
    let field_type = fields.unnamed.iter().map(|field| &field.ty);

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

// e.g. struct Ident;
fn proc_unit_struct(ident: &Ident) -> TokenStream {
    quote! {
	impl castflip::Flip for #ident
	{
	    fn flip_val_swapped(&self) -> Self {
		Self
	    }
	    fn flip_var_swapped(&mut self) {}
	}
	impl castflip::NopFlip for #ident {}
    }.into()
}

// e.g. union Ident { field1: Type1, field2: Type2, ... }
fn proc_union(ident: &Ident, fields: &FieldsNamed) -> TokenStream {
    let field_type = fields.named.iter().map(|field| &field.ty);

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
