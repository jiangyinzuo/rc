use crate::lexer::token::Token;

pub mod file;
pub mod item;
pub mod types;
pub mod expr;
pub mod stmt;
pub mod pattern;
pub mod visit;

#[macro_export]
macro_rules! from_token {
    (
        #[$($attrs_pub:tt)*]
        pub enum $name:ident {
            $(
              $(#[$($attrs:tt)*])*
              $variant:ident,)*
        }
    ) => {
        #[$($attrs_pub)*]
        pub enum $name {
            $(
              $(#[$($attrs)*])*
              $variant,)*
        }

        impl crate::ast::FromToken for $name {
            fn from_token(tk: Token) -> Option<Self> {
                match tk {
                    $(Token::$variant => Some(Self::$variant),)*
                    _ => None,
                }
            }
        }
    };
}

pub trait FromToken: Sized {
    fn from_token(tk: Token) -> Option<Self>;
}


from_token! {
    #[derive(Debug, PartialEq, Clone)]
    pub enum Visibility {
        Pub,
        Priv,
    }
}


pub trait NamedASTNode {
    fn ident_name(&self) -> &str;
}

pub trait TokenStart {
    fn is_token_start(tk: &Token) -> bool;
}
