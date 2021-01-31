use crate::analyser::scope::Scope;
use crate::analyser::sym_resolver::{VarInfo, VarKind};
use crate::ast::types::{TypeAnnotation, TypeLit};

#[test]
fn scope_test() {
    let mut scope = Scope::new();
    let var_info = VarInfo::new(3, VarKind::Local, TypeAnnotation::Lit(TypeLit::U64));
    scope.add_variable("a", VarInfo::new(1, VarKind::Local, TypeAnnotation::Bool));
    scope.add_variable(
        "a",
        VarInfo::new(3, VarKind::Local, TypeAnnotation::Lit(TypeLit::U64)),
    );
    scope.add_variable(
        "a",
        VarInfo::new(8, VarKind::LocalMut, TypeAnnotation::Bool),
    );
    scope.cur_stmt_id = 4;
    assert_eq!(
        &var_info,
        scope.find_variable("a").unwrap()
    );
}
