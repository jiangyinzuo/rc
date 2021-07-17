use crate::analyser::sym_resolver::VarKind;
use crate::ir::cfg::{BasicBlock, BasicBlockId, CFG};
use crate::ir::{IRInst, Operand, Place};
use crate::rcc::RccError;
use bit_vector::BitVector;
use std::collections::{HashMap, VecDeque};

pub struct ReachingDefinitionsAnalysis<'cfg> {
    cfg: &'cfg CFG,
    definitions: HashMap<&'cfg String, Vec<(usize, BasicBlockId, isize)>>,
    num_definitions: usize,

    ins: Vec<BitVector>,
    outs: Vec<BitVector>,
    out_changed: bool,
}

impl<'cfg> ReachingDefinitionsAnalysis<'cfg> {
    pub fn new(cfg: &'cfg CFG) -> ReachingDefinitionsAnalysis {
        fn add_definitions<'a>(
            dest: &'a Place,
            definitions: &mut HashMap<&'a String, Vec<(usize, BasicBlockId, isize)>>,
            next_definition_id: &mut usize,
            bb_id: BasicBlockId,
            inst_id: isize,
        ) {
            match dest.kind {
                VarKind::Local | VarKind::LocalMut => {
                    match definitions.get_mut(&dest.label) {
                        Some(ids) => ids.push((*next_definition_id, bb_id, inst_id)),
                        None => {
                            definitions
                                .insert(&dest.label, vec![(*next_definition_id, bb_id, inst_id)]);
                        }
                    }

                    *next_definition_id += 1;
                }
                _ => {}
            }
        }

        // (definition_id, bb_id, inst_id)
        let mut definitions: HashMap<&String, Vec<(usize, usize, isize)>> = HashMap::new();
        let mut next_definition_id = cfg.fn_args.len();

        // function arguments' definitions
        for definition_id in 0..next_definition_id {
            definitions.insert(
                &cfg.fn_args_local_var[definition_id],
                vec![(definition_id, 0, -1)],
            );
        }

        for (bb_id, bb) in cfg.basic_blocks.iter().enumerate() {
            for (inst_id, inst) in bb.instructions.iter().enumerate() {
                match inst {
                    IRInst::BinOp { op: _op, dest, .. } => add_definitions(
                        dest,
                        &mut definitions,
                        &mut next_definition_id,
                        bb_id,
                        inst_id as isize,
                    ),
                    IRInst::LoadData { dest, .. } => add_definitions(
                        dest,
                        &mut definitions,
                        &mut next_definition_id,
                        bb_id,
                        inst_id as isize,
                    ),
                    _ => {}
                }
            }
        }

        let mut ins = vec![BitVector::new(next_definition_id); cfg.basic_blocks.len()];
        for i in 0..cfg.fn_args.len() {
            ins[0].set(i, true);
        }

        ReachingDefinitionsAnalysis {
            cfg,
            definitions,
            num_definitions: next_definition_id,
            ins,
            outs: vec![BitVector::new(next_definition_id); cfg.basic_blocks.len()],
            out_changed: true,
        }
    }

    pub fn apply(&mut self) -> Result<(), RccError> {
        while self.out_changed {
            self.out_changed = false;

            // variables for breadth-first search
            let mut visited = BitVector::new(self.cfg.basic_blocks.len());
            let mut queue = VecDeque::<BasicBlockId>::new();

            // iterate from start
            queue.push_back(0);

            // iterate all the basic block in CFG with breadth-first search
            while !queue.is_empty() {
                let bb_id: BasicBlockId = queue.pop_front().unwrap();
                let bb = self.cfg.basic_blocks.get(bb_id).unwrap();

                visited.set(bb_id, true);

                if bb_id == 0 {
                    // merge fn args
                    let out = self.join(&bb.predecessors);
                    self.ins[bb_id].set_bitor(&out);
                } else {
                    self.ins[bb_id] = self.join(&bb.predecessors);
                }

                self.out_changed = self.block_apply(bb_id, bb)?;

                for s_bb_id in self.cfg.successors_of(bb_id) {
                    if !visited.get(s_bb_id).unwrap() {
                        queue.push_back(s_bb_id);
                    }
                }
            }
        }
        Ok(())
    }

    fn join(&mut self, predecessors: &[BasicBlockId]) -> BitVector {
        predecessors
            .iter()
            .map(|bb_id: &BasicBlockId| self.outs.get(*bb_id).unwrap())
            .fold(BitVector::new(self.num_definitions), |mut acc, x| {
                acc.set_bitor(x);
                acc
            })
    }

    fn block_apply(&mut self, bb_id: usize, bb: &BasicBlock) -> Result<bool, RccError> {
        let in_ = &self.ins[bb_id];
        // let out = &mut self.outs[bb_id];
        debug_assert_eq!(in_.len(), self.num_definitions, "num definitions error");
        debug_assert_eq!(
            self.outs[bb_id].len(),
            self.num_definitions,
            "num definitions error"
        );

        let old_out = self.outs[bb_id].clone();
        self.outs[bb_id].clone_from(in_);
        for (inst_id, inst) in bb.instructions.iter().enumerate() {
            match inst {
                IRInst::BinOp {
                    op: _op,
                    dest,
                    src1,
                    src2,
                } => {
                    self.valid(bb_id, src1)?;
                    self.valid(bb_id, src2)?;
                    self.gen_kill(dest, bb_id, inst_id as isize);
                }
                IRInst::LoadData { dest, src } => {
                    self.valid(bb_id, src)?;
                    self.gen_kill(dest, bb_id, inst_id as isize);
                }
                _ => {}
            }
        }
        Ok(old_out.ne(&self.outs[bb_id]))
    }

    fn valid(&mut self, in_bb_id: BasicBlockId, operand: &Operand) -> Result<(), RccError> {
        let in_ = &mut self.outs[in_bb_id];
        if let Operand::Place(place) = operand {
            let mut has_definitions = false;

            // None: may use global definitions
            if let Some(defs) = self.definitions.get(&place.label) {
                for (definition_id, _, _) in defs {
                    has_definitions |= in_.get(*definition_id).unwrap();
                }
            }

            return if has_definitions {
                Ok(())
            } else {
                Err(RccError::from(format!(
                    "`{}` may not have definition",
                    place.label
                )))
            };
        }
        Ok(())
    }

    fn gen_kill(&mut self, dest: &Place, bb_id: BasicBlockId, inst_id: isize) {
        let out = &mut self.outs[bb_id];
        if let Some(definitions) = self.definitions.get(&dest.label) {
            for &(definition_id, bb_id2, inst_id2) in definitions {
                out.set(definition_id, bb_id == bb_id2 && inst_id == inst_id2);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ir::dataflow::reaching_definitions::ReachingDefinitionsAnalysis;
    use crate::ir::dataflow::tests::get_cfg;

    #[test]
    fn new_test() {
        let cfg = get_cfg(
            r#"
        fn foo(ddd: i32) {
            let mut a = 3;
            a = 3 + 2;
            let b = 4;
            let c = a + b;
        }
    "#,
        )
        .unwrap();
        let mut analysis = ReachingDefinitionsAnalysis::new(&cfg);
        analysis.apply().unwrap();
    }

    #[test]
    fn fn_arg_test() {
        let cfg = get_cfg(
            r#"
            fn add10(x: i32) -> i32 {
                x + 10
            }
        "#,
        )
        .unwrap();
        let mut analysis = ReachingDefinitionsAnalysis::new(&cfg);
        assert_eq!(analysis.apply(), Ok(()));
    }

    #[test]
    fn if_test() {
        let cfg = get_cfg(
            r#"
            fn add10(x: i32) -> i32 {
                let mut b;
                if x == 3 {
                    b = 4;
                } else {
                    b = 9;
                }
                b + 333i32
            }
        "#,
        )
            .unwrap();
        let mut analysis = ReachingDefinitionsAnalysis::new(&cfg);
        assert_eq!(analysis.apply(), Ok(()));
    }

    #[test]
    fn undefined_test1() {
        let cfg = get_cfg(
            r#"
                fn bar(a: i32, b: i128) {
                    let mut c;
                    let d = c + 3;
                    c = 3;
                }
            "#,
        )
        .unwrap();
        let mut analysis = ReachingDefinitionsAnalysis::new(&cfg);
        assert_eq!(
            analysis.apply(),
            Err("`c_2` may not have definition".into())
        );
    }

    #[test]
    fn undefined_test2() {
        let cfg = get_cfg(
            r#"
        fn foo(ddd: i32) {
            let mut a = 3;
            a = 3 + 2;
            let mut b;
            if a == 5 {
                b = 4i32;
            } else {
                //b = 5i32;
            }
            let c = a + b;
            b += 1;
        }
    "#,
        )
        .unwrap();
        let mut analysis = ReachingDefinitionsAnalysis::new(&cfg);
        assert_eq!(
            analysis.apply(),
            Err("`b_2` may not have definition".into())
        );
    }

    #[test]
    fn undefined_test3() {
        let cfg = get_cfg(
            r#"
fn bar(b: i32) {
    let mut a: i32;
    if  b == 3 {
        a = 3;
    }
    let b = a + 4i32;
}
        "#,
        )
        .unwrap();
        let mut analysis = ReachingDefinitionsAnalysis::new(&cfg);
        assert_eq!(
            analysis.apply(),
            Err("`a_2` may not have definition".into())
        );
    }
}
