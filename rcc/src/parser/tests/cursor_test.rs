use crate::parser::tests::get_parser;
use crate::lexer::token::Token;

#[test]
fn cursor_test() {
    let mut cursor = get_parser("a - b");
    assert_eq!(Ok("a"), cursor.eat_identifier()) ;
    assert!(cursor.eat_token_in(&[Token::Minus, Token::Le]).is_ok());
}