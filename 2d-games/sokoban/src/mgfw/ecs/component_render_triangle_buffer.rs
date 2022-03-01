use super::*;
use crate::mgfw::log;

struct TriangleBuffer {
    // WARNING: Anything below this line is not in cache!
    pnts: Vec<Position>,
    clrs: Vec<Color>,
}

struct TriangleRenderComponentManagerData {
    num_triangles: u16,
    constructed: bool,
    reconstruct_needed: bool,
}

pub struct TriangleRenderComponentManager {
    cache_data: *mut TriangleRenderComponentManagerData,
    // WARNING: Anything below this line is not in cache!
    data: std::boxed::Box<Vec<TriangleBuffer>>,
}

#[allow(dead_code)]
impl TriangleRenderComponentManager {
    pub fn new(mgr: &mut CacheManager) -> TriangleRenderComponentManager {
        log(format!("Constructing TriangleRenderComponentManager"));

        let mut data: Vec<TriangleBuffer> = Vec::new();
        for _i in 0..ENTITY_SZ {
            data.push(TriangleBuffer {
                pnts: Vec::new(),
                clrs: Vec::new(),
            });
        }

        // allocate system memory in cache
        let sz_bytes = std::mem::size_of::<TriangleRenderComponentManagerData>() * ENTITY_SZ;
        let cache_data = mgr.allocate(sz_bytes) as *mut TriangleRenderComponentManagerData;

        TriangleRenderComponentManager {
            data: Box::new(data),
            cache_data,
        }
    }

    pub fn clear(&mut self) {
        
    }

    pub fn set_triangle_buffer(&mut self, idx: usize, pnts: &Vec<Position>, clrs: &Vec<Color>) {
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

        let num_triangles = pnts.len() / 3;

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
        gl.buffer_triangle_data(vao, vbo, num_triangles, data_ptr);

        cache_data.reconstruct_needed = false;
        cache_data.constructed = true;
        cache_data.num_triangles = num_triangles as u16;
    }

    pub fn get_num_triangles(&self, idx: usize) -> usize {
        self.get_data_ref(idx).num_triangles as usize
    }

    fn get_data_ref_mut(&self, idx: usize) -> &mut TriangleRenderComponentManagerData {
        assert!(idx < ENTITY_SZ);
        unsafe { &mut *(self.cache_data.offset(idx as isize)) }
    }

    fn get_data_ref(&self, idx: usize) -> &TriangleRenderComponentManagerData {
        assert!(idx < ENTITY_SZ);
        unsafe { &*(self.cache_data.offset(idx as isize)) }
    }
}
