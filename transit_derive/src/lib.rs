extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn;

#[proc_macro_derive(TransitSerialize)]
pub fn transit_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_transit_macro(&ast)
}

fn process_struct_named(
    name: &Ident,
    tag: String,
    fields: &syn::FieldsNamed,
) -> proc_macro2::TokenStream {
    let fields_named = fields
        .named
        .iter()
        .map(|x| (&x.ident).clone().expect("Requires identifier"));
    let fields_str = fields_named.clone().map(|x| x.to_string());
    let len = fields_named.len();

    quote! {
        impl TransitSerialize for #name {
            const TF_TYPE: TransitType = TransitType::Composite;
            fn transit_serialize<S: TransitSerializer>(&self, serializer: S)
                -> S::Output {
                let mut ser_map = serializer
                    .clone()
                    .serialize_tagged_map(#tag, Some(#len));
                #(ser_map.serialize_pair(#fields_str, self.#fields_named.clone());)*
                ser_map.end()
            }
            fn transit_serialize_key<S: TransitSerializer>(&self, serializer: S)
                -> Option<S::Output> {
                None
            }
        }
    }
}

fn process_struct_unnamed(
    name: &Ident,
    tag: String,
    fields: &syn::FieldsUnnamed,
) -> proc_macro2::TokenStream {
    let len = fields.unnamed.len();
    let accessors = 0..len;

    quote! {
        impl TransitSerialize for #name {
            const TF_TYPE: TransitType = TransitType::Composite;
            fn transit_serialize<S: TransitSerializer>(&self, serializer: S)
                -> S::Output {
                let mut ser_arr = serializer
                    .clone()
                    .serialize_tagged_array(#tag, Some(#len));
                #(ser_arr.serialize_item(self.#accessors);)*
                ser_arr.end()
            }
            fn transit_serialize_key<S: TransitSerializer>(&self, serializer: S)
                -> Option<S::Output> {
                None
            }
        }
    }
}

fn impl_transit_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name: &Ident = &ast.ident;
    let tag = format!("~#{}", name).to_lowercase();
    let gen = match &ast.data {
        syn::Data::Struct(ds) => match &ds.fields {
            syn::Fields::Named(fields) => process_struct_named(name, tag, fields),
            syn::Fields::Unnamed(fields) => process_struct_unnamed(name, tag, fields),
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };
    gen.into()
}
