use crate::analyser::tests::get_ast_file;

#[test]
fn scope_test() {
    let ast_file = get_ast_file(r#"
        fn main() {
            fn foo() {}
        }
    "#).unwrap();
    assert_eq!(1, ast_file.scope.types.len());
}