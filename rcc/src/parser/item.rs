use crate::ast::expr::BlockExpr;
use crate::ast::item::{
    ExternalItem, ExternalItemFn, FnParam, FnParams, Item, ItemExternalBlock, ItemFn, ItemStruct,
    StructField, TupleField, TypeEnum, ABI,
};
use crate::ast::pattern::Pattern;
use crate::ast::types::TypeAnnotation;
use crate::ast::{TokenStart, Visibility};
use crate::lexer::token::Token;
use crate::parser::expr::primitive::parse_lit_string;
use crate::parser::{Parse, ParseCursor};
use crate::rcc::RccError;
use std::string::ToString;

impl Parse for Item {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        let vis = Visibility::parse(cursor)?;

        match cursor.next_token()? {
            Token::Fn => Ok(Self::Fn(ItemFn::parse_with_attr(cursor, vis)?)),
            Token::Struct => Ok(Self::Struct(ItemStruct::parse_with_attr(cursor, vis)?)),
            Token::Enum => Ok(Self::Enum(TypeEnum::parse_with_attr(cursor, vis)?)),
            Token::Static => unimplemented!(),
            Token::Const => unimplemented!(),
            Token::Impl => unimplemented!(),
            _ => unreachable!("inner item must be fn, struct, enum, static, const or impl"),
        }
    }
}

/// Parse struct definition
/// ItemStruct -> struct Identifier ; | TupleField ; | StructField
impl ItemStruct {
    fn parse_with_attr(cursor: &mut ParseCursor, vis: Visibility) -> Result<Self, RccError> {
        debug_assert!(cursor.next_token()? == &Token::Struct);
        cursor.bump_token()?;
        let ident = cursor.bump_token()?;
        if let Token::Identifier(struct_name) = ident {
            let type_struct = Self::new(vis, struct_name.to_string());
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

impl TypeEnum {
    fn parse_with_attr(cursor: &mut ParseCursor, vis: Visibility) -> Result<Self, RccError> {
        unimplemented!()
    }
}

fn parse_fn_signature(
    cursor: &mut ParseCursor,
) -> Result<(String, FnParams, TypeAnnotation), RccError> {
    cursor.eat_token_eq(Token::Fn)?;
    let fn_name = cursor.eat_identifier()?.to_string();

    cursor.eat_token_eq(Token::LeftParen)?;
    let fn_params = if cursor.eat_token_if_eq(Token::RightParen) {
        FnParams::new()
    } else {
        let fn_params = FnParams::parse(cursor)?;
        cursor.eat_token_eq(Token::RightParen)?;
        fn_params
    };

    let ret_type = match cursor.next_token()? {
        Token::RArrow => {
            cursor.bump_token()?;
            TypeAnnotation::parse(cursor)?
        }
        Token::Semi | Token::LeftCurlyBraces => TypeAnnotation::Unit,
        _ => return Err("except '->' or '{'".into()),
    };
    Ok((fn_name, fn_params, ret_type))
}

/// ItemFn -> vis? `fn` identifier `(` FnParams? `)` ( `->` Type )? BlockExpr
impl ItemFn {
    fn parse_with_attr(cursor: &mut ParseCursor, vis: Visibility) -> Result<Self, RccError> {
        let (fn_name, fn_params, ret_type) = parse_fn_signature(cursor)?;
        let fn_block = BlockExpr::parse(cursor)?;
        Ok(ItemFn::new(vis, fn_name, fn_params, ret_type, fn_block))
    }
}

/// FnParams -> FnParam (, FnParam)* ,?
impl Parse for FnParams {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        let mut fn_params: FnParams = vec![FnParam::parse(cursor)?].into();
        while cursor.eat_token_if_eq(Token::Comma) {
            if FnParam::is_token_start(cursor.next_token()?) {
                fn_params.push(FnParam::parse(cursor)?);
            } else {
                break;
            }
        }
        Ok(fn_params)
    }
}

/// FnParam -> Pattern `:` Type
impl Parse for FnParam {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        let ptn = Pattern::parse(cursor)?;
        cursor.eat_token_eq(Token::Colon)?;
        let _type = TypeAnnotation::parse(cursor)?;
        Ok(FnParam::new(ptn, _type))
    }
}

/// ExternalItem -> extern ABI { ExternalItem* }
impl Parse for ItemExternalBlock {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        cursor.eat_token_eq(Token::Extern)?;
        let abi = ABI::parse(cursor)?;
        cursor.eat_token_eq(Token::LeftCurlyBraces)?;
        let mut external_items = vec![];
        while cursor.next_token()? != &Token::RightCurlyBraces {
            external_items.push(ExternalItem::parse(cursor)?);
        }
        cursor.eat_token_eq(Token::RightCurlyBraces)?;
        Ok(ItemExternalBlock::new(abi, external_items))
    }
}

impl Parse for ABI {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        let s = parse_lit_string(cursor)?;
        ABI::from_string(&s)
    }
}
/// ExternalItem -> ExternalItemFn
impl Parse for ExternalItem {
    fn parse(cursor: &mut ParseCursor) -> Result<Self, RccError> {
        let vis = Visibility::parse(cursor)?;
        match cursor.next_token()? {
            Token::Fn => {
                let external_fn = ExternalItemFn::parse_after_vis(cursor, vis)?;
                Ok(ExternalItem::Fn(external_fn))
            }
            _ => {
                todo!()
            }
        }
    }
}

/// ExternalItemFn -> vis? `fn` identifier `(` FnParams? `)` ( `->` Type )? `;`
impl ExternalItemFn {
    fn parse_after_vis(
        cursor: &mut ParseCursor,
        vis: Visibility,
    ) -> Result<ExternalItemFn, RccError> {
        let (fn_name, fn_params, ret_type) = parse_fn_signature(cursor)?;
        cursor.eat_token_eq(Token::Semi)?;
        Ok(ExternalItemFn::new(vis, fn_name, fn_params, ret_type))
    }
}
