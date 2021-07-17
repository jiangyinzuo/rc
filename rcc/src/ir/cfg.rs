use crate::ir::linear_ir::{Func, LinearIR};
use crate::ir::var_name::local_var;
use crate::ir::{IRInst, IRType};
use std::collections::{BTreeSet, HashMap, LinkedList};
use crate::rcc::RccError;
use crate::ir::dataflow::reaching_definitions::ReachingDefinitionsAnalysis;

/// Control FLow Graph's immediate representation
pub struct CFGIR {
    pub cfgs: Vec<CFG>,

    /// read only local strings, <label, value>
    pub ro_local_strs: HashMap<String, String>,
}

impl CFGIR {
    pub fn new(linear_ir: LinearIR) -> CFGIR {
        let cfgs: Vec<CFG> = linear_ir.funcs.into_iter().map(CFG::new).collect();
        CFGIR {
            cfgs,
            ro_local_strs: linear_ir.ro_local_strs,
        }
    }

    pub fn reaching_definitions_analysis(&self) -> Result<(), RccError>{
        for cfg in &self.cfgs {
            let mut analysis =ReachingDefinitionsAnalysis::new(cfg);
            analysis.apply()?;
        }
        Ok(())
    }
}

/// Control Flow Graph
pub struct CFG {
    pub basic_blocks: Vec<BasicBlock>,

    /// Information of local variables
    /// <variable name, (variable id, variable's IRType)>
    pub local_variables: HashMap<String, (usize, IRType)>,

    /// function information
    pub func_name: String,
    pub func_scope_id: u64,
    pub func_is_global: bool,
    pub fn_args: Vec<(String, IRType)>,
    pub fn_args_local_var: Vec<String>,
    pub is_leaf: bool,
}

pub type BasicBlockId = usize;

/// number of successors less equal than 2 (the next leader or goto label)
#[derive(Debug)]
pub struct BasicBlock {
    /// start from 0
    pub id: BasicBlockId,
    /// Predecessors of this `BasicBlock` in CFG
    pub predecessors: Vec<BasicBlockId>,
    pub instructions: LinkedList<IRInst>,
}

impl CFG {
    /// Instructions like `(n) if cond goto n+1` will be deleted in this pass.
    pub fn new(mut func: Func) -> CFG {
        let (leaders, is_leaf) = get_leaders_and_is_leaf(&func);
        let local_variables = get_local_variables(&func);

        // generate basic blocks and label map
        let mut label_map = HashMap::new();
        let mut leader = 1usize;

        let mut inst_id = 1;
        let mut basic_blocks: Vec<BasicBlock> = leaders
            .iter()
            .enumerate()
            .map(|(i, next_leader)| {
                label_map.insert(leader, i);
                let mut inst_count = next_leader - leader;
                leader = *next_leader;
                let mut bb = LinkedList::new();
                while inst_count > 0 {
                    let inst = func.insts.pop_front().unwrap();
                    match inst {
                        // delete instructions like `(n) if cond goto n+1`
                        IRInst::Jump { label }
                        | IRInst::JumpIf { label, .. }
                        | IRInst::JumpIfNot { label, .. }
                        | IRInst::JumpIfCond { label, .. } => {
                            if inst_id + 1 != label {
                                bb.push_back(inst);
                            }
                        }
                        _ => {
                            bb.push_back(inst);
                        }
                    }
                    inst_count -= 1;
                    inst_id += 1;
                }
                BasicBlock::new(i, bb)
            })
            .collect();

        // change goto labels to bb id
        let mut unreachable_bb = vec![];
        let last_bb_id = basic_blocks.len() - 1;
        for i in 0..=last_bb_id {
            let basic_block = basic_blocks.get_mut(i).unwrap();
            if let Some(inst) = basic_block.instructions.back_mut() {
                if let Some(bs) = match inst {
                    IRInst::Jump { label, .. } => {
                        *label = *label_map.get(label).unwrap();
                        Some(vec![*label])
                    }
                    IRInst::JumpIfNot { label, .. }
                    | IRInst::JumpIf { label, .. }
                    | IRInst::JumpIfCond { label, .. } => {
                        *label = *label_map.get(label).unwrap();
                        if i < last_bb_id {
                            Some(vec![*label, i + 1])
                        } else {
                            Some(vec![*label])
                        }
                    }
                    _ => {
                        if i < last_bb_id {
                            Some(vec![i + 1])
                        } else {
                            if i != 0 {
                                unreachable_bb.push(i);
                            }
                            None
                        }
                    }
                } {
                    for b in bs {
                        basic_blocks.get_mut(b).unwrap().predecessors.push(i);
                    }
                }
            }
        }

        let mut fn_args_local_var = Vec::with_capacity(func.fn_args.len());
        for (arg, _) in &func.fn_args {
            fn_args_local_var.push(local_var(arg, func.block_scope_id));
        }

        CFG {
            basic_blocks,
            local_variables,
            func_name: func.name,
            func_scope_id: func.block_scope_id,
            func_is_global: func.is_global,
            fn_args: func.fn_args,
            fn_args_local_var,
            is_leaf,
        }
    }

    /// Get all successors of BasicBlock with id `bb_id`
    pub fn successors_of(&self, bb_id: BasicBlockId) -> Vec<usize> {
        debug_assert!(bb_id < self.basic_blocks.len(), "bb_id out of range");

        match self
            .basic_blocks
            .get(bb_id)
            .unwrap()
            .instructions
            .back()
            .unwrap()
        {
            IRInst::Jump { label } => vec![*label],

            IRInst::JumpIf { label, .. }
            | IRInst::JumpIfNot { label, .. }
            | IRInst::JumpIfCond { label, .. } => {
                let mut succ = vec![*label];
                if bb_id < self.basic_blocks.len() - 1 {
                    succ.push(bb_id + 1);
                }
                succ
            }
            _ => vec![],
        }
    }

    pub fn get_name_of_fn_arg(&self, i: usize) -> Option<String> {
        let (raw_name, _) = self.fn_args.get(i)?;
        Some(local_var(raw_name, self.func_scope_id))
    }

    pub fn iter_inst(&self) -> CFGIterMut {
        CFGIterMut::new(self)
    }
}

fn get_leaders_and_is_leaf(func: &Func) -> (BTreeSet<usize>, bool) {
    macro_rules! insert_leaders {
        ($leaders:ident, $label:ident, $next_id:expr) => {
            $leaders.insert(*$label);
            $leaders.insert($next_id);
        };
    }

    let mut leaders = BTreeSet::new();
    let mut is_leaf = true;
    for (i, inst) in func.insts.iter().enumerate() {
        match inst {
            IRInst::Jump { label }
            | IRInst::JumpIf { label, .. }
            | IRInst::JumpIfNot { label, .. }
            | IRInst::JumpIfCond { label, .. } => {
                if i + 2 != *label {
                    insert_leaders!(leaders, label, i + 2);
                }
            }
            IRInst::Call { .. } => {
                is_leaf = false;
            }
            _ => {}
        }
    }
    leaders.remove(&1usize);
    leaders.insert(func.insts.len() + 1);
    (leaders, is_leaf)
}

fn get_local_variables(func: &Func) -> HashMap<String, (usize, IRType)> {
    let mut local_variables = HashMap::new();
    let mut next_id: usize = 0;
    for arg in &func.fn_args {
        let var_name = local_var(&arg.0, func.block_scope_id);
        local_variables.insert(var_name, (next_id, arg.1));
    }

    for inst in func.insts.iter() {
        match inst {
            IRInst::BinOp { dest, .. }
            | IRInst::LoadData { dest, .. }
            | IRInst::LoadAddr { dest, .. } => {
                if !local_variables.contains_key(&dest.label) {
                    local_variables.insert(dest.label.clone(), (next_id, dest.ir_type));
                    next_id += 1;
                }
            }
            _ => {}
        }
    }
    local_variables
}

impl BasicBlock {
    pub fn new(id: usize, instructions: LinkedList<IRInst>) -> BasicBlock {
        BasicBlock {
            id,
            predecessors: vec![],
            instructions,
        }
    }
}

pub struct CFGIterMut<'cfg> {
    bb_iter: std::slice::Iter<'cfg, BasicBlock>,
    ir_iter: Option<std::collections::linked_list::Iter<'cfg, IRInst>>,
}

impl<'cfg> CFGIterMut<'cfg> {
    pub fn new(cfg: &'cfg CFG) -> CFGIterMut<'cfg> {
        let iter = cfg.basic_blocks.iter();
        CFGIterMut {
            bb_iter: iter,
            ir_iter: None,
        }
    }

    #[inline]
    fn reset_bb(&mut self) -> Option<<Self as Iterator>::Item> {
        match self.bb_iter.next() {
            Some(b) => {
                self.ir_iter = Some(b.instructions.iter());
                self.next()
            }
            None => None,
        }
    }
}

impl<'cfg> Iterator for CFGIterMut<'cfg> {
    type Item = &'cfg IRInst;

    fn next(&mut self) -> Option<Self::Item> {
        match self.ir_iter.as_mut() {
            Some(ir_iter) => match ir_iter.next() {
                Some(item) => Some(item),
                None => self.reset_bb(),
            },
            None => self.reset_bb(),
        }
    }
}
