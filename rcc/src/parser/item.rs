use crate::ast::expr::BlockExpr;
use crate::ast::item::{InnerItem, ItemFn, ItemStruct, StructField, TupleField, TypeEnum, VisItem};
use crate::ast::types::Type;
use crate::ast::Visibility;
use crate::lexer::token::Token;
use crate::parser::{Parse, ParseCursor};
use crate::rcc::RccError;
use std::string::ToString;

impl Parse for VisItem {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        match cursor.next_token()? {
            Token::Pub => {
                cursor.bump_token()?;
                Ok(VisItem {
                    vis: Visibility::Pub,
                    inner_item: InnerItem::parse(cursor)?,
                })
            }
            Token::Fn
            | Token::Struct
            | Token::Enum
            | Token::Const
            | Token::Static
            | Token::Impl => Ok(VisItem {
                vis: Visibility::Priv,
                inner_item: InnerItem::parse(cursor)?,
            }),
            _ => Err("invalid vis item".into()),
        }
    }
}

impl Parse for InnerItem {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        match cursor.next_token()? {
            Token::Fn => Ok(Self::Fn(ItemFn::parse(cursor)?)),
            Token::Struct => Ok(Self::Struct(ItemStruct::parse(cursor)?)),
            Token::Enum => Ok(Self::Enum(TypeEnum::parse(cursor)?)),
            Token::Static => unimplemented!(),
            Token::Const => unimplemented!(),
            Token::Impl => unimplemented!(),
            _ => unreachable!("inner item must be fn, struct, enum, static, const or impl"),
        }
    }
}

/// Parse struct definition
/// ItemStruct -> struct Identifier ; | TupleField ; | StructField
impl Parse for ItemStruct {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        debug_assert!(cursor.next_token()? == &Token::Struct);
        cursor.bump_token()?;
        let ident = cursor.bump_token()?;
        if let Token::Identifier(struct_name) = ident {
            let type_struct = Self::new(struct_name.to_string());
            match cursor.next_token()? {
                // struct Foo;
                Token::Semi => Ok(type_struct),
                // struct Foo(i32);
                Token::LeftParen => {
                    let tuple_fields = Vec::<TupleField>::parse(cursor)?;
                    // eat semicolon
                    let tk = cursor.bump_token()?;
                    if tk == &Token::Semi {
                        Ok(type_struct.tuple_fields(tuple_fields))
                    } else {
                        Err("invalid struct definition(consider adding ';' after ')')".into())
                    }
                }
                // struct Foo {id: i32}
                Token::LeftCurlyBraces => {
                    let struct_fields = Vec::<StructField>::parse(cursor)?;
                    Ok(type_struct.struct_fields(struct_fields))
                }
                _ => Err("invalid struct definition".into()),
            }
        } else {
            Err("no identifier for struct".into())
        }
    }
}

impl Parse for TypeEnum {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        unimplemented!()
    }
}

impl Parse for ItemFn {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        cursor.eat_token(Token::Fn)?;
        let fn_name = cursor.eat_identifier()?;
        cursor.eat_token(Token::LeftParen)?;
        cursor.eat_token(Token::RightParen)?;
        match cursor.next_token()? {
            Token::RArrow => {
                cursor.bump_token()?;
                let ret_type = if cursor.next_token()? == &Token::LeftCurlyBraces {
                    Type::unit()
                } else {
                    Type::parse(cursor)?
                };

                let fn_block = BlockExpr::parse(cursor)?;
                Ok(ItemFn::new(fn_name.to_string(), ret_type, fn_block))
            }
            Token::Semi => unimplemented!("fn declaration without block not implemented"),
            Token::LeftCurlyBraces => {
                let fn_block = BlockExpr::parse(cursor)?;
                Ok(ItemFn::new(fn_name.to_string(), Type::unit(), fn_block))
            }
            _ => Err("except '->' or '{'".into()),
        }
    }
}
