use crate::analyser::sym_resolver::SymbolResolver;
use crate::analyser::tests::get_ast_file;
use crate::ast::visit::Visit;
use crate::rcc::RccError;

fn file_validate(inputs: &[&str], expecteds: &[Result<(), RccError>]) {
    assert_eq!(inputs.len(), expecteds.len());
    for (input, expected) in inputs.iter().zip(expecteds) {
        let mut sym_resolver = SymbolResolver::new();
        let mut ast_file = get_ast_file(input).expect("invalid ast file");
        let actual = sym_resolver.visit_file(&mut ast_file);
        assert_eq!(expected, &actual);
    }
}

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

#[test]
fn str_test() {
    let mut sym_resolver = SymbolResolver::new();
    let mut ast_file = get_ast_file(
        r#"
        fn main() {
            let mut a = "hello";

            fn foo(a: &str) {
            }

            a = "world";
            let b = foo("apple");
            a = "hello";
        }
    "#,
    )
    .unwrap();
    assert_eq!(1, ast_file.scope.types.len());
    assert_eq!(Ok(()), sym_resolver.visit_file(&mut ast_file));
    assert_eq!(3, sym_resolver.str_constants.len());
}

#[test]
fn fn_param_test() {
    let mut sym_resolver = SymbolResolver::new();
    let mut ast_file = get_ast_file(
        r#"
        fn add(a: i32, b: i32) {
           a + b
        }

        fn main() {
            add(1, 2);
        }
    "#,
    )
    .unwrap();
    assert_eq!(2, ast_file.scope.types.len());
    assert_eq!(Ok(()), sym_resolver.visit_file(&mut ast_file));
}

#[test]
fn assign_expr_test() {
    file_validate(
        &[
            r#"
        fn main() {
            let mut a = 32;
            a = 64i64;
        }
    "#,
            r#"fn sub(a: i32, b: i64) -> i64 {
        a - b
    }"#,
        ],
        &[Ok(()), Err("invalid operand for `-`".into())],
    );
}

#[test]
fn loop_test() {
    file_validate(
        &[
            r#"
        fn foo() {
            loop {
            }
        }
    "#,
            r#"
        fn foo() {
            let mut a = loop {
                break 32;
            };
            a = 30i64;
        }
    "#,
            r#"
        fn foo() {
            let mut a = loop { break };
            a = 64;
        }
    "#,
        ],
        &[Ok(()), Ok(()), Err("invalid type in assign expr".into())],
    );
}
