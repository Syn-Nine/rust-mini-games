use super::*;
use crate::mgfw::log;

pub const PROJECTION_MODE_ORTHO: u8 = 0;
pub const PROJECTION_MODE_PERSPECTIVE: u8 = 1;

#[derive(Debug, Copy, Clone)]
pub struct Projection {
    pub mode: u8,
}

pub struct ProjectionComponentManager {
    data: *mut Projection,
    // WARNING: Anything below this line is not in cache!
}

#[allow(dead_code)]
impl ProjectionComponentManager {
    pub fn new(mgr: &mut CacheManager) -> ProjectionComponentManager {
        log(format!("Constructing ProjectionComponentManager"));
        let sz_bytes = std::mem::size_of::<Projection>() * ENTITY_SZ;
        ProjectionComponentManager {
            data: mgr.allocate(sz_bytes) as *mut Projection,
        }
    }

    pub fn clear(&mut self) {
        for i in 0..ENTITY_SZ {
            self.set_projection(i, PROJECTION_MODE_ORTHO);
        }
    }

    pub fn set_projection(&self, idx: usize, mode: u8) {
        let d = self.get_data_ref_mut(idx);
        d.mode = mode;
    }

    pub fn get_projection(&self, idx: usize) -> u8 {
        let d = self.get_data_ref(idx);
        d.mode
    }

    pub fn get_data_ref_mut(&self, idx: usize) -> &mut Projection {
        assert!(idx < ENTITY_SZ);
        unsafe { &mut *(self.data.offset(idx as isize)) }
    }

    pub fn get_data_ref(&self, idx: usize) -> &Projection {
        assert!(idx < ENTITY_SZ);
        unsafe { &*(self.data.offset(idx as isize)) }
    }
}
