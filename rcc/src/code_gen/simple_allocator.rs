use crate::ir::cfg::CFG;
use crate::ir::IRType;

struct SimpleAllocator<'cfg> {
    cfg: &'cfg CFG,
    addr_size: u32,
}

impl<'cfg> SimpleAllocator<'cfg> {
    pub fn new(cfg: &CFG, addr_size: u32) -> SimpleAllocator {
        debug_assert!(addr_size == 32 || addr_size == 64);
        SimpleAllocator { cfg, addr_size }
    }

    pub fn get_frame_size(&self) -> u32 {
        let mut frame_size = 0u32;
        for (_key, v) in &self.cfg.local_ids {
            match v.1 {
                IRType::I8 | IRType::U8 | IRType::Char | IRType::Bool => {
                    frame_size += 1;
                }
                IRType::I16 | IRType::U16 => {
                    frame_size += 2;
                }
                IRType::I32 | IRType::U32 | IRType::F32 => {
                    frame_size += 4;
                }
                IRType::I64 | IRType::U64 | IRType::F64 => {
                    frame_size += 4;
                }
                IRType::I128 | IRType::U128 => {
                    frame_size += 128;
                }
                IRType::Isize | IRType::Usize | IRType::Addr => {
                    frame_size += self.addr_size;
                }
                IRType::Unit | IRType::Never => {
                    frame_size += 0;
                }
            }
        }
        frame_size
    }
}
