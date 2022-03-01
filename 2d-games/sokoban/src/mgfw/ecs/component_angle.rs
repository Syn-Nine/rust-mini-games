use super::*;
use crate::mgfw::log;

#[derive(Debug, Copy, Clone)]
pub struct Angle {
    pub x: f32,
}

pub struct AngleComponentManager {
    data: *mut Angle,
    // WARNING: Anything below this line is not in cache!
}

#[allow(dead_code)]
impl AngleComponentManager {
    pub fn new(mgr: &mut CacheManager) -> AngleComponentManager {
        log(format!("Constructing AngleComponentManager"));
        let sz_bytes = std::mem::size_of::<Angle>() * ENTITY_SZ;
        AngleComponentManager {
            data: mgr.allocate(sz_bytes) as *mut Angle,
        }
    }

    pub fn clear(&mut self) {
        for i in 0..ENTITY_SZ {
            self.set_angle(i, 0.0);
        }
    }

    pub fn set_angle(&self, idx: usize, x: f32) {
        let pos = self.get_data_ref_mut(idx);
        pos.x = x;
    }

    pub fn get_angle(&self, idx: usize) -> f32 {
        let pos = self.get_data_ref(idx);
        pos.x
    }

    pub fn get_data_ref_mut(&self, idx: usize) -> &mut Angle {
        assert!(idx < ENTITY_SZ);
        unsafe { &mut *(self.data.offset(idx as isize)) }
    }

    pub fn get_data_ref(&self, idx: usize) -> &Angle {
        assert!(idx < ENTITY_SZ);
        unsafe { &*(self.data.offset(idx as isize)) }
    }
}
