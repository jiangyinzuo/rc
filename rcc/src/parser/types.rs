use crate::ast::item::{StructField, TupleField, TypeEnum};
use crate::ast::types::TypeAnnotation::Ptr;
use crate::ast::types::{
    PtrKind, TypeAnnotation, TypeArray, TypeFnPtr, TypePtr, TypeSlice, TypeTuple,
};
use crate::ast::Visibility;
use crate::lexer::token::Token;
use crate::lexer::token::Token::{Comma, LeftParen, RightParen, Semi};
use crate::parser::{Parse, ParseCursor};
use crate::rcc::RccError;

/// Used for parsing array or slice
#[derive(Debug, PartialEq)]
enum TypeArrayOrSlice {
    Array(TypeArray),
    Slice(TypeSlice),
}

impl Parse for TypeArrayOrSlice {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        unimplemented!()
    }
}

impl Parse for TypeAnnotation {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        match cursor.bump_token()? {
            Token::Identifier(s) => Ok(Self::Identifier(s.to_string())),
            Token::LeftParen => Ok(Self::Tuple(TypeTuple::parse(cursor)?)),
            Token::LeftSquareBrackets => {
                let arr_or_slice = TypeArrayOrSlice::parse(cursor)?;
                match arr_or_slice {
                    TypeArrayOrSlice::Array(arr) => Ok(Self::Array(arr)),
                    TypeArrayOrSlice::Slice(slice) => Ok(Self::Slice(slice)),
                }
            }
            Token::Fn => Ok(Self::FnPtr(TypeFnPtr::parse(cursor)?)),
            Token::Not => Ok(Self::Never),
            tk if matches!(tk, Token::And | Token::AndAnd | Token::Star) => {
                let tk = tk.clone();
                Ok(Self::Ptr(TypePtr::parse_from_first(cursor, tk)?))
            }
            tk => Err(format!("invalid token `{:?}` for type annotation", tk).into()),
        }
    }
}

impl Parse for TypeTuple {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        unimplemented!()
    }
}

impl Parse for TypeArray {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        unimplemented!()
    }
}

impl Parse for TypeFnPtr {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        unimplemented!()
    }
}

impl TypePtr {
    pub fn parse_from_first(cursor: &mut ParseCursor, first_tk: Token) -> Result<Self, RccError> {
        let next_is_mut = cursor.eat_token_if_eq(Token::Mut);

        let parse_and = |cursor: &mut ParseCursor| {
            Ok(TypePtr::new(
                if next_is_mut {
                    PtrKind::MutRef
                } else {
                    PtrKind::Ref
                },
                TypeAnnotation::parse(cursor)?,
            ))
        };

        match first_tk {
            Token::And => parse_and(cursor),
            Token::AndAnd => Ok(TypePtr::new(
                PtrKind::Ref,
                TypeAnnotation::Ptr(parse_and(cursor)?),
            )),
            Token::Star => Ok(TypePtr::new(
                if next_is_mut {
                    PtrKind::MutRawPtr
                } else {
                    cursor.eat_token_eq(Token::Const)?;
                    PtrKind::ConstRawPtr
                },
                TypeAnnotation::parse(cursor)?,
            )),
            _ => Err("invalid token of type ptr".into()),
        }
    }
}

impl Parse for Vec<StructField> {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        unimplemented!()
    }
}

impl Parse for Vec<TupleField> {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        if cursor.bump_token()? != &LeftParen {
            return Err("invalid tuple field: except '('".into());
        }

        let mut tuple_fields = vec![];
        if cursor.next_token()? == &RightParen {
            cursor.bump_token()?;
            return Ok(tuple_fields);
        }

        loop {
            let vis = Visibility::parse(cursor)?;
            let _type = TypeAnnotation::parse(cursor)?;
            tuple_fields.push(TupleField { vis, _type });
            match cursor.bump_token()? {
                Comma => {}
                RightParen => break,
                _ => return Err("invalid tuple field: except ','".into()),
            }
        }
        Ok(tuple_fields)
    }
}
