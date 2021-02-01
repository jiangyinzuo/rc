use crate::analyser::sym_resolver::SymbolResolver;
use crate::analyser::tests::get_ast_file;
use crate::ast::visit::Visit;
use crate::rcc::RccError;

#[test]
fn ident_not_found_test() {
    let mut sym_resolver = SymbolResolver::new();
    let mut ast_file = get_ast_file(
        r#"
        fn main() {
            fn foo() {}
            a = 2;
        }
    "#,
    )
    .unwrap();
    assert_eq!(1, ast_file.scope.types.len());
    assert_eq!(
        Err(RccError("identifier `a` not found".into())),
        sym_resolver.visit_file(&mut ast_file)
    );
}

#[test]
fn let_stmt_add_ident_test() {
    let mut sym_resolver = SymbolResolver::new();
    let mut ast_file = get_ast_file(
        r#"
        fn main() {
            let mut a = 3;
            fn foo() {}
            a = 2;
        }
    "#,
    )
    .unwrap();
    assert_eq!(1, ast_file.scope.types.len());
    assert_eq!(Ok(()), sym_resolver.visit_file(&mut ast_file));
}
