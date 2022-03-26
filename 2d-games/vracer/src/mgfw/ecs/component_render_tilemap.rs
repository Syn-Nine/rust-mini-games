use super::*;
use crate::mgfw::log;

struct TilesetBuffer {
    // WARNING: Anything below this line is not in cache!
    image_width: usize,
    image_height: usize,
    tile_width: usize,
    tile_height: usize,
    span: usize,
    count: usize,
}

struct TilemapBuffer {
    // WARNING: Anything below this line is not in cache!
    data: Vec<u16>,
}

struct TilemapRenderComponentManagerData {
    columns: usize,
    rows: usize,
    constructed: bool,
    reconstruct_needed: bool,
    tileset: usize,
    num_tiles: u16,
}

pub struct TilemapRenderComponentManager {
    cache_data: *mut TilemapRenderComponentManagerData,
    // WARNING: Anything below this line is not in cache!
    tileset: std::boxed::Box<Vec<TilesetBuffer>>,
    tilemap: std::boxed::Box<Vec<TilemapBuffer>>,
}

#[allow(dead_code)]
impl TilemapRenderComponentManager {
    pub fn new(mgr: &mut CacheManager) -> TilemapRenderComponentManager {
        log(format!("Constructing TilemapRenderComponentManager"));

        let mut tileset: Vec<TilesetBuffer> = Vec::new();
        let mut tilemap: Vec<TilemapBuffer> = Vec::new();
        for _i in 0..ENTITY_SZ {
            tileset.push(TilesetBuffer {
                tile_width: 16,
                tile_height: 16,
                image_width: 320,
                image_height: 240,
                span: 16,
                count: 1,
            });
            tilemap.push(TilemapBuffer { data: Vec::new() });
        }

        // allocate system memory in cache
        let sz_bytes = std::mem::size_of::<TilemapRenderComponentManagerData>() * ENTITY_SZ;
        let cache_data = mgr.allocate(sz_bytes) as *mut TilemapRenderComponentManagerData;

        TilemapRenderComponentManager {
            tileset: Box::new(tileset),
            tilemap: Box::new(tilemap),
            cache_data,
        }
    }

    pub fn clear(&mut self) {
        
    }

    pub fn set_tileset(
        &mut self,
        idx: usize,
        image_width: usize,
        image_height: usize,
        tile_width: usize,
        tile_height: usize,
    ) {
        self.tileset[idx].image_width = image_width;
        self.tileset[idx].image_height = image_height;
        self.tileset[idx].tile_width = tile_width;
        self.tileset[idx].tile_height = tile_height;

        self.tileset[idx].span = (image_width - (image_width % tile_width)) / tile_width;
        self.tileset[idx].count =
            self.tileset[idx].span * (image_height - (image_height % tile_height)) / tile_height;

        println!(
            "tileset {},{},{}",
            idx, self.tileset[idx].span, self.tileset[idx].count
        );
    }

    pub fn set_tilemap(&mut self, idx: usize, tileset_idx: usize, columns: usize, data: &Vec<u16>) {
        let cache_data = self.get_data_ref_mut(idx);
        cache_data.reconstruct_needed = true;
        cache_data.columns = columns;
        cache_data.tileset = tileset_idx;
        let n = data.len();
        assert!(0 != columns);
        assert!(0 != n);
        assert!(0 == n % columns);
        cache_data.rows = (n - (n % columns)) / columns;
        self.tilemap[idx].data = data.clone();
    }

    pub fn is_constructed(&self, idx: usize) -> bool {
        self.get_data_ref(idx).constructed
    }

    pub fn reconstruct(&self, idx: usize) -> bool {
        self.get_data_ref(idx).reconstruct_needed
    }

    pub fn get_tileset_idx(&self, idx: usize) -> usize {
        self.get_data_ref(idx).tileset
    }

    pub fn get_num_tiles(&self, idx: usize) -> usize {
        self.get_data_ref(idx).num_tiles as usize
    }

    pub fn construct(&self, idx: usize, gl: &Gl, vao: u32, vbo: u32) {
        let cache_data = self.get_data_ref_mut(idx);
        let cols = cache_data.columns;

        let mut vertex_data: Vec<f32> = Vec::new();

        let map = &self.tilemap[idx].data;

        let mut num_tiles: usize = 0;

        let tileset = &self.tileset[cache_data.tileset];
        let uscale = tileset.tile_width as f32 / tileset.image_width as f32;
        let vscale = tileset.tile_height as f32 / tileset.image_height as f32;
        let usub = uscale * 0.0; //(0.2 / tileset.tile_width as f32);
        let vsub = uscale * 0.0; //(0.2 / tileset.tile_height as f32);

        for i in 0..map.len() {
            let t0 = map[i] as usize;
            if EMPTY_TILE == t0 as u16 || tileset.count < t0 {
                continue;
            }
            let t0 = t0 - 1;

            let u0 = (t0 % tileset.span) as f32 * uscale + usub;
            let v0 = ((t0 - (t0 % tileset.span)) / tileset.span) as f32 * vscale + vsub;
            let u1 = u0 + uscale - usub;
            let v1 = v0 + vscale - vsub;

            let x0 = (i % cols) as f32;
            let y0 = 1.0 * (((i - (i % cols)) / cols) as f32);
            let x1 = x0 + 1.0;
            let y1 = y0 + 1.0;

            vertex_data.push(x0);
            vertex_data.push(y0);
            vertex_data.push(u0);
            vertex_data.push(v0);
            vertex_data.push(x0);
            vertex_data.push(y1);
            vertex_data.push(u0);
            vertex_data.push(v1);
            vertex_data.push(x1);
            vertex_data.push(y1);
            vertex_data.push(u1);
            vertex_data.push(v1);

            vertex_data.push(x0);
            vertex_data.push(y0);
            vertex_data.push(u0);
            vertex_data.push(v0);
            vertex_data.push(x1);
            vertex_data.push(y1);
            vertex_data.push(u1);
            vertex_data.push(v1);
            vertex_data.push(x1);
            vertex_data.push(y0);
            vertex_data.push(u1);
            vertex_data.push(v0);

            num_tiles += 1;
        }

        let data_ptr = vertex_data.as_ptr() as *const _;
        gl.buffer_tilemap_data(vao, vbo, num_tiles, data_ptr);

        cache_data.reconstruct_needed = false;
        cache_data.constructed = true;
        cache_data.num_tiles = num_tiles as u16;
        println!("Constructing tilemap {}", idx);
    }

    fn get_data_ref_mut(&self, idx: usize) -> &mut TilemapRenderComponentManagerData {
        assert!(idx < ENTITY_SZ);
        unsafe { &mut *(self.cache_data.offset(idx as isize)) }
    }

    fn get_data_ref(&self, idx: usize) -> &TilemapRenderComponentManagerData {
        assert!(idx < ENTITY_SZ);
        unsafe { &*(self.cache_data.offset(idx as isize)) }
    }
}
