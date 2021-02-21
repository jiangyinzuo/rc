use std::collections::HashMap;

use crate::ast::item::ItemFn;
use crate::ast::pattern::Pattern;
use crate::ast::Visibility;
use crate::ir::{Func, IRInst, IRType, Operand, Place};

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

    pub fn add_func(&mut self, item_fn: &ItemFn) {
        let fn_name = item_fn.name.clone();
        let is_global = item_fn.vis() == Visibility::Pub;

        let fn_args: Vec<String> = item_fn
            .fn_params
            .params
            .iter()
            .map(|param| match &param.pattern {
                Pattern::Identifier(i) => i.ident().to_string(),
            })
            .collect();
        self.funcs.push(Func::new(fn_name, is_global, fn_args));
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
