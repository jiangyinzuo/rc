use crate::analyser::sym_resolver::TypeInfo::*;
use crate::analyser::sym_resolver::{TypeInfo, VarInfo, VarKind};
use crate::ast::expr::BlockExpr;
use crate::ast::file::File;
use crate::ast::item::{Item, ItemFn, ItemStruct};
use crate::ast::types::TypeLitNum::*;
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::ptr::NonNull;
use std::rc::Rc;

lazy_static! {
    pub static ref BULITIN_SCOPE: Scope = {
        let mut s = Scope::new(0);
        s.types.insert("bool".into(), Bool);
        s.types.insert("char".into(), Char);
        s.types.insert("str".into(), Str);
        s.types.insert("f32".into(), LitNum(F32));
        s.types.insert("f64".into(), LitNum(F64));
        s.types.insert("i8".into(), LitNum(I8));
        s.types.insert("i16".into(), LitNum(I16));
        s.types.insert("i32".into(), LitNum(I32));
        s.types.insert("i64".into(), LitNum(I64));
        s.types.insert("i128".into(), LitNum(I128));
        s.types.insert("isize".into(), LitNum(Isize));
        s.types.insert("u8".into(), LitNum(U8));
        s.types.insert("u16".into(), LitNum(U16));
        s.types.insert("u32".into(), LitNum(U32));
        s.types.insert("u64".into(), LitNum(U64));
        s.types.insert("u128".into(), LitNum(U128));
        s.types.insert("usize".into(), LitNum(Usize));
        s
    };
}

pub struct Scope {
    father: Option<NonNull<Scope>>,
    pub(crate) types: HashMap<String, TypeInfo>,
    variables: HashMap<String, Vec<VarInfo>>,
    pub cur_stmt_id: u64,
    temp_count: u64,
    scope_id: u64,
}

unsafe impl std::marker::Sync for Scope {}

impl Scope {
    pub fn new(scope_id: u64) -> Scope {
        Scope {
            father: None,
            types: HashMap::new(),
            variables: HashMap::new(),
            cur_stmt_id: 0,
            temp_count: 0,
            scope_id,
        }
    }

    pub fn gen_temp_variable(&mut self, type_info: Rc<RefCell<TypeInfo>>) -> String {
        let kind = VarKind::Local;
        let ident = format!("${}_{}", self.temp_count, self.scope_id);
        self.temp_count += 1;
        self.add_variable(&ident, kind, type_info);
        ident
    }

    pub fn add_variable(&mut self, ident: &str, kind: VarKind, type_info: Rc<RefCell<TypeInfo>>) {
        let var_info = VarInfo::new(self.cur_stmt_id, kind, type_info);
        if let Some(v) = self.variables.get_mut(ident) {
            v.push(var_info);
        } else {
            self.variables.insert(ident.to_string(), vec![var_info]);
        }
    }

    pub fn find_variable(& self, ident: &str) -> Option<&VarInfo> {
        let mut cur_scope: *const Scope = self;
        loop {
            let s = unsafe { &*cur_scope };
            if let Some(v) = s.variables.get(ident) {
                let mut left = 0;
                let mut right = v.len();
                if right == 1 {
                    return Some(unsafe { v.get_unchecked(0) });
                }
                while left < right {
                    let mid = (left + right + 1) / 2;
                    let stmt_id = unsafe { (*v.get_unchecked(mid)).stmt_id() };
                    // Let stmt and variable using stmt is impossible to be the same.
                    debug_assert_ne!(stmt_id, self.cur_stmt_id);
                    if self.cur_stmt_id < stmt_id {
                        right = mid - 1;
                    } else {
                        left = mid;
                    }
                }
                return Some(unsafe { v.get_unchecked(left) });
            } else if let Some(f) = s.father {
                cur_scope = f.as_ptr();
            } else {
                return None;
            }
        }
    }

    pub fn find_def_except_fn(&self, ident: &str) -> TypeInfo {
        let mut cur_scope: *const Scope = self;
        loop {
            let s = unsafe { &*cur_scope };
            if let Some(ti) = s.types.get(ident) {
                match ti {
                    TypeInfo::Fn { .. } => {}
                    _ => return ti.clone(),
                }
            }
            if let Some(f) = s.father {
                cur_scope = f.as_ptr();
            } else {
                return Unknown;
            }
        }
    }

    pub fn find_fn(&self, ident: &str) -> TypeInfo {
        let mut cur_scope: *const Scope = self;
        loop {
            let s = unsafe { &*cur_scope };
            if let Some(ti) = s.types.get(ident) {
                if let TypeInfo::Fn { .. } = ti {
                    return ti.clone();
                }
            }
            if let Some(f) = s.father {
                cur_scope = f.as_ptr();
            } else {
                return Unknown;
            }
        }
    }

    pub fn add_typedef(&mut self, item: &Item) {
        match item {
            Item::Fn(item_fn) => self.add_type_fn(item_fn),
            Item::Struct(item_struct) => self.add_type_struct(item_struct),
            _ => todo!(),
        }
    }

    fn add_type_fn(&mut self, item_fn: &ItemFn) {
        let type_info = TypeInfo::from_item_fn(item_fn);
        self.types.insert(item_fn.name.clone(), type_info);
    }

    fn add_type_struct(&mut self, item_struct: &ItemStruct) {
        let type_info = TypeInfo::from_item_struct(item_struct);
        self.types.insert(item_struct.name().to_string(), type_info);
    }

    pub fn set_father(&mut self, father: *mut Scope) {
        self.father = Some(unsafe { NonNull::new_unchecked(father) });
    }

    pub fn set_father_as_builtin_scope(&mut self) {
        self.father = Some(NonNull::from(BULITIN_SCOPE.deref()));
    }

    pub fn scope_id(&self) -> u64 {
        self.scope_id
    }
}

pub struct ScopeStack {
    cur_scope: *mut Scope,
    file_scope: Option<NonNull<Scope>>,
    scope_stack: Vec<*mut Scope>,
}

impl ScopeStack {
    pub fn new() -> ScopeStack {
        ScopeStack {
            cur_scope: std::ptr::null_mut(),
            file_scope: None,
            scope_stack: vec![],
        }
    }

    pub fn enter_scope(&mut self, block_expr: &mut BlockExpr) {
        block_expr.scope.set_father(self.cur_scope);
        self.scope_stack.push(self.cur_scope);
        self.cur_scope = &mut block_expr.scope;
    }

    pub fn exit_scope(&mut self) {
        if let Some(s) = self.scope_stack.pop() {
            self.cur_scope = s;
            unsafe { &mut *self.cur_scope }.cur_stmt_id = 0;
        } else {
            debug_assert!(false, "scope_stack is empty!");
        }
    }

    pub fn cur_scope_is_global(&mut self) -> bool {
        if let Some(f) = &mut self.file_scope {
            self.cur_scope == f.as_ptr()
        } else {
            false
        }
    }

    pub fn cur_scope(&self) -> &Scope {
        unsafe { &*self.cur_scope }
    }

    pub fn cur_scope_mut(&mut self) -> &mut Scope {
        unsafe { &mut *self.cur_scope }
    }

    pub fn enter_file(&mut self, file: &mut File) {
        self.cur_scope = &mut file.scope;
        self.file_scope = Some(NonNull::new(&mut file.scope).unwrap());
    }
}
