#![crate_type = "proc-macro"]

extern crate proc_macro;
use strenum_core::add_from_str;
use syn::parse_macro_input;

#[proc_macro_derive(EnumFromStr, attributes(disabled, value))]
pub fn derive_answer_fn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    add_from_str(parse_macro_input!(input)).into()
}
