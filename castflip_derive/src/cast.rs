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
		    proc_named(&ast.ident, &fields_named),
		Fields::Unnamed(fields_unnamed) =>
		    proc_unnamed(&ast.ident, &fields_unnamed),
		Fields::Unit =>
		    proc_unit(&ast.ident),
	    },
	Data::Enum(_data_enum) =>
	    panic!("Trait Cast cannot be implemented for enum."),
	Data::Union(data_union) =>
	    proc_union(&ast.ident, &data_union.fields),
    }
}

fn proc_named(ident: &Ident, fields: &FieldsNamed) -> TokenStream {
    let field_type = fields.named.iter().map(|field| &field.ty);

    quote! {
	impl castflip::Cast for #ident
	where
	    #( #field_type: castflip::Cast, )*
	{}
    }.into()
}

fn proc_unnamed(ident: &Ident, fields: &FieldsUnnamed) -> TokenStream {
    let field_type = fields.unnamed.iter().map(|field| &field.ty);

    quote! {
	impl castflip::Cast for #ident
	where
	    #( #field_type: castflip::Cast, )*
	{}
    }.into()
}

fn proc_unit(ident: &Ident) -> TokenStream {
    quote! {
	impl castflip::Cast for #ident {}
    }.into()
}

fn proc_union(ident: &Ident, fields: &FieldsNamed) -> TokenStream {
    let field_type = fields.named.iter().map(|field| &field.ty);

    quote! {
	impl castflip::Cast for #ident
	where
	    #( #field_type: castflip::Cast, )*
	{}
    }.into()
}
