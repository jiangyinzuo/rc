#[cfg(test)]
mod ir_tests{
    use crate::ast::file::File;
    use crate::ir::{Data, IRGen, IRGenContext, Quad};
    use crate::parser::{Parse, ParseContext};
    use lexer::Lexer;

    #[test]
    fn test() {
        let input = "fn main() -> i32 {20}";
        let mut lexer = Lexer::new(input);
        let token_stream = lexer.tokenize();
        let mut parse_context = ParseContext::new(token_stream);
        let file = File::parse(&mut parse_context).unwrap();
        let mut cxt = IRGenContext::new();
        file.generate(&mut cxt);
        let basic_blocks = cxt.basic_blocks.pop().unwrap();
        assert_eq!(basic_blocks.name, "main");
        assert_eq!(
            basic_blocks.quads,
            vec![Quad::ret(Data {
                _type: "i32".to_string(),
                value: "20".to_string()
            })]
        );
    }
}
