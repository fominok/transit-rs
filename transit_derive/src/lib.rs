extern crate proc_macro;

use proc_macro2::{Span, Ident, TokenStream};
use quote::quote;
use syn;

#[proc_macro_derive(TransitSerialize)]
pub fn transit_macro_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_transit_macro(&ast)
}

fn put_serialize_struct_body(name: &Ident, body: TokenStream) -> TokenStream {
    quote! {
        impl TransitSerialize for #name {
            const TF_TYPE: TransitType = TransitType::Composite;
            fn transit_serialize<S: TransitSerializer>(&self, serializer: S)
                -> S::Output {
                #body
            }
            fn transit_serialize_key<S: TransitSerializer>(&self, serializer: S)
                -> Option<S::Output> {
                None
            }
        }
    }
}

fn named_body(tag: String, fields: &syn::FieldsNamed) -> TokenStream {
    let fields_named = fields
        .named
        .iter()
        .map(|x| (&x.ident).clone().expect("Requires identifier"));
    let fields_str = fields_named.clone().map(|x| x.to_string());
    let len = fields_named.len();

    quote! {
        let mut ser_map = serializer
            .clone()
            .serialize_tagged_map(#tag, Some(#len));
        #(ser_map.serialize_pair(#fields_str, self.#fields_named.clone());)*
        ser_map.end()
    }
}

fn unnamed_body(tag: String, fields: &syn::FieldsUnnamed) -> TokenStream {
    let len = fields.unnamed.len();
    let accessors = 0..len;
    quote! {
        let mut ser_arr = serializer
            .clone()
            .serialize_tagged_array(#tag, Some(#len));
        #(ser_arr.serialize_item(self.#accessors);)*
        ser_arr.end()
    }
}

fn process_struct_named(name: &Ident, tag: String, fields: &syn::FieldsNamed) -> TokenStream {
    let body = named_body(tag, fields);
    put_serialize_struct_body(name, body)
}

fn process_struct_unnamed(name: &Ident, tag: String, fields: &syn::FieldsUnnamed) -> TokenStream {
    let body = unnamed_body(tag, fields);
    put_serialize_struct_body(name, body)
}

fn process_enum(name: &Ident, variants: &[TokenStream]) -> TokenStream {
    quote! {
        impl TransitSerialize for #name {
            const TF_TYPE: TransitType = TransitType::Composite;

            fn transit_serialize<S: TransitSerializer>(&self, serializer: S) -> S::Output {
                match self {
                    #(#variants),*
                }
            }

            fn transit_serialize_key<S: TransitSerializer>(
                &self,
                serializer: S,
            ) -> Option<S::Output> {
                None
            }
        }
    }
}

fn process_enum_variants(
    name: &Ident,
    variants: &syn::punctuated::Punctuated<syn::Variant, syn::token::Comma>,
) -> Vec<TokenStream> {
    let mut quoted_variants: Vec<TokenStream> = vec![];
    for v in variants.iter() {
        let tag = format!("~#{}", v.ident).to_lowercase();
        match &v.fields {
            syn::Fields::Named(fields) => {
                let fields_named = fields
                    .named
                    .iter()
                    .map(|x| (&x.ident).clone().expect("Requires identifier"));
                let fields_named2 = fields_named.clone();

                let fields_str = fields_named.clone().map(|x| x.to_string());
                let len = fields_named.len();

                let body = quote! {
                    let mut ser_map = serializer
                        .clone()
                        .serialize_tagged_map(#tag, Some(#len));
                    #(ser_map.serialize_pair(#fields_str, #fields_named.clone());)*
                    ser_map.end()
                };
                let vident = v.clone().ident;
                let arm = quote! {
                    #name::#vident {#(#fields_named2),*} => {#body}
                };
                quoted_variants.push(arm);
            }
            syn::Fields::Unnamed(fields) => {
                let len = fields.unnamed.len();
                let accessors = (0..len).map(|x| {
                    let ident = format!("syn{}", x);
                    syn::Ident::new(&ident, Span::call_site())
                });
                let accessors2 = accessors.clone();

                let body = quote! {
                    let mut ser_arr = serializer
                        .clone()
                        .serialize_tagged_array(#tag, Some(#len));
                    #(ser_arr.serialize_item(#accessors.clone());)*
                    ser_arr.end()
                };
                let vident = v.clone().ident;
                let arm = quote! {
                    #name::#vident (#(#accessors2),*) => {#body}
                };
                quoted_variants.push(arm);
            }
            _ => unimplemented!(),
        }
    }
    quoted_variants
}

fn impl_transit_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let name: &Ident = &ast.ident;
    let tag = format!("~#{}", name).to_lowercase();
    let gen = match &ast.data {
        syn::Data::Struct(ds) => match &ds.fields {
            syn::Fields::Named(fields) => process_struct_named(name, tag, fields),
            syn::Fields::Unnamed(fields) => process_struct_unnamed(name, tag, fields),
            _ => unimplemented!(),
        },
        syn::Data::Enum(de) => process_enum(name, &process_enum_variants(name, &de.variants)),
        _ => unimplemented!(),
    };
    gen.into()
}
