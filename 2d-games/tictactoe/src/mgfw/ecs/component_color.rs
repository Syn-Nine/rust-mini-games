use super::*;
use crate::mgfw::log;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

struct Color16 {
    pub r: i16,
    pub g: i16,
    pub b: i16,
    pub a: i16,
}

pub struct ColorComponentManager {
    data: *mut Color16,
    // WARNING: Anything below this line is not in cache!
}

const COLOR_SCALE: i16 = 255;
const COLOR_SCALE_F: f32 = 1.0 / COLOR_SCALE as f32;

#[allow(dead_code)]
impl ColorComponentManager {
    pub fn new(mgr: &mut CacheManager) -> ColorComponentManager {
        log(format!("Constructing ColorComponentManager"));
        let sz_bytes = std::mem::size_of::<Color16>() * ENTITY_SZ;
        let data = mgr.allocate(sz_bytes) as *mut Color16;

        // default init colors to opaque white
        for i in 0..ENTITY_SZ {
            let p = unsafe { &mut *(data.offset(i as isize)) };
            p.r = COLOR_SCALE;
            p.g = COLOR_SCALE;
            p.b = COLOR_SCALE;
            p.a = COLOR_SCALE;
        }

        ColorComponentManager { data }
    }

    pub fn set_color(&self, idx: usize, color: Color) {
        self.set_color_rgba(idx, color.r, color.g, color.b, color.a);
    }

    pub fn set_color_rgba(&self, idx: usize, r: f32, g: f32, b: f32, a: f32) {
        let clr = self.get_data_ref_mut(idx);
        clr.r = (r / COLOR_SCALE_F) as i16;
        clr.g = (g / COLOR_SCALE_F) as i16;
        clr.b = (b / COLOR_SCALE_F) as i16;
        clr.a = (a / COLOR_SCALE_F) as i16;
    }

    pub fn get_color(&self, idx: usize) -> Color {
        let clr = self.get_data_ref(idx);
        convert(clr)
    }

    pub fn set_alpha(&self, idx: usize, alpha: f32) {
        let clr = self.get_data_ref_mut(idx);
        clr.a = (alpha / COLOR_SCALE_F) as i16;
    }

    pub fn get_alpha(&self, idx: usize) -> f32 {
        self.get_data_ref(idx).a as f32 * COLOR_SCALE_F
    }

    fn get_data_ref_mut(&self, idx: usize) -> &mut Color16 {
        assert!(idx < ENTITY_SZ);
        unsafe { &mut *(self.data.offset(idx as isize)) }
    }

    fn get_data_ref(&self, idx: usize) -> &Color16 {
        assert!(idx < ENTITY_SZ);
        unsafe { &*(self.data.offset(idx as isize)) }
    }
}

fn convert(color: &Color16) -> Color {
    Color {
        r: color.r as f32 * COLOR_SCALE_F,
        g: color.g as f32 * COLOR_SCALE_F,
        b: color.b as f32 * COLOR_SCALE_F,
        a: color.a as f32 * COLOR_SCALE_F,
    }
}
