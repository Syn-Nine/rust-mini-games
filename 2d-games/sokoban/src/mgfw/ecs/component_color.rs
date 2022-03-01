use super::*;
use crate::mgfw::log;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

pub struct ColorComponentManager {
    data: *mut Color,
    // WARNING: Anything below this line is not in cache!
}

#[allow(dead_code)]
impl ColorComponentManager {
    pub fn new(mgr: &mut CacheManager) -> ColorComponentManager {
        log(format!("Constructing ColorComponentManager"));
        let sz_bytes = std::mem::size_of::<Color>() * ENTITY_SZ;
        let data = mgr.allocate(sz_bytes) as *mut Color;

        // default init colors to opaque white
        for i in 0..ENTITY_SZ {
            let p = unsafe { &mut *(data.offset(i as isize)) };
            p.r = 1.0;
            p.g = 1.0;
            p.b = 1.0;
            p.a = 1.0;
        }

        ColorComponentManager { data }
    }

    pub fn clear(&mut self) {
        for i in 0..ENTITY_SZ {
            self.set_color_rgba(i, 1.0, 1.0, 1.0, 1.0);
        }
    }

    pub fn set_color(&self, idx: usize, color: Color) {
        self.set_color_rgba(idx, color.r, color.g, color.b, color.a);
    }

    pub fn set_color_rgba(&self, idx: usize, r: f32, g: f32, b: f32, a: f32) {
        let clr = self.get_data_ref_mut(idx);
        clr.r = r;
        clr.g = g;
        clr.b = b;
        clr.a = a;
    }

    pub fn get_color(&self, idx: usize) -> Color {
        self.get_data_ref(idx).clone()
    }

    pub fn set_alpha(&self, idx: usize, alpha: f32) {
        let clr = self.get_data_ref_mut(idx);
        clr.a = alpha;
    }

    pub fn get_alpha(&self, idx: usize) -> f32 {
        self.get_data_ref(idx).a
    }

    fn get_data_ref_mut(&self, idx: usize) -> &mut Color {
        assert!(idx < ENTITY_SZ);
        unsafe { &mut *(self.data.offset(idx as isize)) }
    }

    fn get_data_ref(&self, idx: usize) -> &Color {
        assert!(idx < ENTITY_SZ);
        unsafe { &*(self.data.offset(idx as isize)) }
    }
}
