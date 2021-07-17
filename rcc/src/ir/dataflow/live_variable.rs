use crate::ir::cfg::{BasicBlock, CFG, BasicBlockId};
use crate::ir::{IRInst, Operand};
use bit_vector::BitVector;
use std::collections::VecDeque;
use crate::ir::dataflow::AnalysisDomain;

pub struct LiveVariableAnalysis<'cfg> {
    cfg: &'cfg CFG,
    pub in_states: Vec<BitVector>,
    pub out_states: Vec<BitVector>,
    in_changed: bool,
}

impl<'cfg> LiveVariableAnalysis<'cfg> {
    pub fn new(cfg: &'cfg CFG) -> LiveVariableAnalysis<'cfg> {
        LiveVariableAnalysis {
            cfg,
            in_states: vec![Self::init_value(cfg); cfg.basic_blocks.len()],
            out_states: vec![Self::init_value(cfg); cfg.basic_blocks.len()],
            in_changed: true,
        }
    }

    pub fn apply(&mut self) {

        // variables for breadth-first search
        let mut visited = BitVector::new(self.cfg.basic_blocks.len());
        let mut queue = VecDeque::<usize>::new();

        while self.in_changed {

            // initialize
            visited.set_all_false();
            self.in_changed = false;
            queue.clear();
            queue.push_back(self.cfg.basic_blocks.len() - 1);

            // iterate all the basic block in CFG with breadth-first search
            while !queue.is_empty() {
                let bb_id = queue.pop_front().unwrap();
                let bb = self.cfg.basic_blocks.get(bb_id).unwrap();

                visited.set(bb_id, true);

                self.out_states[bb_id] = self.join_succ(bb);
                for ir_inst in bb.instructions.iter() {
                    self.gen_kill(bb_id, ir_inst);
                }

                for p in bb.predecessors.iter() {
                    if !visited.get(*p).unwrap() {
                        queue.push_back(*p);
                    }
                }
            }
        }
    }

    fn boundary(cfg: &CFG) -> BitVector {
        BitVector::bottom_value(cfg)
    }

    fn init_value(cfg: &CFG) -> BitVector {
        BitVector::bottom_value(cfg)
    }

    fn join_succ(&self, basic_block: &BasicBlock) -> BitVector {
        let bid = basic_block.id;

        let succs = self.cfg.successors_of(bid);
        let res = succs
            .iter()
            .map(|succ_bb_id| self.in_states.get(*succ_bb_id).unwrap())
            .fold(BitVector::bottom_value(self.cfg), |mut acc, x| {
                acc.set_bitor(x);
                acc
            });
        res
    }

    fn gen_kill(&mut self, bb_id: BasicBlockId, inst: &IRInst) {
        macro_rules! gen {
            ($cxt:tt, $dest:tt, $in_state:ident) => {
                let dest_id = $cxt.cfg.local_variables.get(&$dest.label).unwrap().0;
                $in_state.set(dest_id, true);
            };
        }

        macro_rules! kill {
            ($cxt:tt, $src:tt, $in_state:ident) => {
                if let Operand::Place(p) = $src {
                    let src_id = $cxt.cfg.local_variables.get(&p.label).unwrap().0;
                    $in_state.set(src_id, false);
                }
            };
        }

        let out_state = self.out_states.get_mut(bb_id).unwrap();
        let in_state = self.in_states.get_mut(bb_id).unwrap();
        in_state.clone_from(out_state);
        match inst {
            IRInst::LoadAddr { .. } => {
                todo!()
            }
            IRInst::LoadData { dest, src } => {
                gen!(self, dest, in_state);
                kill!(self, src, in_state);
            }
            IRInst::BinOp {
                dest, src1, src2, ..
            } => {
                gen!(self, dest, in_state);
                kill!(self, src1, in_state);
                kill!(self, src2, in_state);
            }
            IRInst::JumpIf { cond, .. } | IRInst::JumpIfNot { cond, .. } => {
                kill!(self, cond, in_state);
            }
            IRInst::JumpIfCond { src1, src2, .. } => {
                kill!(self, src1, in_state);
                kill!(self, src2, in_state);
            }
            IRInst::Call {args, ..} => {
                for arg in args {
                    kill!(self, arg, in_state);
                }
            }
            _ => {}
        }
    }
}
