use super::*;
use crate::mgfw::log;

#[derive(Debug, Copy, Clone)]
pub struct Scale {
    pub x: f32,
    pub y: f32,
}

pub struct ScaleComponentManager {
    data: *mut Scale,
    // WARNING: Anything below this line is not in cache!
}

#[allow(dead_code)]
impl ScaleComponentManager {
    pub fn new(mgr: &mut CacheManager) -> ScaleComponentManager {
        log(format!("Constructing ScaleComponentManager"));
        let sz_bytes = std::mem::size_of::<Scale>() * ENTITY_SZ;
        ScaleComponentManager {
            data: mgr.allocate(sz_bytes) as *mut Scale,
        }
    }

    pub fn clear(&mut self) {
        for i in 0..ENTITY_SZ {
            self.set_scale(i, 0.0, 0.0);
        }
    }

    pub fn set_scale(&self, idx: usize, x: f32, y: f32) {
        let scale = self.get_data_ref_mut(idx);
        scale.x = x;
        scale.y = y;
    }

    pub fn get_scale(&self, idx: usize) -> Scale {
        let scale = self.get_data_ref(idx);
        scale.clone()
    }

    pub fn get_data_ref_mut(&self, idx: usize) -> &mut Scale {
        assert!(idx < ENTITY_SZ);
        unsafe { &mut *(self.data.offset(idx as isize)) }
    }

    pub fn get_data_ref(&self, idx: usize) -> &Scale {
        assert!(idx < ENTITY_SZ);
        unsafe { &*(self.data.offset(idx as isize)) }
    }
}
