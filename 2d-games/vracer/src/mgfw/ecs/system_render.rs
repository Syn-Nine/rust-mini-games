use super::*;
use crate::mgfw::log;

struct RenderSystemData {
    vao_pri: u32,
    vbo_pri: u32,
}

pub struct RenderSystem {
    data: *mut RenderSystemData,
    // WARNING: Anything below this line is not in cache!
    frame: usize,
    screen_vao: u32,
}

#[allow(dead_code)]
impl RenderSystem {
    pub fn new(mgr: &mut CacheManager, gl: &Gl) -> RenderSystem {
        log(format!("Constructing RenderSystem"));
        // allocate system memory in cache
        let sz_bytes = std::mem::size_of::<RenderSystemData>() * ENTITY_SZ;
        let data = mgr.allocate(sz_bytes) as *mut RenderSystemData;

        // screen quad VAO
        let screen_vao: u32 = gl.gen_vao();
        let screen_vbo: u32 = gl.gen_vbo();

        let mut vertex_data: Vec<f32> = Vec::new();

        let w: f32 = 1.0;

        vertex_data.extend_from_slice(&[-w, -w]); // pos
        vertex_data.extend_from_slice(&[0.0, 0.0]); // uv

        vertex_data.extend_from_slice(&[-w, w]); // pos
        vertex_data.extend_from_slice(&[0.0, 1.0]); // uv

        vertex_data.extend_from_slice(&[w, w]); // pos
        vertex_data.extend_from_slice(&[1.0, 1.0]); // uv

        vertex_data.extend_from_slice(&[-w, -w]); // pos
        vertex_data.extend_from_slice(&[0.0, 0.0]); // uv

        vertex_data.extend_from_slice(&[w, w]); // pos
        vertex_data.extend_from_slice(&[1.0, 1.0]); // uv

        vertex_data.extend_from_slice(&[w, -w]); // pos
        vertex_data.extend_from_slice(&[1.0, 0.0]); // uv

        gl.buffer_billboard_data(screen_vao, screen_vbo, vertex_data.as_ptr() as *const _);

        let ret = RenderSystem {
            data,
            frame: 0,
            screen_vao,
        };

        // pre-generate a VAO/VBO for each entity
        for i in 0..ENTITY_SZ {
            let d = ret.get_data_ref_mut(i);
            d.vao_pri = gl.gen_vao();
            d.vbo_pri = gl.gen_vbo();
        }
        ret
    }

    fn skip_entity(&self, idx: usize, world: &World) -> bool {
        let ent = world.get_entities();
        let rcm = world.get_manager_render();
        if !ent.is_active(idx)
            || !ent.has_component(idx, COMPONENT_RENDER)
            || rcm.has_type(idx, RENDER_TYPE_INVALID)
        {
            return true;
        }
        false
    }

    pub fn update(&mut self, gl: &Gl, world: &mut World) -> bool {
        let mut expect_blown = false;

        let span = world.get_entities().get_id_span();
        for i in span.first..=span.last {
            if self.skip_entity(i, world) {
                continue;
            }

            expect_blown |= self.update_vbo(i, gl, world);
            
            //// to do - figure out why this doesn't work correctly
            /*// Amortize workload
            match self.frame % 2 {
                // priority 1
                0 => (),

                // priority 2
                1 => {
                    //expect_blown |= self.update_vbo(i, gl, world);
                    if expect_blown {
                        //break;
                    } // don't update all VBOs every frame
                }
                _ => (),
            }*/
        }
        self.frame += 1;
        expect_blown
    }

    // WARNING: Expect Blown
    fn update_vbo(&self, idx: usize, gl: &Gl, world: &mut World) -> bool {
        let mut expect_blown = false;

        // Update Text VBOs
        match world.get_manager_render().get_type(idx) {
            RENDER_TYPE_LINE_BUFFER => {
                if world.get_manager_line().reconstruct(idx) {
                    let dat = self.get_data_ref(idx);
                    world.line_buffer_construct(idx, gl, dat.vao_pri, dat.vbo_pri);
                    expect_blown = true;
                }
            }
            RENDER_TYPE_TRIANGLE_BUFFER => {
                if world.get_manager_triangle().reconstruct(idx) {
                    let dat = self.get_data_ref(idx);
                    world.triangle_buffer_construct(idx, gl, dat.vao_pri, dat.vbo_pri);
                    expect_blown = true;
                }
            }
            RENDER_TYPE_TEXT => {
                if world.get_manager_text().reconstruct(idx) {
                    let dat = self.get_data_ref(idx);
                    world.text_construct(idx, gl, dat.vao_pri, dat.vbo_pri);
                    expect_blown = true;
                }
            }
            RENDER_TYPE_BILLBOARD => {
                if world.get_manager_billboard().reconstruct(idx) {
                    let dat = self.get_data_ref(idx);
                    world.billboard_construct(idx, gl, dat.vao_pri, dat.vbo_pri);
                    expect_blown = true;
                }
            }
            RENDER_TYPE_TILEMAP => {
                if world.get_manager_tilemap().reconstruct(idx) {
                    let tileset = world.tilemap_get_tileset_idx(idx);
                    world.billboard_load_image(tileset, gl);
                    let dat = self.get_data_ref(idx);
                    world.tilemap_construct(idx, gl, dat.vao_pri, dat.vbo_pri);
                    expect_blown = true;
                }
            }
            _ => (),
        }
        expect_blown
    }

    pub fn render(&self, gl: &Gl, world: &World, start_time: std::time::Instant) {
        let pcm = world.get_manager_position();
        let scm = world.get_manager_scale();
        let acm = world.get_manager_angle();
        let fcm = world.get_manager_frame();
        let phcm = world.get_manager_physics();
        let rcm = world.get_manager_render();
        let tcm = world.get_manager_text();
        let bbcm = world.get_manager_billboard();
        let tmcm = world.get_manager_tilemap();
        let lcm = world.get_manager_line();
        let trm = world.get_manager_triangle();
        let pjcm = world.get_manager_projection();
        let ent = world.get_entities();
        
        gl.bind_framebuffer();
        gl.clear_frame();

        let span = ent.get_id_span();
        for i in span.first..=span.last {
            if !ent.is_visible(i) || self.skip_entity(i, world) {
                continue;
            }
            let color = world.entity_get_color(i);

            let mut angle = 0.0 as f32;
            if ent.has_component(i, COMPONENT_ANGLE) {
                angle = acm.get_angle(i);
            }

            let mut scale = Scale { x: 1.0, y: 1.0 };
            if ent.has_component(i, COMPONENT_SCALE) {
                scale = scm.get_scale(i);
            }

            let dt = std::time::Instant::now()
                .duration_since(start_time)
                .as_micros() as f32
                * 1.0e-6;

            match rcm.get_type(i) {
                RENDER_TYPE_LINE_BUFFER => {

                    let mut perspective = false;
                    if ent.has_component(i, COMPONENT_PROJECTION) {
                        perspective = match pjcm.get_projection(i) { PROJECTION_MODE_ORTHO => false, PROJECTION_MODE_PERSPECTIVE => true, _ => false };
                    }

                    if lcm.is_constructed(i) {
                        let vao = self.get_data_ref(i).vao_pri;
                        let pos = pcm.get_data_ref(i);
                        let phys = phcm.get_data_ref(i);
                        gl.draw_lines(
                            pos.x + phys.velocity.x * dt,
                            pos.y + phys.velocity.y * dt,
                            angle,
                            scale.x,
                            scale.y,
                            vao,
                            lcm.get_num_lines(i),
                            color,
                            perspective,
                        );
                    }
                }
                RENDER_TYPE_TRIANGLE_BUFFER => {
                    if trm.is_constructed(i) {
                        let vao = self.get_data_ref(i).vao_pri;
                        let pos = pcm.get_data_ref(i);
                        let phys = phcm.get_data_ref(i);
                        gl.draw_triangles(
                            pos.x + phys.velocity.x * dt,
                            pos.y + phys.velocity.y * dt,
                            angle,
                            scale.x,
                            scale.y,
                            vao,
                            trm.get_num_triangles(i),
                            color,
                        );
                    }
                }
                RENDER_TYPE_TEXT => {
                    if tcm.is_constructed(i) {//&& !tcm.reconstruct(i) {
                        let vao = self.get_data_ref(i).vao_pri;
                        let pos = pcm.get_data_ref(i);
                        let phys = phcm.get_data_ref(i);
                        gl.draw_text(
                            pos.x + phys.velocity.x * dt,
                            pos.y + phys.velocity.y * dt,
                            angle,
                            scale.x,
                            scale.y,
                            vao,
                            tcm.get_length(i),
                            color,
                        );
                    }
                }
                RENDER_TYPE_BILLBOARD => {
                    if bbcm.is_constructed(i) {
                        let vao = self.get_data_ref(i).vao_pri;
                        let pos = pcm.get_data_ref(i);
                        let phys = phcm.get_data_ref(i);

                        let mut framed = false;
                        let mut u = 0.0 as f32;
                        let mut v = 0.0 as f32;
                        let mut du = 1.0 as f32;
                        let mut dv = 1.0 as f32;
                        if ent.has_component(i, COMPONENT_FRAME) {
                            framed = true;
                            du = 32.0 / 128.0;
                            dv = 32.0 / 128.0;

                            let frame = fcm.get_frame(i);
                            u = (frame % 4) as f32 * du;
                            v = ((frame - (frame % 4)) / 4) as f32 * dv;
                        }

                        gl.draw_billboard(
                            pos.x + phys.velocity.x * dt,
                            pos.y + phys.velocity.y * dt,
                            angle,
                            scale.x,
                            scale.y,
                            vao,
                            bbcm.get_tex_handle(i),
                            color,
                            framed,
                            u,
                            v,
                            du,
                            dv,
                        );
                    }
                }
                RENDER_TYPE_TILEMAP => {
                    if tmcm.is_constructed(i) {
                        let vao = self.get_data_ref(i).vao_pri;
                        let pos = pcm.get_data_ref(i);
                        let phys = phcm.get_data_ref(i);
                        let num_tiles = tmcm.get_num_tiles(i);
                        let tex_handle = bbcm.get_tex_handle(tmcm.get_tileset_idx(i));
                        gl.draw_tilemap(
                            pos.x + phys.velocity.x * dt,
                            pos.y + phys.velocity.y * dt,
                            angle,
                            scale.x,
                            scale.y,
                            vao,
                            num_tiles,
                            tex_handle,
                            color,
                        );
                    }
                }

                _ => (),
            }
        }

        gl.unbind_framebuffer();
        gl.clear_frame();

        gl.draw_screen_billboard(
            self.screen_vao,
            Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            },
        );
    }

    fn get_data_ref_mut(&self, idx: usize) -> &mut RenderSystemData {
        assert!(idx < ENTITY_SZ);
        unsafe { &mut *(self.data.offset(idx as isize)) }
    }

    fn get_data_ref(&self, idx: usize) -> &RenderSystemData {
        assert!(idx < ENTITY_SZ);
        unsafe { &*(self.data.offset(idx as isize)) }
    }
}
