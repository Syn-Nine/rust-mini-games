use super::*;
use crate::mgfw::log;

#[derive(Debug, Copy, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub struct PositionComponentManager {
    data: *mut Position,
    // WARNING: Anything below this line is not in cache!
}

#[allow(dead_code)]
impl PositionComponentManager {
    pub fn new(mgr: &mut CacheManager) -> PositionComponentManager {
        log(format!("Constructing PositionComponentManager"));
        let sz_bytes = std::mem::size_of::<Position>() * ENTITY_SZ;
        PositionComponentManager {
            data: mgr.allocate(sz_bytes) as *mut Position,
        }
    }

    pub fn clear(&mut self) {
        for i in 0..ENTITY_SZ {
            self.set_position(i, 0.0, 0.0);
        }
    }

    pub fn set_position(&self, idx: usize, x: f32, y: f32) {
        let pos = self.get_data_ref_mut(idx);
        pos.x = x;
        pos.y = y;
    }

    pub fn get_position(&self, idx: usize) -> Position {
        let pos = self.get_data_ref(idx);
        pos.clone()
    }

    pub fn get_data_ref_mut(&self, idx: usize) -> &mut Position {
        assert!(idx < ENTITY_SZ);
        unsafe { &mut *(self.data.offset(idx as isize)) }
    }

    pub fn get_data_ref(&self, idx: usize) -> &Position {
        assert!(idx < ENTITY_SZ);
        unsafe { &*(self.data.offset(idx as isize)) }
    }
}
