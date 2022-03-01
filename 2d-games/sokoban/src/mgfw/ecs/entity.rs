use super::*;
use crate::mgfw::log;

pub const ENTITY_SZ: usize = 1024; // limit to 256

#[derive(Copy, Clone)]
pub struct EntityIdSpan {
    pub first: usize,
    pub last: usize,
}

struct Entity {
    components: u32,
}

pub struct EntityRegistry {
    data: *mut Entity,
    // WARNING: Anything below this line is not in cache!
    cursor: usize, // current insertion cursor
    span: EntityIdSpan,
}

#[allow(dead_code)]
impl EntityRegistry {
    pub fn new(mgr: &mut CacheManager) -> EntityRegistry {
        log(format!("Constructing EntityRegistry"));
        let sz_bytes = std::mem::size_of::<Entity>() * ENTITY_SZ;
        EntityRegistry {
            data: mgr.allocate(sz_bytes) as *mut Entity,
            cursor: 0,
            span: EntityIdSpan {
                first: ENTITY_SZ - 1,
                last: 0,
            },
        }
    }

    pub fn clear(&mut self) {
        self.cursor = 0;
        self.span = EntityIdSpan { first: ENTITY_SZ - 1, last: 0 };
        for i in 0..ENTITY_SZ {
            self.get_data_ref_mut(i).components = 0;
        }
    }

    pub fn add(&mut self) -> usize {
        // find first non-active entity
        for _i in 0..ENTITY_SZ {
            if !self.has_component(self.cursor, COMPONENT_ACTIVE) {
                break;
            }
            self.cursor = (self.cursor + 1) % ENTITY_SZ; // wrap around
        }

        if self.has_component(self.cursor, COMPONENT_ACTIVE) {
            log(format!(
                "WARNING: EntityRegistry: Ran out of available entity slots!"
            ));
            assert!(false); // make sure we actually found an open slot
        }
        self.add_component(self.cursor, COMPONENT_ACTIVE); // set to used
        self.cursor
    }

    pub fn add_component(&mut self, idx: usize, component: u32) {
        let entity = self.get_data_ref_mut(idx);
        entity.components |= component;

        // lazy span update
        if idx < self.span.first {
            self.span.first = idx;
        }
        if idx > self.span.last {
            self.span.last = idx;
        }
    }

    pub fn has_component(&self, idx: usize, component: u32) -> bool {
        let entity = self.get_data_ref(idx);
        (entity.components & component) == component
    }

    pub fn set_active(&mut self, idx: usize, val: bool) {
        self.overwrite_component(idx, COMPONENT_VISIBLE, val);
    }

    pub fn is_active(&self, idx: usize) -> bool {
        return self.has_component(idx, COMPONENT_ACTIVE);
    }

    pub fn set_visibility(&mut self, idx: usize, val: bool) {
        self.overwrite_component(idx, COMPONENT_VISIBLE, val);
    }

    pub fn is_visible(&self, idx: usize) -> bool {
        return self.has_component(idx, COMPONENT_VISIBLE);
    }

    pub fn clear_component(&mut self, idx: usize, component: u32) {
        let entity = self.get_data_ref_mut(idx);
        entity.components &= !component;

        // check if span can be updated
        if COMPONENT_ACTIVE == component && (idx == self.span.first || idx == self.span.last) {
            self.update_span();
        }
    }

    pub fn overwrite_component(&mut self, idx: usize, component: u32, val: bool) {
        self.clear_component(idx, component);
        if val {
            self.add_component(idx, component);
        }
    }

    pub fn get_id_span(&self) -> EntityIdSpan {
        self.span
    }

    fn update_span(&mut self) {
        self.span = EntityIdSpan {
            first: ENTITY_SZ - 1,
            last: 0,
        };
        for idx in 0..ENTITY_SZ {
            if self.has_component(idx, COMPONENT_ACTIVE) {
                if idx < self.span.first {
                    self.span.first = idx;
                }
                if idx > self.span.last {
                    self.span.last = idx;
                }
            }
        }
    }

    fn get_data_ref_mut(&self, idx: usize) -> &mut Entity {
        assert!(idx < ENTITY_SZ);
        unsafe { &mut *(self.data.offset(idx as isize)) }
    }

    fn get_data_ref(&self, idx: usize) -> &Entity {
        assert!(idx < ENTITY_SZ);
        unsafe { &*(self.data.offset(idx as isize)) }
    }
}
