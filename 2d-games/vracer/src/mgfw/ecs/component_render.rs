use super::*;
use crate::mgfw::log;

pub const RENDER_TYPE_INVALID: u8 = 0;
pub const RENDER_TYPE_TEXT: u8 = 1;
pub const RENDER_TYPE_LINE_BUFFER: u8 = 2;
pub const RENDER_TYPE_TRIANGLE_BUFFER: u8 = 3;
pub const RENDER_TYPE_BILLBOARD: u8 = 4;
pub const RENDER_TYPE_TILEMAP: u8 = 5;

struct RenderComponentManagerData {
    render_type: u8,
}

pub struct RenderComponentManager {
    data: *mut RenderComponentManagerData,
    // WARNING: Anything below this line is not in cache!
}

#[allow(dead_code)]
impl RenderComponentManager {
    pub fn new(mgr: &mut CacheManager) -> RenderComponentManager {
        log(format!("Constructing RenderComponentManager"));
        // allocate system memory in cache
        let sz_bytes = std::mem::size_of::<RenderComponentManagerData>() * ENTITY_SZ;
        let data = mgr.allocate(sz_bytes) as *mut RenderComponentManagerData;

        RenderComponentManager { data }
    }

    pub fn clear(&mut self) {
        for i in 0..ENTITY_SZ {
            self.set_type(i, RENDER_TYPE_INVALID);
        }
    }

    pub fn set_type(&mut self, idx: usize, render_type: u8) {
        self.get_data_ref_mut(idx).render_type = render_type;
    }

    pub fn get_type(&self, idx: usize) -> u8 {
        self.get_data_ref(idx).render_type
    }

    pub fn has_type(&self, idx: usize, render_type: u8) -> bool {
        self.get_data_ref(idx).render_type == render_type
    }

    fn get_data_ref_mut(&self, idx: usize) -> &mut RenderComponentManagerData {
        assert!(idx < ENTITY_SZ);
        unsafe { &mut *(self.data.offset(idx as isize)) }
    }

    fn get_data_ref(&self, idx: usize) -> &RenderComponentManagerData {
        assert!(idx < ENTITY_SZ);
        unsafe { &*(self.data.offset(idx as isize)) }
    }
}
