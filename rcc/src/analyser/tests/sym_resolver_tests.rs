use crate::analyser::sym_resolver::SymbolResolver;
use crate::analyser::tests::get_ast_file;
use crate::ast::visit::Visit;

#[test]
fn scope_test() {
    let mut sym_resolver = SymbolResolver::new();
    let mut ast_file = get_ast_file(r#"
        fn main() {
            fn foo() {}
        }
    "#).unwrap();
    assert_eq!(1, ast_file.scope.types.len());
    sym_resolver.visit_file(&mut ast_file);
}