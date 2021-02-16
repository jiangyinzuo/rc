use crate::ir::{Func, IRInst};
use std::collections::{BTreeSet, HashMap, LinkedList};

/// Control Flow Graph
pub struct CFG {
    pub basic_blocks: Vec<BasicBlock>,
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
    pub fn new(mut func: Func) -> CFG {
        macro_rules! insert_leaders {
            ($leaders:ident, $label:ident, $next_id:expr) => {
                $leaders.insert(*$label);
                $leaders.insert($next_id);
            };
        }

        // get leaders
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
                    let inst = func.insts.pop_front().unwrap();
                    match inst {
                        // delete instructions like `(n) if cond goto n+1`
                        IRInst::Jump { label }
                        | IRInst::JumpIf { label, .. }
                        | IRInst::JumpIfNot { label, .. }
                        | IRInst::JumpIfCond { label, .. } => {
                            if i + 2 != label {
                                bb.push_back(inst);
                            }
                        }
                        _ => {
                            bb.push_back(inst);
                        }
                    }
                    inst_count -= 1;
                }
                BasicBlock::new(i, bb)
            })
            .collect();

        // change goto labels to bb id
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
                _ => None,
            } {
                for b in bs {
                    basic_blocks.get_mut(b).unwrap().predecessors.push(i);
                }
            }
        }

        //
        CFG { basic_blocks }
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

impl BasicBlock {
    pub fn new(id: usize, instructions: LinkedList<IRInst>) -> BasicBlock {
        BasicBlock {
            id,
            predecessors: vec![],
            instructions,
        }
    }
}
