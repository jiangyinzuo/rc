use crate::analyser::sym_resolver::SymbolResolver;
use crate::analyser::tests::get_ast_file;
use crate::rcc::RccError;

fn file_validate(inputs: &[&str], expecteds: &[Result<(), RccError>]) {
    assert_eq!(inputs.len(), expecteds.len());
    for (i, (input, expected)) in inputs.iter().zip(expecteds).enumerate() {
        let mut sym_resolver = SymbolResolver::new();
        let ast_file = get_ast_file(input);
        match ast_file {
            Ok(mut f) => {
                let actual = sym_resolver.visit_file(&mut f);
                assert_eq!(expected, &actual, "{}th test case", i);
            }
            Err(e) => {
                assert_eq!(expected, &Err(e), "{}th test case", i);
            }
        }

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
        Err("identifier `a` not found".into()),
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
fn type_annotation_test() {
    file_validate(
        &[
            r#"
        fn foo() {
            let a: char = 'c';
            let b: i64 = 34;
            let b: i128 = 33i128;
        }
    "#,
            r#"
        fn foo() {
            let a: i32 = 4i64;
        }
    "#,
            r#"
        fn fff() -> char {
            'a'
        }
    "#,
        ],
        &[
            Ok(()),
            Err("invalid type in let stmt: expected `LitNum(i32)`, found `LitNum(i64)`".into()),
            Ok(()),
        ],
    );
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
}

#[test]
fn fn_param_test() {
    let mut sym_resolver = SymbolResolver::new();
    let mut ast_file = get_ast_file(
        r#"
        fn add(a: i32, b: i32) -> i32 {
           let c: i32 = a + b;
           c
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
            a >>= 1usize;
            a += 128i64;
            let mut b = true;
            b ^= false;
            b |= true;
        }
    "#,
            r#"
        fn sub(a: i32, b: i64) -> i64 {
            a - b
        }
    "#,
            r#"
        fn foo() {
            let a = 3;
            a=4;
        }
    "#,
            r#"
        fn foo() {
            let mut a = 3;
            a>>=4;
            a^=3f32;
        }
    "#,
        ],
        &[
            Ok(()),
            Err("invalid operand type `LitNum(i32)` and `LitNum(i64)` for `-`".into()),
            Err("lhs is not mutable".into()),
            Err("invalid type `LitNum(#i)` for `^=`".into()),
        ],
    );
}

#[test]
fn bin_op_test() {
    file_validate(
        &[
            r#"
        fn rem3(x: i32) -> i32 {
            x % 3.3
        }
      
    "#,
            "fn rem2(x: u32) -> u32 {x%2}",
        ],
        &[Err(
            "invalid operand type `LitNum(i32)` and `LitNum(#f)` for `%`".into(),
        ), Ok(())],
    );
}

#[test]
fn block_test() {
    file_validate(
        &[r##"
        fn main() {
            let a = 3;
            if true {
                2
            } else {
                3
            }
            let b = 3;
        }
    "##],
        &[Err(
            "invalid type for expr stmt: expected `()`, found LitNum(#i)".into(),
        )],
    );
}

#[test]
fn loop_test() {
    file_validate(
        &[
            r#"
        fn foo() {
            loop {
            };
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
            r#"
        fn foo() {
            let a = true;
            while !a {
                break;
            }
        }
    "#,
            r#"
        fn bar() {
            let b = false;
            let c = true;
            let a = while b ^ c {
                break 3;
            };
        }
    "#,
            r#"
        fn bar() {
            let b = false;
            let c = true;
            let a = while b ^ c {
                break;
            };
        }
    "#,
        ],
        &[
            Ok(()),
            Ok(()),
            Err("invalid type `Unit` for `=`".into()),
            Ok(()),
            Err("only loop can return values".into()),
            Ok(()),
        ],
    );
}

#[test]
fn return_test() {
    file_validate(
        &[
            r#"
        fn add(a: i32, b: i32) -> i32 { a+b }
    "#,
            r#"fn add(a: i32, b: i32) -> i64 { a+b}"#,
            r#"fn add(a: i32, b: i32) -> i64 {return a+b;}"#,
            r#"fn add(a: i32, b: i32) -> i32 {return a+b;}"#,
            r#"fn never_return() -> ! {loop{}}"#,
            r#"fn never_return() -> ! {
            let a = loop{};
            a
           }"#,
            r#"fn main() -> i32 { let mut a = 7;
                 if a == 2 {a = 3;}
                 else {
                    a = 4;
                 }
            }"#,
            r#"fn main() -> i32 { let mut a = 7;
                 if a == 2 {3}
                 else {
                    a
                 }
            }"#,
        ],
        &[
            Ok(()),
            Err("invalid return type: excepted `LitNum(i64)`, found `LitNum(i32)`".into()),
            Err("invalid return type: excepted `LitNum(i64)`, found `LitNum(i32)`".into()),
            Ok(()),
            Ok(()),
            Ok(()),
            Err("invalid return type: excepted `LitNum(i32)`, found `Unit`".into()),
            Ok(()),
        ],
    );
}

#[test]
fn never_type_test() {
    file_validate(
        &[
            r#"fn f() -> i32 {loop {}}"#,
            r#"
        fn foo() -> i32 {
            let b: char = loop {
            };
            let c: u64 = loop {
                return loop {};
            };
            let a = loop {};
            a
        }
    "#,
            r#"
        fn foo() -> i64 {
            let b = !false;
            if b && true {
                if loop {} && false {}
                loop {
                    return 3;
                }
            } else {
                return loop {}
            }
        }
    "#,
            r#"fn foo() -> bool{ let a = loop{};a&true}"#,
            r#"
        fn add() {
            let a = loop {} + 1;
        }
    "#,
        ],
        &[
            Ok(()),
            Ok(()),
            Ok(()),
            Err("invalid operand type `Never` and `Bool` for `&`".into()),
            Err("invalid operand type `Never` and `LitNum(#i)` for `+`".into()),
        ],
    );
}

#[test]
fn control_flow_test() {
    file_validate(
        &[
            r#"
        fn guess(mid: i32) -> i32 {
            mid
        }
        fn foo123() -> i32 {
            let actual_value = 23 + 4 - 5 + 6 - 77 + 77;
            let mut left = 0;
            let mut right = 100;
            while left < right {
                let mid = (left + right) / 2;
                if guess(mid) < actual_value {
                    left = mid + 1;
                } else if guess(mid) > actual_value {
                    right = mid - 1;
                } else {
                    return actual_value;
                }
            }
            return -1;
        }
    "#,
            r#"fn fff() { if 2 {let a = 3;}}"#,
            r#"fn bbb() { while true {2}}"#,
        ],
        &[
            Ok(()),
            Err("invalid type of condition expr: expected `bool`, found: LitNum(#i)".into()),
            Err("invalid type in while block: expected Unit, found LitNum(#i)".into()),
        ],
    );
}

#[test]
fn call_test() {
    file_validate(
        &[
            r#"
        fn add(a: i32, b: i32) -> i32 {a+b}
        fn foo() {}
        fn main() {
            let a = foo();
            let b = add(1, 2);
            let c: i32 = b + 3i32;
        }
    "#,
            r#"fn foo(a: i32) {}
        fn main() {
            foo(3i64);
        }
    "#,
            r#"
                fn foo() {
                }
                fn add() {
                    foo(3);
                }
            "#,
            r#"
                fn main() {
                    let a = 1;
                    a();
                }
            "#,
        ],
        &[
            Ok(()),
            Err("invalid type for call expr: expected LitNum(i32), found LitNum(i64)".into()),
            Err("This function takes 0 parameters but 1 parameters was supplied".into()),
            Err("expr is not callable".into()),
        ],
    );
}

#[test]
fn local_mut_test() {
    file_validate(
        &[r#"fn add() {
        let a = 2;
        a = 3;
    }"#],
        &[Err("lhs is not mutable".into())],
    );
}

#[test]
fn external_block_test() {
    file_validate(&[r#"
extern "C" {
    fn putchar(i: i32);
}

fn main() {
    putchar(97);
}
    "#, r#"
extern "C" {
    fn putchar(i: i32) {
        let i = 3;
    }
}
    "#], &[Ok(()), Err("error in parsing: except ;".into())]);
}
