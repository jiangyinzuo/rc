use crate::ir::cfg::CFG;

struct SimpleAllocator<'cfg> {
    cfg: &'cfg CFG
}

impl<'cfg> SimpleAllocator<'cfg> {
    pub fn new(cfg: &CFG) -> SimpleAllocator {
        SimpleAllocator {
            cfg
        }
    }

    pub fn get_frame_size(&self) -> u32 {
        todo!()
    }
}