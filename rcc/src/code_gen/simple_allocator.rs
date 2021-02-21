use crate::code_gen::Allocator;
use crate::ir::cfg::CFG;

pub struct SimpleAllocator<'cfg> {
    cfg: &'cfg CFG,
    addr_size: u32,
}

impl<'cfg> SimpleAllocator<'cfg> {
    pub(crate) fn new(cfg: &CFG, addr_size: u32) -> SimpleAllocator {
        debug_assert!(addr_size == 32 || addr_size == 64);
        SimpleAllocator { cfg, addr_size }
    }
}

impl<'cfg> Allocator for SimpleAllocator<'cfg> {
    fn get_frame_size(&self) -> u32 {
        // ra
        let mut frame_size = self.addr_size / 8;
        // locals
        for (_id, ir_type) in self.cfg.local_infos.values() {
            frame_size += ir_type.byte_size(self.addr_size);
        }
        if frame_size % 8 == 0 {
            frame_size
        } else {
            (frame_size / 8 + 1) * 8
        }
    }
}
