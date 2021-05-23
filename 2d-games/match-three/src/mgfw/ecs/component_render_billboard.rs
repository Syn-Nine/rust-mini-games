use super::*;
use crate::mgfw::log;
use std::collections::HashMap;

struct BillboardRenderComponentManagerData {
    texture: u16,
    constructed: bool,
    reconstruct_needed: bool,
    load_image_needed: bool,
}

pub struct BillboardRenderComponentManager {
    cache_data: *mut BillboardRenderComponentManagerData,
    // WARNING: Anything below this line is not in cache!
    texture_files: std::boxed::Box<HashMap<usize, String>>,
    texture_handles: std::boxed::Box<HashMap<String, u32>>,
}

#[allow(dead_code)]
impl BillboardRenderComponentManager {
    pub fn new(mgr: &mut CacheManager) -> BillboardRenderComponentManager {
        log(format!("Constructing BillboardRenderComponentManager"));

        let fdata: HashMap<usize, String> = HashMap::new();
        let hdata: HashMap<String, u32> = HashMap::new();

        // allocate system memory in cache
        let sz_bytes = std::mem::size_of::<BillboardRenderComponentManagerData>() * ENTITY_SZ;
        let cache_data = mgr.allocate(sz_bytes) as *mut BillboardRenderComponentManagerData;

        BillboardRenderComponentManager {
            texture_files: Box::new(fdata),
            texture_handles: Box::new(hdata),
            cache_data,
        }
    }

    pub fn set_image(&mut self, idx: usize, image: String) {
        let cache_data = self.get_data_ref_mut(idx);
        cache_data.load_image_needed = true;
        cache_data.reconstruct_needed = true;
        self.texture_files.insert(idx, image);
    }

    pub fn is_constructed(&self, idx: usize) -> bool {
        self.get_data_ref(idx).constructed
    }

    pub fn reconstruct(&self, idx: usize) -> bool {
        self.get_data_ref(idx).reconstruct_needed
    }

    pub fn construct(&mut self, idx: usize, gl: &Gl, vao: u32, vbo: u32) {
        if self.get_data_ref(idx).load_image_needed {
            let filename = self.texture_files.get(&idx).unwrap().to_string();
            let handle: u32 = match self.texture_handles.contains_key(&filename) {
                true => *self.texture_handles.get(&filename).unwrap(),
                false => {
                    let h = gl.load_texture(&filename);
                    self.texture_handles.insert(filename, h);
                    h
                }
            };

            self.get_data_ref_mut(idx).texture = handle as u16;
            self.get_data_ref_mut(idx).load_image_needed = false;
        }

        let mut vertex_data: Vec<f32> = Vec::new();

        let mut pnts: Vec<Position> = Vec::new();
        let mut uvs: Vec<Position> = Vec::new();

        pnts.push(Position { x: -0.5, y: -0.5 });
        pnts.push(Position { x: -0.5, y: 0.5 });
        pnts.push(Position { x: 0.5, y: 0.5 });

        pnts.push(Position { x: -0.5, y: -0.5 });
        pnts.push(Position { x: 0.5, y: 0.5 });
        pnts.push(Position { x: 0.5, y: -0.5 });

        uvs.push(Position { x: 0.0, y: 0.0 });
        uvs.push(Position { x: 0.0, y: 1.0 });
        uvs.push(Position { x: 1.0, y: 1.0 });

        uvs.push(Position { x: 0.0, y: 0.0 });
        uvs.push(Position { x: 1.0, y: 1.0 });
        uvs.push(Position { x: 1.0, y: 0.0 });

        for i in 0..pnts.len() {
            vertex_data.push(pnts[i].x);
            vertex_data.push(pnts[i].y);
            vertex_data.push(uvs[i].x);
            vertex_data.push(uvs[i].y);
        }

        let data_ptr = vertex_data.as_ptr() as *const _;
        gl.buffer_billboard_data(vao, vbo, data_ptr);

        let cache_data = self.get_data_ref_mut(idx);
        cache_data.reconstruct_needed = false;
        cache_data.constructed = true;
    }

    pub fn get_tex_handle(&self, idx: usize) -> u16 {
        self.get_data_ref(idx).texture
    }

    fn get_data_ref_mut(&self, idx: usize) -> &mut BillboardRenderComponentManagerData {
        assert!(idx < ENTITY_SZ);
        unsafe { &mut *(self.cache_data.offset(idx as isize)) }
    }

    fn get_data_ref(&self, idx: usize) -> &BillboardRenderComponentManagerData {
        assert!(idx < ENTITY_SZ);
        unsafe { &*(self.cache_data.offset(idx as isize)) }
    }
}
