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
            panic!("The derive macro Flip does not support enum."),
        Data::Union(_data_union) =>
            panic!("The derive macro Flip does not support union."),
    }
}

// e.g. struct Ident { field1: Type1, field2: Type2, ... }
fn proc_named_struct(ident: &Ident, fields: &FieldsNamed) -> TokenStream {
    let field_type = fields.named.iter().map(|field| &field.ty);
    let field_name1 = fields.named.iter().map(|field| &field.ident);
    let field_name2 = fields.named.iter().map(|field| &field.ident);
    let field_name3 = fields.named.iter().map(|field| &field.ident);

    quote! {
        impl castflip::Flip for #ident
        where
            #( #field_type: castflip::Flip, )*
        {
            fn flip_val_swapped(&self) -> Self {
                Self {
                    #(
                        #field_name1: self. #field_name2 .flip_val_swapped(),
                    )*
                }
            }
            fn flip_var_swapped(&mut self) {
                #(
                    self. #field_name3 .flip_var_swapped();
                )*
            }
        }
    }.into()
}

// e.g. struct Ident ( Type1, Type2, ... );
fn proc_unnamed_struct(ident: &Ident, fields: &FieldsUnnamed) -> TokenStream {
    let field_type = fields.unnamed.iter().map(|field| &field.ty);
    let field_index1 = ( 0 .. fields.unnamed.len() ).map(syn::Index::from);
    let field_index2 = ( 0 .. fields.unnamed.len() ).map(syn::Index::from);

    quote! {
        impl castflip::Flip for #ident
        where
            #( #field_type: castflip::Flip, )*
        {
            fn flip_val_swapped(&self) -> Self {
                Self (
                    #(
                        self. #field_index1 .flip_val_swapped(),
                    )*
                )
            }
            fn flip_var_swapped(&mut self) {
                #(
                    self. #field_index2 .flip_var_swapped();
                )*
            }
        }
    }.into()
}

// e.g. struct Ident;
fn proc_unit_struct(ident: &Ident) -> TokenStream {
    quote! {
        impl castflip::Flip for #ident {
            fn flip_val_swapped(&self) -> Self {
                Self
            }
            fn flip_var_swapped(&mut self) {}
        }
    }.into()
}
