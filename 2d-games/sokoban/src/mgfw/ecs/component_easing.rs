use super::*;
use crate::mgfw::log;

#[derive(Debug, Copy, Clone)]
pub struct Ease {
    pub end: f32,
    pub dxdt: f32,
    pub entity: u8,
    pub variable: u8,
}

#[derive(Copy, Clone)]
pub struct EasingIdSpan {
    pub first: usize,
    pub last: usize,
}

const EASING_SZ: usize = 96;
const EASING_VAR_INACTIVE: u8 = 0;
pub const EASING_VAR_ALPHA: u8 = 1;

pub struct EasingComponentManager {
    data: *mut Ease,
    // WARNING: Anything below this line is not in cache!
    cursor: usize, // current insertion cursor
    span: EasingIdSpan,
}

#[allow(dead_code)]
impl EasingComponentManager {
    pub fn new(mgr: &mut CacheManager) -> EasingComponentManager {
        log(format!("Constructing EasingComponentManager"));
        let sz_bytes = std::mem::size_of::<Ease>() * EASING_SZ;
        EasingComponentManager {
            data: mgr.allocate(sz_bytes) as *mut Ease,
            cursor: 0,
            span: EasingIdSpan {
                first: EASING_SZ - 1,
                last: 0,
            },
        }
    }

    pub fn clear(&mut self) {
        // to do
    }

    fn slot_open(&self, idx: usize) -> bool {
        EASING_VAR_INACTIVE == self.get_data_ref(idx).variable
    }

    fn find_existing(&self, entity: usize, variable: u8) -> usize {
        for idx in self.span.first..=self.span.last {
            let data = self.get_data_ref(idx);
            if data.entity == entity as u8 && data.variable == variable {
                return idx;
            }
        }
        EASING_SZ
    }

    fn add_ease(&mut self, entity: usize, start: f32, end: f32, dt: f32, variable: u8) {
        let dxdt = (end - start) / dt;

        let existing = self.find_existing(entity, variable);
        if EASING_SZ > existing {
            self.cursor = existing;
            self.get_data_ref_mut(self.cursor).variable = EASING_VAR_INACTIVE;
        } else {
            // find first non-active ease
            for _i in 0..EASING_SZ {
                if self.slot_open(self.cursor) {
                    break;
                }
                self.cursor = (self.cursor + 1) % EASING_SZ; // wrap around
            }
        }

        if !self.slot_open(self.cursor) {
            log(format!(
                "WARNING: EasingComponentManager: Ran out of available easing slots!"
            ));
            assert!(false); // make sure we actually found an open slot
        }

        let data = self.get_data_ref_mut(self.cursor);
        data.end = end;
        data.entity = entity as u8;
        data.variable = variable;
        data.dxdt = dxdt;

        // lazy span update
        if self.cursor < self.span.first {
            self.span.first = self.cursor;
        }
        if self.cursor > self.span.last {
            self.span.last = self.cursor;
        }
    }

    pub fn set_alpha_ease(&mut self, idx: usize, start: f32, end: f32, dt: f32) {
        self.add_ease(idx, start, end, dt, EASING_VAR_ALPHA);
    }

    pub fn deactivate(&mut self, idx: usize) {
        self.get_data_ref_mut(idx).variable = EASING_VAR_INACTIVE;

        // check if span can be updated
        if idx == self.span.first || idx == self.span.last {
            self.update_span();
        }
    }

    fn update_span(&mut self) {
        self.span = EasingIdSpan {
            first: EASING_SZ - 1,
            last: 0,
        };
        for idx in 0..EASING_SZ {
            if EASING_VAR_INACTIVE != self.get_data_ref(idx).variable {
                if idx < self.span.first {
                    self.span.first = idx;
                }
                if idx > self.span.last {
                    self.span.last = idx;
                }
            }
        }
    }

    pub fn get_id_span(&self) -> EasingIdSpan {
        self.span
    }

    fn get_data_ref_mut(&self, idx: usize) -> &mut Ease {
        assert!(idx < EASING_SZ);
        unsafe { &mut *(self.data.offset(idx as isize)) }
    }

    pub fn get_data_ref(&self, idx: usize) -> &Ease {
        assert!(idx < EASING_SZ);
        unsafe { &*(self.data.offset(idx as isize)) }
    }
}
