extern crate lexer;
extern crate strenum;

#[cfg(test)]
mod lexer_tests {
    use super::lexer::tokenize;
    use super::strenum::AnswerFn;

    #[derive(AnswerFn)]
    enum Struct {
        Foo,
        Bar,
        #[disabled]
        Barzzz
    }

    #[test]
    fn test1() {
        let tokens = tokenize("h  h ");
        assert_eq!(2, tokens.len());
        Struct::describe();
    }
}
