#![crate_type = "proc-macro"]
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FieldsNamed, FieldsUnnamed, DataEnum, DataUnion, DeriveInput};

#[proc_macro_derive(AnswerFn, attributes(disabled))]
pub fn derive_answer_fn(input: TokenStream) -> TokenStream {
    let DeriveInput{ident, data, ..} = parse_macro_input!(input);

    let description = match data {
        syn::Data::Enum(DataEnum { variants, .. }) => {

            let vs = variants.iter().map(|v| (v.attrs.len()));
            format!("an enum with these variants: {}", quote! {#(#vs),*})
        }
        _ => "not enum".to_string()
    };

    let output = quote! {
        impl #ident {
            fn describe() {
                println!("{} is {}.", stringify!(#ident), #description);
            }
        }
    };

    output.into()
}


