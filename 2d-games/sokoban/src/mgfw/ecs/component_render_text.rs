use super::*;
use crate::mgfw::log;

struct Text {
    // WARNING: Anything below this line is not in cache!
    text: String,
}

struct TextRenderComponentManagerData {
    width: u16,
    num_chars: u16,
    constructed: bool,
    reconstruct_needed: bool,
}

pub struct TextRenderComponentManager {
    cache_data: *mut TextRenderComponentManagerData,
    // WARNING: Anything below this line is not in cache!
    data: std::boxed::Box<Vec<Text>>,
    font: std::boxed::Box<fonts::retro_gaming::Font>,
}

#[allow(dead_code)]
impl TextRenderComponentManager {
    pub fn new(mgr: &mut CacheManager) -> TextRenderComponentManager {
        log(format!("Constructing TextRenderComponentManager"));

        let mut data: Vec<Text> = Vec::new();
        for _i in 0..ENTITY_SZ {
            data.push(Text {
                text: String::new(),
            });
        }

        // allocate system memory in cache
        let sz_bytes = std::mem::size_of::<TextRenderComponentManagerData>() * ENTITY_SZ;
        let cache_data = mgr.allocate(sz_bytes) as *mut TextRenderComponentManagerData;

        TextRenderComponentManager {
            data: Box::new(data),
            font: Box::new(fonts::retro_gaming::Font::new()),
            cache_data,
        }
    }

    pub fn clear(&mut self) {
        
    }

    pub fn set_text(&mut self, idx: usize, text: String) {
        self.get_data_ref_mut(idx).reconstruct_needed = true;
        self.data[idx].text = text;
    }

    pub fn is_constructed(&self, idx: usize) -> bool {
        self.get_data_ref(idx).constructed
    }

    pub fn reconstruct(&self, idx: usize) -> bool {
        self.get_data_ref(idx).reconstruct_needed
    }

    // potential cache miss
    fn recalc_width(&self, idx: usize) {
        let cache_data = self.get_data_ref_mut(idx);

        let bytes = self.data[idx].text.as_bytes();
        let mut basex: f32 = 0.0;

        for i in 0..bytes.len() {
            let idx = bytes[i] as u16;
            let data = self.font.data[&idx];
            let advance = data[6] as f32;
            basex += advance;
        }

        cache_data.width = basex as u16;
    }

    // probable cache miss
    pub fn construct(&self, idx: usize, gl: &Gl, vao: u32, vbo: u32) {
        let cache_data = self.get_data_ref_mut(idx);

        /*println!("{}, {}", self.font.scale_w, self.font.scale_h);
        println!("{:?}", self.font.page_files);
        println!("{:?}", self.font.data[&('t' as u16)]);
        println!("{:?}", self.data[idx].text);
        let bytes = self.data[idx].text.as_bytes();
        for i in 0..bytes.len() {
            let idx = bytes[i] as u16;
            println!("{:?} {:?}", bytes, self.font.data[&idx]);
        }*/

        let ww = 256.0;
        let hh = 64.0;
        let mut vertex_data: Vec<f32> = Vec::new();

        let bytes = self.data[idx].text.as_bytes();
        let num_chars = bytes.len();

        let mut basex: f32 = 0.0;

        for i in 0..num_chars {
            let idx = bytes[i] as u16;

            let data = self.font.data[&idx];
            let dx = data[0] as f32 / ww;
            let dy = data[1] as f32 / hh;
            let dw = data[2] as f32;
            let dh = data[3] as f32;
            let dwt = dw as f32 / ww;
            let dht = dh as f32 / hh;
            let dxoff = data[4] as f32;
            let dyoff = data[5] as f32;
            let advance = data[6] as f32;

            let p0 = [basex + 0.0 + dxoff, 0.0 + dyoff, dx, dy];
            let p1 = [basex + 0.0 + dxoff, dh + dyoff, dx, dy + dht];
            let p2 = [basex + dw + dxoff, dh + dyoff, dx + dwt, dy + dht];
            let p3 = [basex + dw + dxoff, 0.0 + dyoff, dx + dwt, dy];

            for i in 0..4 {
                vertex_data.push(p0[i]);
            }
            for i in 0..4 {
                vertex_data.push(p1[i]);
            }
            for i in 0..4 {
                vertex_data.push(p2[i]);
            }
            for i in 0..4 {
                vertex_data.push(p0[i]);
            }
            for i in 0..4 {
                vertex_data.push(p2[i]);
            }
            for i in 0..4 {
                vertex_data.push(p3[i]);
            }

            basex += advance;
        }

        let data_ptr = vertex_data.as_ptr() as *const _;
        gl.buffer_font_data(vao, vbo, num_chars, data_ptr);

        cache_data.reconstruct_needed = false;
        cache_data.constructed = true;
        cache_data.num_chars = num_chars as u16;
        cache_data.width = basex as u16;
    }

    pub fn get_length(&self, idx: usize) -> usize {
        self.get_data_ref(idx).num_chars as usize
    }

    pub fn get_width(&self, idx: usize) -> usize {
        if self.reconstruct(idx) {
            // force recalc if hasn't happened on its own yet
            self.recalc_width(idx);
        }
        self.get_data_ref(idx).width as usize
    }

    fn get_data_ref_mut(&self, idx: usize) -> &mut TextRenderComponentManagerData {
        assert!(idx < ENTITY_SZ);
        unsafe { &mut *(self.cache_data.offset(idx as isize)) }
    }

    fn get_data_ref(&self, idx: usize) -> &TextRenderComponentManagerData {
        assert!(idx < ENTITY_SZ);
        unsafe { &*(self.cache_data.offset(idx as isize)) }
    }
}
