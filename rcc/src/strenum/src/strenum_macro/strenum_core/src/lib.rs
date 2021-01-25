use proc_macro2::Ident;
use proc_macro2::Span;
use quote::*;
use std::fmt::Formatter;
use syn::spanned::Spanned;
use syn::{parse2, DataEnum, DeriveInput, Expr};

const STR_ENUM: &str = "strenum";
const DISABLED: &str = "disabled";

/// add `from_str` and `fmt` method for deriving enum
pub fn add_impl_items(ast: syn::DeriveInput) -> proc_macro2::TokenStream {
    let mut strs: Vec<String> = vec![];
    let mut enums: Vec<Ident> = vec![];
    let DeriveInput {
        generics,
        data,
        attrs,
        ident,
        vis,
    } = ast;

    match data {
        syn::Data::Enum(DataEnum { variants, .. }) => {
            for v in variants {
                if v.attrs.is_empty() {
                    strs.push(v.ident.to_string().to_lowercase());
                    enums.push(format_ident!("{}", v.ident));
                } else {
                    for attr in v.attrs {
                        let path = attr.path.get_ident().unwrap();
                        if path == STR_ENUM {
                            let tks = attr.tokens;
                            if let Ok(res) = syn::parse2::<syn::ExprParen>(tks) {
                                let expr = res.expr.as_ref();
                                let tks = expr.to_token_stream();
                                if let Ok(ident) = syn::parse2::<syn::Ident>(tks.clone()) {
                                    if ident.to_string() != DISABLED {
                                        let span = Span::call_site();
                                        return quote_spanned! {
                                            span => compiler_error!("ident must be disabled");
                                        };
                                    }
                                } else if let Ok(str_literal) = syn::parse2::<syn::LitStr>(tks) {
                                    strs.push(str_literal.value());
                                    enums.push(format_ident!("{}", v.ident));
                                } else {
                                    let span = Span::call_site();
                                    return quote_spanned! {
                                        span => compiler_error!("must be disabled or string literal");
                                    };
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => {
            let span = Span::call_site();
            return quote_spanned! {
                span => compiler_error!("must be enum");
            };
        }
    }

    quote! (
        impl#generics std::str::FromStr for #ident#generics {
            type Err = ();
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    #( #strs => Ok(Self::#enums), )*
                    _ => Err(()),
                }
            }
        }

        impl#generics std::fmt::Display for #ident#generics {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #( Self::#enums => write!(f, "{}", #strs), )*
                    _ => Err(std::fmt::Error {})
                }
            }
        }
    )
}

#[test]
fn add_from_str_test() {
    use quote::quote;

    let tokens = quote! {
        enum Color<'a, T> {
            #[strenum(disabled)]
            Red(T, &'a str),
            Yellow,
            #[strenum("+")]
            Orange
        }
    };
    let tokens = add_impl_items(syn::parse2(tokens).unwrap());
    println!("{}", tokens);
}

#[test]
fn bar_test() {
    let tokens = quote! {
        struct Foo {
            id: i32,
            name: String
        }
    };
    let tokens = add_impl_items(syn::parse2(tokens).unwrap());
    println!("{}", tokens);
}
