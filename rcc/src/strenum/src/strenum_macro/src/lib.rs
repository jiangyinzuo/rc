#![crate_type = "proc-macro"]

extern crate proc_macro;
use strenum_core::add_impl_items;
use syn::parse_macro_input;

#[proc_macro_derive(StrEnum, attributes(strenum))]
pub fn derive_str_enum(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    add_impl_items(parse_macro_input!(input)).into()
}
