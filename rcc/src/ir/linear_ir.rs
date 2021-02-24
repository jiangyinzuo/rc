use crate::ast::item::{ItemFn, FnSignature};
use crate::ast::pattern::Pattern;
use crate::ast::Visibility;
use crate::ir::{IRInst, IRType, Operand, Place};
use crate::rcc::RccError;
use std::collections::{HashMap, VecDeque};

pub struct LinearIR {
    pub funcs: Vec<Func>,
    /// label, value
    pub ro_local_strs: HashMap<String, String>,
}

impl LinearIR {
    pub fn new() -> LinearIR {
        LinearIR {
            funcs: vec![],
            ro_local_strs: HashMap::new(),
        }
    }

    pub fn add_ro_local_str(&mut self, s: String) -> Operand {
        let label = format!(".LC{}", self.ro_local_strs.len());
        self.ro_local_strs.insert(label.clone(), s);
        Operand::Place(Place::lit_const(label, IRType::Char))
    }

    pub fn add_func(&mut self, item_fn: &ItemFn) -> Result<(), RccError> {
        let fn_name = item_fn.name.clone();
        let is_global = item_fn.vis() == Visibility::Pub;

        let scope = &item_fn.fn_block.scope;
        let scope_id = scope.scope_id;
        debug_assert_ne!(0, scope_id);

        let mut fn_args = Vec::new();
        for param in item_fn.fn_params.params.iter() {
            fn_args.push(match &param.pattern {
                Pattern::Identifier(i) => {
                    let (var_info, _) = scope.find_variable(i.ident()).unwrap();
                    (i.ident().to_string(), IRType::from_var_info(var_info)?)
                }
            });
        }

        self.funcs
            .push(Func::new(fn_name, is_global, fn_args, scope_id));
        Ok(())
    }

    pub fn cur_func_mut(&mut self) -> &mut Func {
        self.funcs.last_mut().unwrap()
    }

    pub fn add_instructions(&mut self, ir_inst: IRInst) {
        self.cur_func_mut().insts.push_back(ir_inst);
    }

    /// Start from 1
    pub fn next_inst_id(&mut self) -> usize {
        self.cur_func_mut().insts.len() + 1
    }

    pub fn get_inst_by_id(&mut self, id: usize) -> &mut IRInst {
        self.cur_func_mut().insts.get_mut(id - 1).unwrap()
    }
}

pub struct Func {
    pub name: String,
    pub insts: VecDeque<IRInst>,
    pub is_global: bool,
    pub fn_args: Vec<(String, IRType)>,
    pub block_scope_id: u64,
}

impl Func {
    pub fn new(
        name: String,
        is_global: bool,
        fn_args: Vec<(String, IRType)>,
        block_scope_id: u64,
    ) -> Func {
        Func {
            name,
            insts: VecDeque::new(),
            is_global,
            fn_args,
            block_scope_id,
        }
    }
}
