use super::*;
use crate::mgfw::log;

struct LineBuffer {
    // WARNING: Anything below this line is not in cache!
    pnts: Vec<Position>,
    clrs: Vec<Color>,
}

struct LineRenderComponentManagerData {
    num_lines: u16,
    constructed: bool,
    reconstruct_needed: bool,
}

pub struct LineRenderComponentManager {
    cache_data: *mut LineRenderComponentManagerData,
    // WARNING: Anything below this line is not in cache!
    data: std::boxed::Box<Vec<LineBuffer>>,
}

#[allow(dead_code)]
impl LineRenderComponentManager {
    pub fn new(mgr: &mut CacheManager) -> LineRenderComponentManager {
        log(format!("Constructing LineRenderComponentManager"));

        let mut data: Vec<LineBuffer> = Vec::new();
        for _i in 0..ENTITY_SZ {
            data.push(LineBuffer {
                pnts: Vec::new(),
                clrs: Vec::new(),
            });
        }

        // allocate system memory in cache
        let sz_bytes = std::mem::size_of::<LineRenderComponentManagerData>() * ENTITY_SZ;
        let cache_data = mgr.allocate(sz_bytes) as *mut LineRenderComponentManagerData;

        LineRenderComponentManager {
            data: Box::new(data),
            cache_data,
        }
    }

    pub fn clear(&mut self) {
        
    }

    pub fn set_line_buffer(&mut self, idx: usize, pnts: &Vec<Position>, clrs: &Vec<Color>) {
        self.get_data_ref_mut(idx).reconstruct_needed = true;
        self.data[idx].pnts = pnts.clone();
        self.data[idx].clrs = clrs.clone();
    }

    pub fn is_constructed(&self, idx: usize) -> bool {
        self.get_data_ref(idx).constructed
    }

    pub fn reconstruct(&self, idx: usize) -> bool {
        self.get_data_ref(idx).reconstruct_needed
    }

    pub fn construct(&self, idx: usize, gl: &Gl, vao: u32, vbo: u32) {
        let cache_data = self.get_data_ref_mut(idx);

        let pnts = &self.data[idx].pnts;
        let clrs = &self.data[idx].clrs;

        let num_lines = pnts.len() / 2;

        let mut vertex_data: Vec<f32> = Vec::new();

        for i in 0..pnts.len() {
            vertex_data.push(pnts[i].x);
            vertex_data.push(pnts[i].y);
            vertex_data.push(clrs[i].r);
            vertex_data.push(clrs[i].g);
            vertex_data.push(clrs[i].b);
            vertex_data.push(clrs[i].a);
        }

        let data_ptr = vertex_data.as_ptr() as *const _;
        gl.buffer_line_data(vao, vbo, num_lines, data_ptr);

        cache_data.reconstruct_needed = false;
        cache_data.constructed = true;
        cache_data.num_lines = num_lines as u16;
    }

    pub fn get_num_lines(&self, idx: usize) -> usize {
        self.get_data_ref(idx).num_lines as usize
    }

    fn get_data_ref_mut(&self, idx: usize) -> &mut LineRenderComponentManagerData {
        assert!(idx < ENTITY_SZ);
        unsafe { &mut *(self.cache_data.offset(idx as isize)) }
    }

    fn get_data_ref(&self, idx: usize) -> &LineRenderComponentManagerData {
        assert!(idx < ENTITY_SZ);
        unsafe { &*(self.cache_data.offset(idx as isize)) }
    }
}
