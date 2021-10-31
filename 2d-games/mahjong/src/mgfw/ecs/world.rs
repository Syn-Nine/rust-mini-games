use crate::mgfw::log;
use std::fs::File;
use std::io::{self, BufRead};

use super::*;

pub struct World {
    // WARNING: Anything below this line is not in cache!
    ent: std::boxed::Box<EntityRegistry>,
    pcm: std::boxed::Box<PositionComponentManager>,
    scm: std::boxed::Box<ScaleComponentManager>,
    acm: std::boxed::Box<AngleComponentManager>,
    phcm: std::boxed::Box<PhysicsComponentManager>,
    rcm: std::boxed::Box<RenderComponentManager>,
    tcm: std::boxed::Box<TextRenderComponentManager>,
    bbcm: std::boxed::Box<BillboardRenderComponentManager>,
    lcm: std::boxed::Box<LineRenderComponentManager>,
    trm: std::boxed::Box<TriangleRenderComponentManager>,
    ccm: std::boxed::Box<ColorComponentManager>,
    ecm: std::boxed::Box<EasingComponentManager>,
    pub mouse_x: i32,
    pub mouse_y: i32,
}

#[allow(dead_code)]
impl World {
    pub fn new(cache: &mut CacheManager) -> World {
        log(format!("Constructing World"));
        World {
            ent: Box::new(EntityRegistry::new(cache)),
            pcm: Box::new(PositionComponentManager::new(cache)),
            scm: Box::new(ScaleComponentManager::new(cache)),
            acm: Box::new(AngleComponentManager::new(cache)),
            phcm: Box::new(PhysicsComponentManager::new(cache)),
            rcm: Box::new(RenderComponentManager::new(cache)),
            tcm: Box::new(TextRenderComponentManager::new(cache)),
            bbcm: Box::new(BillboardRenderComponentManager::new(cache)),
            lcm: Box::new(LineRenderComponentManager::new(cache)),
            trm: Box::new(TriangleRenderComponentManager::new(cache)),
            ccm: Box::new(ColorComponentManager::new(cache)),
            ecm: Box::new(EasingComponentManager::new(cache)),
            mouse_x: 0,
            mouse_y: 0,
        }
    }

    pub fn new_entity(&mut self) -> usize {
        self.ent.add()
    }

    pub fn entity_add_component(&mut self, idx: usize, component: u32) {
        self.ent.add_component(idx, component);
    }

    pub fn entity_get_angle(&mut self, idx: usize) -> f32 {
        self.acm.get_angle(idx)
    }

    pub fn entity_set_angle(&mut self, idx: usize, angle: f32) {
        self.acm.set_angle(idx, angle);
        self.ent.add_component(idx, COMPONENT_ANGLE);
    }

    pub fn entity_set_color(&mut self, idx: usize, color: Color) {
        self.ccm.set_color(idx, color);
        self.ent.add_component(idx, COMPONENT_COLOR);
    }

    pub fn entity_set_color_rgba(&mut self, idx: usize, r: f32, g: f32, b: f32, a: f32) {
        self.ccm.set_color_rgba(idx, r, g, b, a);
        self.ent.add_component(idx, COMPONENT_COLOR);
    }

    pub fn entity_get_color(&self, idx: usize) -> Color {
        self.ccm.get_color(idx)
    }

    pub fn entity_set_alpha(&mut self, idx: usize, alpha: f32) {
        self.ccm.set_alpha(idx, alpha);
        self.ent.add_component(idx, COMPONENT_COLOR);
    }

    pub fn entity_set_alpha_ease(&mut self, idx: usize, start: f32, end: f32, dt: f32) {
        self.entity_set_alpha(idx, start);
        self.ecm.set_alpha_ease(idx, start, end, dt);
        self.ent.add_component(idx, COMPONENT_COLOR);
    }

    pub fn entity_get_alpha(&self, idx: usize) -> f32 {
        self.ccm.get_alpha(idx)
    }
    pub fn entity_get_position(&mut self, idx: usize) -> Position {
        self.pcm.get_position(idx)
    }

    pub fn entity_set_position(&mut self, idx: usize, pos: Position) {
        self.pcm.set_position(idx, pos.x, pos.y);
        self.ent.add_component(idx, COMPONENT_POSITION);
    }

    pub fn entity_set_position_xy(&mut self, idx: usize, x: f32, y: f32) {
        self.pcm.set_position(idx, x, y);
        self.ent.add_component(idx, COMPONENT_POSITION);
    }

    pub fn entity_get_scale(&mut self, idx: usize) -> Scale {
        self.scm.get_scale(idx)
    }

    pub fn entity_set_scale(&mut self, idx: usize, pos: Scale) {
        self.scm.set_scale(idx, pos.x, pos.y);
        self.ent.add_component(idx, COMPONENT_SCALE);
    }

    pub fn entity_set_scale_xy(&mut self, idx: usize, x: f32, y: f32) {
        self.scm.set_scale(idx, x, y);
        self.ent.add_component(idx, COMPONENT_SCALE);
    }

    pub fn entity_get_velocity(&mut self, idx: usize) -> Velocity {
        self.phcm.get_velocity(idx)
    }

    pub fn entity_get_acceleration(&mut self, idx: usize) -> Acceleration {
        self.phcm.get_acceleration(idx)
    }

    pub fn entity_get_angular_velocity(&mut self, idx: usize) -> f32 {
        self.phcm.get_angular_velocity(idx)
    }
    pub fn entity_set_angular_velocity(&mut self, idx: usize, vel: f32) {
        self.phcm.set_angular_velocity(idx, vel);
        self.ent.add_component(idx, COMPONENT_ANGLE);
        self.ent.add_component(idx, COMPONENT_PHYSICS);
    }

    pub fn entity_set_velocity(&mut self, idx: usize, vel: Velocity) {
        self.phcm.set_velocity(idx, vel.x, vel.y);
        self.ent.add_component(idx, COMPONENT_POSITION);
        self.ent.add_component(idx, COMPONENT_PHYSICS);
    }

    pub fn entity_set_velocity_xy(&mut self, idx: usize, x: f32, y: f32) {
        self.phcm.set_velocity(idx, x, y);
        self.ent.add_component(idx, COMPONENT_POSITION);
        self.ent.add_component(idx, COMPONENT_PHYSICS);
    }

    pub fn entity_set_acceleration(&mut self, idx: usize, accel: Acceleration) {
        self.phcm.set_acceleration(idx, accel.x, accel.y);
        self.ent.add_component(idx, COMPONENT_POSITION);
        self.ent.add_component(idx, COMPONENT_PHYSICS);
    }

    pub fn entity_set_acceleration_xy(&mut self, idx: usize, x: f32, y: f32) {
        self.phcm.set_acceleration(idx, x, y);
        self.ent.add_component(idx, COMPONENT_POSITION);
        self.ent.add_component(idx, COMPONENT_PHYSICS);
    }

    pub fn entity_set_text(&mut self, idx: usize, text: String) {
        self.tcm.set_text(idx, text);
        self.ent.add_component(idx, COMPONENT_RENDER);
        self.rcm.set_type(idx, RENDER_TYPE_TEXT);
    }

    pub fn entity_set_billboard(&mut self, idx: usize, image: String) {
        self.bbcm.set_image(idx, image);
        self.ent.add_component(idx, COMPONENT_RENDER);
        self.rcm.set_type(idx, RENDER_TYPE_BILLBOARD);
    }

    pub fn entity_get_billboard(&self, idx: usize) -> String {
        self.bbcm.get_image(idx)
    }

    pub fn entity_set_line_buffer(&mut self, idx: usize, pnts: &Vec<Position>, clrs: &Vec<Color>) {
        self.lcm.set_line_buffer(idx, pnts, clrs);
        self.ent.add_component(idx, COMPONENT_RENDER);
        self.rcm.set_type(idx, RENDER_TYPE_LINE_BUFFER);
    }

    pub fn entity_set_triangle_buffer(
        &mut self,
        idx: usize,
        pnts: &Vec<Position>,
        clrs: &Vec<Color>,
    ) {
        self.trm.set_triangle_buffer(idx, pnts, clrs);
        self.ent.add_component(idx, COMPONENT_RENDER);
        self.rcm.set_type(idx, RENDER_TYPE_TRIANGLE_BUFFER);
    }

    pub fn entity_set_active(&mut self, idx: usize, val: bool) {
        self.ent.set_active(idx, val);
    }

    pub fn entity_is_active(&self, idx: usize) -> bool {
        self.ent.is_active(idx)
    }

    pub fn entity_set_visibility(&mut self, idx: usize, val: bool) {
        self.ent.set_visibility(idx, val);
    }

    pub fn entity_is_visible(&self, idx: usize) -> bool {
        self.ent.is_visible(idx)
    }

    pub fn get_entities(&self) -> &EntityRegistry {
        &self.ent
    }

    // Managers should not be mutable! Use interface functions instead.

    pub fn get_manager_position(&self) -> &PositionComponentManager {
        &self.pcm
    }

    pub fn get_manager_scale(&self) -> &ScaleComponentManager {
        &self.scm
    }

    pub fn get_manager_angle(&self) -> &AngleComponentManager {
        &self.acm
    }

    pub fn get_manager_text(&self) -> &TextRenderComponentManager {
        &self.tcm
    }

    pub fn get_manager_billboard(&self) -> &BillboardRenderComponentManager {
        &self.bbcm
    }

    pub fn get_manager_render(&self) -> &RenderComponentManager {
        &self.rcm
    }

    pub fn get_manager_physics(&self) -> &PhysicsComponentManager {
        &self.phcm
    }

    pub fn get_manager_line(&self) -> &LineRenderComponentManager {
        &self.lcm
    }

    pub fn get_manager_triangle(&self) -> &TriangleRenderComponentManager {
        &self.trm
    }

    pub fn get_manager_color(&self) -> &ColorComponentManager {
        &self.ccm
    }

    pub fn get_manager_easing(&self) -> &EasingComponentManager {
        &self.ecm
    }

    pub fn easing_disable(&mut self, idx: usize) {
        self.ecm.deactivate(idx);
    }

    pub fn text_get_width(&self, idx: usize) -> usize {
        self.tcm.get_width(idx)
    }

    pub fn text_construct(&self, idx: usize, gl: &Gl, vao: u32, vbo: u32) {
        self.tcm.construct(idx, gl, vao, vbo);
    }

    pub fn text_is_constructed(&self, idx: usize) -> bool {
        self.tcm.is_constructed(idx)
    }

    pub fn text_reconstruct(&self, idx: usize) -> bool {
        self.tcm.reconstruct(idx)
    }

    pub fn billboard_construct(&mut self, idx: usize, gl: &Gl, vao: u32, vbo: u32) {
        self.bbcm.construct(idx, gl, vao, vbo);
    }

    pub fn line_buffer_construct(&self, idx: usize, gl: &Gl, vao: u32, vbo: u32) {
        self.lcm.construct(idx, gl, vao, vbo);
    }

    pub fn triangle_buffer_construct(&self, idx: usize, gl: &Gl, vao: u32, vbo: u32) {
        self.trm.construct(idx, gl, vao, vbo);
    }

    pub fn parse_world(&mut self, filename: &str) {
        log(format!("World: Parsing '{}'", filename));

        let file = File::open(filename).unwrap();
        let reader = io::BufReader::new(file);

        for line in reader.lines() {
            let line = line.unwrap();

            let len = line.len();
            if 2 > len {
                continue;
            }

            let bytes = line.as_bytes();
            if b'/' == bytes[0] && b'/' == bytes[1] {
                continue;
            }

            //log(format!("{:?}", line));

            let split: Vec<&str> = line.split(',').collect();
            if 1 > split.len() {
                continue;
            }

            let id = split[0].parse::<usize>().unwrap();
            if id >= entity::ENTITY_SZ {
                continue;
            }

            let component = split[1];

            match component {
                "text" => {
                    if 3 == split.len() {
                        let val = split[2].replace("\"", "");
                        self.ent.add_component(id, COMPONENT_ACTIVE);
                        self.entity_set_text(id, val);
                    }
                }
                "billboard" => {
                    if 3 == split.len() {
                        let image = split[2];
                        self.ent.add_component(id, COMPONENT_ACTIVE);
                        self.entity_set_billboard(id, String::from(image));
                    }
                }
                "linebuffer" => {
                    let n = split.len() - 2;
                    let n_lines: usize = n / 12;
                    let n_points: usize = n_lines * 2;
                    if n_lines * 12 == n {
                        let mut pnts: Vec<Position> = Vec::new();
                        let mut clrs: Vec<Color> = Vec::new();

                        for p in 0..n_points {
                            let pidx: usize = 2 + p * 6;
                            pnts.push(Position {
                                x: split[pidx + 0].parse::<f32>().unwrap(),
                                y: split[pidx + 1].parse::<f32>().unwrap(),
                            });
                            clrs.push(Color {
                                r: split[pidx + 2].parse::<f32>().unwrap(),
                                g: split[pidx + 3].parse::<f32>().unwrap(),
                                b: split[pidx + 4].parse::<f32>().unwrap(),
                                a: split[pidx + 5].parse::<f32>().unwrap(),
                            });
                        }
                        self.ent.add_component(id, COMPONENT_ACTIVE);
                        self.entity_set_line_buffer(id, &pnts, &clrs);
                    }
                }
                "tribuffer" => {
                    let n = split.len() - 2;
                    let n_triangles: usize = n / 18;
                    let n_points: usize = n_triangles * 3;
                    if n_triangles * 18 == n {
                        let mut pnts: Vec<Position> = Vec::new();
                        let mut clrs: Vec<Color> = Vec::new();

                        for p in 0..n_points {
                            let pidx: usize = 2 + p * 6;
                            pnts.push(Position {
                                x: split[pidx + 0].parse::<f32>().unwrap(),
                                y: split[pidx + 1].parse::<f32>().unwrap(),
                            });
                            clrs.push(Color {
                                r: split[pidx + 2].parse::<f32>().unwrap(),
                                g: split[pidx + 3].parse::<f32>().unwrap(),
                                b: split[pidx + 4].parse::<f32>().unwrap(),
                                a: split[pidx + 5].parse::<f32>().unwrap(),
                            });
                        }
                        self.ent.add_component(id, COMPONENT_ACTIVE);
                        self.entity_set_triangle_buffer(id, &pnts, &clrs);
                    }
                }
                "visible" => {
                    if 3 == split.len() {
                        let val = split[2];
                        match val {
                            "true" => self.ent.set_visibility(id, true),
                            "false" => self.ent.set_visibility(id, false),
                            _ => (),
                        }
                    }
                }
                "position" => {
                    if 4 == split.len() {
                        let x = split[2].parse::<f32>().unwrap();
                        let y = split[3].parse::<f32>().unwrap();
                        self.ent.add_component(id, COMPONENT_ACTIVE);
                        self.entity_set_position_xy(id, x, y);
                    }
                }
                "velocity" => {
                    if 4 == split.len() {
                        let x = split[2].parse::<f32>().unwrap();
                        let y = split[3].parse::<f32>().unwrap();
                        self.ent.add_component(id, COMPONENT_ACTIVE);
                        self.entity_set_velocity_xy(id, x, y);
                    }
                }
                "scale" => {
                    if 3 <= split.len() {
                        let x = split[2].parse::<f32>().unwrap();
                        let mut y = 1.0 as f32;
                        if 4 == split.len() {
                            y = split[3].parse::<f32>().unwrap();
                        }
                        self.ent.add_component(id, COMPONENT_ACTIVE);
                        self.entity_set_scale_xy(id, x, y);
                    }
                }
                "angle" => {
                    if 3 == split.len() {
                        let angle = split[2].parse::<f32>().unwrap();
                        self.ent.add_component(id, COMPONENT_ACTIVE);
                        self.entity_set_angle(id, crate::mgfw::deg2rad(angle));
                    }
                }
                "alpha" => {
                    if 3 == split.len() {
                        let alpha = split[2].parse::<f32>().unwrap();
                        self.ent.add_component(id, COMPONENT_COLOR);
                        self.entity_set_alpha(id, alpha);
                    }
                }
                "angular_velocity" => {
                    if 3 == split.len() {
                        let avel = split[2].parse::<f32>().unwrap();
                        self.ent.add_component(id, COMPONENT_ACTIVE);
                        self.entity_set_angular_velocity(id, crate::mgfw::deg2rad(avel));
                    }
                }
                "acceleration" => {
                    if 4 == split.len() {
                        let x = split[2].parse::<f32>().unwrap();
                        let y = split[3].parse::<f32>().unwrap();
                        self.ent.add_component(id, COMPONENT_ACTIVE);
                        self.entity_set_acceleration_xy(id, x, y);
                    }
                }
                _ => (),
            }
        }
    }
}
