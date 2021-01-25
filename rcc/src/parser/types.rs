use crate::ast::item::{StructField, TupleField, TypeEnum};
use crate::ast::types::{Type, TypeArray, TypeFnPtr, TypePtr, TypeSlice, TypeTuple};
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

impl<'a> Parse<'a> for TypeArrayOrSlice {
    fn parse(cursor: &mut ParseCursor<'a>) -> Result<Self, RccError> {
        unimplemented!()
    }
}

impl<'a> Parse<'a> for Type {
    fn parse(cursor: &mut ParseCursor<'a>) -> Result<Self, RccError> {
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
            Token::Ref | Token::Star => Ok(Self::Ptr(TypePtr::parse(cursor)?)),
            _ => Err("invalid type".into())
        }
    }
}

impl<'a> Parse<'a> for TypeTuple {
    fn parse(cursor: &mut ParseCursor<'a>) -> Result<Self, RccError> {
        unimplemented!()
    }
}

impl<'a> Parse<'a> for TypeArray {
    fn parse(cursor: &mut ParseCursor<'a>) -> Result<Self, RccError> {
        unimplemented!()
    }
}

impl<'a> Parse<'a> for TypeFnPtr {
    fn parse(cursor: &mut ParseCursor<'a>) -> Result<Self, RccError> {
        unimplemented!()
    }
}

impl<'a> Parse<'a> for TypePtr {
    fn parse(cursor: &mut ParseCursor<'a>) -> Result<Self, RccError> {
        unimplemented!()
    }
}

impl<'a> Parse<'a> for Vec<StructField> {
    fn parse(cursor: &mut ParseCursor<'a>) -> Result<Self, RccError> {
        unimplemented!()
    }
}

impl<'a> Parse<'a> for Vec<TupleField> {
    fn parse(cursor: &mut ParseCursor<'a>) -> Result<Self, RccError> {
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
            let _type = Type::parse(cursor)?;
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
