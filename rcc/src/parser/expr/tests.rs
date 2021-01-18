#[cfg(test)]
mod expr_tests {
    use crate::parser::expr::path_expr::PathExpr;
    use crate::{Parse, ParseContext};
    use lexer::Lexer;

    fn parse_context(input: &str) -> ParseContext {
        let mut lexer = Lexer::new(input);
        ParseContext::new(lexer.tokenize())
    }

    fn validate_path_expr(
        inputs: Vec<&str>,
        excepted_segments: Vec<std::result::Result<Vec<&str>, &'static str>>,
    ) {
        for (input, excepted) in inputs.into_iter().zip(excepted_segments) {
            let result = PathExpr::parse(parse_context(input));
            match excepted {
                Ok(segments) => assert_eq!(Ok(PathExpr { segments }), result),
                Err(s) => assert_eq!(excepted.unwrap_err(), s),
            }
        }
    }

    #[test]
    fn path_expr_test() {
        validate_path_expr(vec!["a::b::c"], vec![Ok(vec!["a", "b", "c"])]);
    }
}
