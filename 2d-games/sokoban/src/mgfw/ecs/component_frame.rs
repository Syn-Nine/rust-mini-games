use super::*;
use crate::mgfw::log;

#[derive(Debug, Copy, Clone)]
pub struct Frame {
    pub frame: u16,
}

pub struct FrameComponentManager {
    data: *mut Frame,
    // WARNING: Anything below this line is not in cache!
}

#[allow(dead_code)]
impl FrameComponentManager {
    pub fn new(mgr: &mut CacheManager) -> FrameComponentManager {
        log(format!("Constructing FrameComponentManager"));
        let sz_bytes = std::mem::size_of::<Frame>() * ENTITY_SZ;
        FrameComponentManager {
            data: mgr.allocate(sz_bytes) as *mut Frame,
        }
    }

    pub fn clear(&mut self) {
        for i in 0..ENTITY_SZ {
            self.set_frame(i, 0);
        }
    }

    pub fn set_frame(&self, idx: usize, frame: u16) {
        let d = self.get_data_ref_mut(idx);
        d.frame = frame;
    }

    pub fn get_frame(&self, idx: usize) -> u16 {
        let d = self.get_data_ref(idx);
        d.frame
    }

    pub fn get_data_ref_mut(&self, idx: usize) -> &mut Frame {
        assert!(idx < ENTITY_SZ);
        unsafe { &mut *(self.data.offset(idx as isize)) }
    }

    pub fn get_data_ref(&self, idx: usize) -> &Frame {
        assert!(idx < ENTITY_SZ);
        unsafe { &*(self.data.offset(idx as isize)) }
    }
}
