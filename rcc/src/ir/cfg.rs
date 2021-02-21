use crate::ir::linear_ir::LinearIR;
use crate::ir::{Func, IRInst, IRType};
use std::collections::{BTreeSet, HashMap, LinkedList};

/// Control FLow Graph's immediate representation
pub struct CFGIR {
    pub cfgs: Vec<CFG>,

    /// label, value
    pub ro_local_strs: HashMap<String, String>,
}

impl CFGIR {
    pub fn new(linear_ir: LinearIR) -> CFGIR {
        let cfgs: Vec<CFG> = linear_ir.funcs.into_iter().map(|f| CFG::new(f)).collect();
        CFGIR {
            cfgs,
            ro_local_strs: linear_ir.ro_local_strs,
        }
    }
}

/// Control Flow Graph
pub struct CFG {
    pub basic_blocks: Vec<BasicBlock>,
    pub local_ids: HashMap<String, (usize, IRType)>,
    pub func_name: String,
    pub func_is_global: bool,
}

/// number of successors less equal than 2 (the next leader or goto label)
#[derive(Debug)]
pub struct BasicBlock {
    /// start from 0
    pub id: usize,
    pub predecessors: Vec<usize>,
    pub instructions: LinkedList<IRInst>,
}

impl CFG {
    /// Instructions like `(n) if cond goto n+1` will be deleted in this pass.
    pub fn new(mut func: Func) -> CFG {
        let leaders = get_leaders(&func);
        let local_ids = get_local_ids(&func);

        // generate basic blocks and label map
        let mut label_map = HashMap::new();
        let mut leader = 1usize;

        let mut basic_blocks: Vec<BasicBlock> = leaders
            .iter()
            .enumerate()
            .map(|(i, next_leader)| {
                label_map.insert(leader, i);
                let mut inst_count = next_leader - leader;
                leader = *next_leader;
                let mut bb = LinkedList::new();
                while inst_count > 0 {
                    let lead_inst = func.insts.pop_front().unwrap();
                    match lead_inst {
                        // delete instructions like `(n) if cond goto n+1`
                        IRInst::Jump { label }
                        | IRInst::JumpIf { label, .. }
                        | IRInst::JumpIfNot { label, .. }
                        | IRInst::JumpIfCond { label, .. } => {
                            if i + 2 != label {
                                bb.push_back(lead_inst);
                            }
                        }
                        _ => {
                            bb.push_back(lead_inst);
                        }
                    }
                    inst_count -= 1;
                }
                BasicBlock::new(i, bb)
            })
            .collect();

        // change goto labels to bb id
        let mut unreachable_bb = vec![];
        let last_bb_id = basic_blocks.len() - 1;
        for i in 0..=last_bb_id {
            if let Some(bs) = match basic_blocks
                .get_mut(i)
                .unwrap()
                .instructions
                .back_mut()
                .unwrap()
            {
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

        CFG {
            basic_blocks,
            local_ids,
            func_name: func.name,
            func_is_global: func.is_global,
        }
    }

    pub fn succ_of(&self, bb_id: usize) -> Vec<usize> {
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
}

fn get_leaders(func: &Func) -> BTreeSet<usize> {
    macro_rules! insert_leaders {
        ($leaders:ident, $label:ident, $next_id:expr) => {
            $leaders.insert(*$label);
            $leaders.insert($next_id);
        };
    }

    let mut leaders = BTreeSet::new();

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
            _ => {}
        }
    }
    leaders.remove(&1usize);
    leaders.insert(func.insts.len() + 1);
    leaders
}

fn get_local_ids(func: &Func) -> HashMap<String, (usize, IRType)> {
    let mut local_ids = HashMap::new();
    let mut next_id: usize = 0;
    for inst in func.insts.iter() {
        match inst {
            IRInst::BinOp { dest, .. }
            | IRInst::LoadData { dest, .. }
            | IRInst::LoadAddr { dest, .. } => {
                if !local_ids.contains_key(&dest.label) {
                    local_ids.insert(dest.label.clone(), (next_id, dest.ir_type));
                    next_id += 1;
                }
            }
            _ => {}
        }
    }
    local_ids
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
