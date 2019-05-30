extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(TransitSerialize)]
pub fn transit_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_transit_macro(&ast)
}

fn impl_transit_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let fields = match &ast.data {
        syn::Data::Struct(ds) => match &ds.fields {
            syn::Fields::Named(f) => f.named.iter(),
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
    .map(|x| (&x.ident).clone().expect("Requires identifier"));
    let fields_str = fields.clone().map(|x| x.to_string());
    let len = fields.len();
    let tag = format!("~#{}", name).to_lowercase();
    let gen = quote! {
        impl TransitSerialize for #name {
            const TF_TYPE: TransitType = TransitType::Composite;
            fn transit_serialize<S: TransitSerializer>(&self, serializer: S)
                -> S::Output {
                let mut ser_map = serializer
                    .clone()
                    .serialize_tagged_map(#tag, Some(#len));
                #(ser_map.serialize_pair(#fields_str, self.#fields.clone());)*
                ser_map.end()
            }
            fn transit_serialize_key<S: TransitSerializer>(&self, serializer: S)
                -> Option<S::Output> {
                None
            }
        }
    };
    gen.into()
}
