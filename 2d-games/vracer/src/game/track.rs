use crate::mgfw;
use crate::game::game;
use crate::game::menu;

#[derive(Copy, Clone, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
}

#[derive(Copy, Clone, Default)]
pub struct Quad {
    pnts: [Point; 4],
    sign: f32,
}

impl Quad {
    pub fn new() -> Quad {
        let p: Point = Point::new();
        Quad { pnts: [p; 4], sign: 0.0 }
    }
}

pub const MAX_QUADS: usize = 640;
pub const MAX_NPCS: usize = 3;
pub const MAX_TRACKS: usize = 10;

#[derive(Copy, Clone)]
pub struct Track {
    quads: [Quad; MAX_QUADS],
    num_quads: usize,
    waypoints: [Point; MAX_QUADS],
}

impl Track {
    pub fn new() -> Track {
        let q: Quad = Quad::new();
        let p: Point = Point::new();
        Track { quads: [q; MAX_QUADS], waypoints: [p; MAX_QUADS], num_quads: 0 }
    }
}

pub struct Ship {
    position: Point,
    velocity: Point,
    track_ent: usize,
    minimap_ent: usize,
    hint_left: bool,
    hint_right: bool,
    hint_up: f32,
    angle: f32,
    next_waypoint: usize,
    skill: f32,
    drive: f32,
    start_time: f64,
    lap: usize,
    lap_timer: f64,
    pub best_timer: f64,
    pub place: usize,
    damage: f32,
    quadrant: usize,
    //
    pub thrust: i8,
    pub steering: i8,
    pub shield: i8,
}

pub struct TrackData {
    track: Track,
    camera: Point,
    scale: f32,
    track_ent: usize,
    move_up: bool,
    move_down: bool,
    move_left: bool,
    move_right: bool,
    minimap_ent: usize,
    minimap_scale: f32,
    pub player: Ship,
    npcs: [Ship; MAX_NPCS],
    countdown: f64,
    ui_timer: f64,
    damage_ent: usize,
    damage_pulse: f64,
    destroyed_ent: usize,
    race_over_ent: usize,
    pub dead: bool,
    race_over: bool,
    race_over_timer: f64,
    pub track_locked: [bool; MAX_TRACKS],
    pub cur_track: usize,
}


#[rustfmt::skip]
pub fn initialize(cache: &mut game::GameData, heap: &mut game::GameDataHeap, world: &mut mgfw::ecs::World) {

    heap.track_ref.clear();

    for _i in 0..MAX_TRACKS {
        gen_track(heap, world);
    }

}

pub fn init_player(cache: &mut game::GameData) {
    cache.track_data.player.thrust = 1;
    cache.track_data.player.steering = 1;
    cache.track_data.player.shield = 1;
    cache.track_data.cur_track = 0;

    for i in 0..MAX_TRACKS {
        cache.track_data.track_locked[i] = false;
        if 2 < i { cache.track_data.track_locked[i] = true; }
    }
}

fn gen_track(heap: &mut game::GameDataHeap, world: &mut mgfw::ecs::World) {

    let mut track: Track = Track::new();

    let mut sign: Vec<f32> = Vec::new();
    let mut legs: Vec<(mgfw::ecs::Position, mgfw::ecs::Position)> = Vec::new();
    let mut corners: Vec<(mgfw::ecs::Position, mgfw::ecs::Position, mgfw::ecs::Position)> = Vec::new();
    let mut segs: Vec<(mgfw::ecs::Position, mgfw::ecs::Position)> = Vec::new();

    let pattern = heap.track_ref.len();

    let grid: [u8; 16] = match pattern {
        0 => [  
            1,2,3,0,
            6,0,4,0,
            0,0,5,0,
            0,0,0,0
        ],
        1 => [  
            1,7,6,5,
            2,0,0,4,
            0,0,3,0,
            0,0,0,0
        ],
        2 => [  
            0,1,2,0,
            5,0,0,0,
            0,4,0,3,
            0,0,0,0
        ],
        3 => [  
            0,2,3,0,
            0,0,4,0,
            1,0,0,0,
            6,5,0,0
        ],
        4 => [  
            1,10,9,8,
            2,0,0,7,
            0,3,6,0,
            0,4,5,0
        ],
        5 => [  
            0,1,2,0,
            8,0,0,3,
            7,0,0,4,
            0,6,5,0
        ],
        6 => [  
            1,13,10,9,
            2,12,11,8,
            3,0,6,7,
            4,5,0,0
        ],
        7 => [  
            0,0,2,3,
            0,1,0,4,
            9,10,0,5,
            8,7,6,0
        ],
        8 => [  
            1,14,0,0,
            2,13,10,9,
            3,12,11,8,
            4,5,6,7
        ],
        9 => [  
            1,16,13,12,
            2,15,14,11,
            3,6,7,10,
            4,5,8,9
        ],
        _ => { assert_eq!(false, true); [0; 16] },
    };


    let mut ord: [u8; 17] = [0; 17];

    let mut cell: Vec<mgfw::ecs::Position> = Vec::new();

    loop {
        let mut maxidx = 0;
        cell.clear();
        legs.clear();

        for i in 0..16 {
            let rx: f32 = world.rnd() * 1.5;
            let ry: f32 = world.rnd() * 0.75 + rx * 0.5;
            cell.push(mgfw::ecs::Position { x: rx + 0.75, y: ry + 0.75 });
            let idx = grid[i] as usize;
            if idx > maxidx { maxidx = idx; }
            ord[idx] = i as u8;
        }

        let skew: f32 = 0.2;
        let mx: f32 = 3.0;
        let my: f32 = 2.0;   

        for i in 0..maxidx {
            let idx = ord[i + 1] as usize;

            let bx = idx % 4;
            let by = (idx - (idx % 4)) / 4;
            let fy = by as f32 * my;
            let fx = bx as f32 * mx + fy * skew;
            
            let va = mgfw::ecs::Position { x: fx + cell[idx].x, y: fy + cell[idx].y };

            let idx = ord[(i + 1) % maxidx + 1] as usize;
            let bx = idx % 4;
            let by = (idx - (idx % 4)) / 4;
            let fy = by as f32 * my;
            let fx = bx as f32 * mx + fy * skew;
            
            let vb = mgfw::ecs::Position { x: fx + cell[idx].x, y: fy + cell[idx].y };

            legs.push((va, vb));
        }

        let mut mindot = 0.0;
        // track validation
        for i in 0..maxidx {
            let p0 = legs[i].0;
            let p1 = legs[i].1;
            let p2 = legs[(i + 1) % maxidx].1;

            let va = mgfw::ecs::Position { x: p1.x - p0.x, y: p1.y - p0.y };
            let vb = mgfw::ecs::Position { x: p2.x - p1.x, y: p2.y - p1.y };

            let maga = (va.x * va.x + va.y * va.y).sqrt();
            let magb = (vb.x * vb.x + vb.y * vb.y).sqrt();

            let dot = (va.x * vb.x + va.y * vb.y) / (maga * magb);
            if dot < mindot {
                mindot = dot;
            }
        }

        if -0.77 > mindot {
            println!("Track generation failed verification, regenerating.");
        } else {
            break;
        }
    }

    for i in 0..legs.len() {
        let va = legs[i].0;
        let vb = legs[i].1;

        let mut uab = mgfw::ecs::Position { x: vb.x - va.x, y: vb.y - va.y };
        let mag = (uab.x * uab.x + uab.y * uab.y).sqrt();
        uab.x /= mag;
        uab.y /= mag;

        let ss = 0.5;
        let pa = mgfw::ecs::Position { x: va.x + uab.x * ss, y: va.y + uab.y * ss };
        let pb = mgfw::ecs::Position { x: va.x + uab.x * (mag - ss), y: va.y + uab.y * (mag - ss) };

        let subc = (((pb.x - pa.x) * (pb.x - pa.x) + (pb.y - pa.y) * (pb.y - pa.y)).sqrt() / 0.1).ceil() as usize;

        let ca = pb;
        let cb = vb;

        let ii = (i + 1) % legs.len();
        let va = legs[ii].0;
        let vb = legs[ii].1;

        let uab0 = uab;
        let mut uab = mgfw::ecs::Position { x: vb.x - va.x, y: vb.y - va.y };
        let mag = (uab.x * uab.x + uab.y * uab.y).sqrt();
        uab.x /= mag;
        uab.y /= mag;

        let mut cross = 1.0;
        if 0.0 > uab0.x * uab.y - uab0.y * uab.x { cross = -1.0; }
        let dot = (1.0 + uab0.x * uab.x + uab0.y * uab.y) * cross;

        for f in 0..subc {
            
            let r0 = f as f32 / (subc as f32);
            let r1 = (f + 1) as f32 / (subc as f32);

            let p0 = mgfw::ecs::Position { x: pa.x + (pb.x - pa.x) * r0, y: pa.y + (pb.y - pa.y) * r0 };
            let p1 = mgfw::ecs::Position { x: pa.x + (pb.x - pa.x) * r1, y: pa.y + (pb.y - pa.y) * r1 };

            segs.push((p0, p1));
            sign.push(dot);
        }        

        let pa = mgfw::ecs::Position { x: va.x + uab.x * ss, y: va.y + uab.y * ss };
        let cc = pa;

        corners.push((ca, cb, cc));
    
        let va = corners[i].0;
        let vb = corners[i].1;
        let vc = corners[i].2;

        let vab = mgfw::ecs::Position { x: vb.x - va.x, y: vb.y - va.y };
        let vbc = mgfw::ecs::Position { x: vc.x - vb.x, y: vc.y - vb.y };

        for j in 0..20 {
            let w0 = j as f32 / 20.0;
            let w1 = (j + 1) as f32 / 20.0;

            let p0 = mgfw::ecs::Position { x: vab.x * w0, y: vab.y * w0 };
            let p1 = mgfw::ecs::Position { x: vab.x + vbc.x * w0, y: vab.y + vbc.y * w0 };
            let pc = mgfw::ecs::Position { x: va.x + p0.x + (p1.x - p0.x) * w0, y: va.y + p0.y + (p1.y - p0.y) * w0 };
            let v0 = pc;

            let p0 = mgfw::ecs::Position { x: vab.x * w1, y: vab.y * w1 };
            let p1 = mgfw::ecs::Position { x: vab.x + vbc.x * w1, y: vab.y + vbc.y * w1 };
            let pc = mgfw::ecs::Position { x: va.x + p0.x + (p1.x - p0.x) * w1, y: va.y + p0.y + (p1.y - p0.y) * w1 };
            let v1 = pc;

            segs.push((v0, v1));//, 1.0 + (j as f32 / 20.0)));
            sign.push(0.0);
        }
    }

    track.num_quads = 0;

    //println!("segs len {}", segs.len());
    for i in 0..segs.len() {

        let pre = segs[(i + segs.len() - 1) % segs.len()];
        let post = segs[(i + 1) % segs.len()];
        let cur = segs[i];

        let npre = mgfw::ecs::Position { x: -(pre.1.y - pre.0.y), y: pre.1.x - pre.0.x };
        let ncur = mgfw::ecs::Position { x: -(cur.1.y - cur.0.y), y: cur.1.x - cur.0.x };
        let npost = mgfw::ecs::Position { x: -(post.1.y - post.0.y), y: post.1.x - post.0.x };

        let mut n0 = mgfw::ecs::Position { x: (npre.x + ncur.x) * 0.5, y: (npre.y + ncur.y) * 0.5 };
        let mut n1 = mgfw::ecs::Position { x: (npost.x + ncur.x) * 0.5, y: (npost.y + ncur.y) * 0.5 };

        let w: f32 = 20.0;
        
        let mag0 = (n0.x * n0.x + n0.y * n0.y).sqrt() * w;
        n0.x /= mag0;
        n0.y /= mag0;

        let mag1 = (n1.x * n1.x + n1.y * n1.y).sqrt() * w;
        n1.x /= mag1;
        n1.y /= mag1;

        let p0 = Point { x: post.0.x + n1.x, y: post.0.y + n1.y };
        let p1 = Point { x: post.0.x - n1.x, y: post.0.y - n1.y };
        let p2 = Point { x: cur.0.x - n0.x, y: cur.0.y - n0.y };
        let p3 = Point { x: cur.0.x + n0.x, y: cur.0.y + n0.y };

        let q = Quad { pnts: [p0, p1, p2, p3], sign: sign[i] };


        let mut w = Point {x: 0.0, y: 0.0 };
        for j in 0..4 {
            w.x += q.pnts[j].x;
            w.y += q.pnts[j].y;
        }

        let deltal = Point { x: (q.pnts[1].x + q.pnts[2].x) * 0.5, y: (q.pnts[1].y + q.pnts[2].y) * 0.5 };
        let deltar = Point { x: (q.pnts[0].x + q.pnts[3].x) * 0.5, y: (q.pnts[0].y + q.pnts[3].y) * 0.5 };
        
        let mut dx = 0.0;
        let mut dy = 0.0;

        if 1.0e-6 < q.sign {
            dx = (deltal.x - deltar.x) * 0.4;
            dy = (deltal.y - deltar.y) * 0.4;
        } else if -1.0e-6 > q.sign {
            dx = (deltar.x - deltal.x) * 0.4;
            dy = (deltar.y - deltal.y) * 0.4;
        }

        track.waypoints[i] = Point { x: w.x / 4.0 + dx, y: w.y / 4.0 + dy };

        track.quads[track.num_quads] = q;
        track.num_quads += 1;
    }

    heap.track_ref.push(track);
}

pub fn get_track_geometry(track: &Track, p: &mut Vec<mgfw::ecs::Position>, c: &mut Vec<mgfw::ecs::Color>, center: &mut Point) {

    p.clear();
    c.clear();

    center.x = 0.0;
    center.y = 0.0;

    for i in 0..track.num_quads {
        let q = track.quads[i];
        for j in 0..4 {
            center.x += q.pnts[j].x;
            center.y += q.pnts[j].y;
        }
    }
    center.x /= (track.num_quads * 4) as f32;
    center.y /= (track.num_quads * 4) as f32;

    for i in 0..track.num_quads {
        let q = track.quads[i];

        for j in 0..4 {
            p.push(mgfw::ecs::Position { x: q.pnts[j].x, y: q.pnts[j].y });
            p.push(mgfw::ecs::Position { x: q.pnts[(j + 1) % 4].x, y: q.pnts[(j + 1) % 4].y });
        }

        c.push(mgfw::ecs::Color { r: 0.4, g: 0.4, b: 0.5, a: 1.0 });
        c.push(mgfw::ecs::Color { r: 0.4, g: 0.4, b: 0.5, a: 1.0 });
        c.push(mgfw::ecs::Color { r: 0.8, g: 0.8, b: 1.0, a: 1.0 });
        c.push(mgfw::ecs::Color { r: 0.4, g: 0.4, b: 0.5, a: 1.0 });
        c.push(mgfw::ecs::Color { r: 0.4, g: 0.4, b: 0.5, a: 1.0 });
        c.push(mgfw::ecs::Color { r: 0.4, g: 0.4, b: 0.5, a: 1.0 });
        c.push(mgfw::ecs::Color { r: 0.4, g: 0.4, b: 0.5, a: 1.0 });
        c.push(mgfw::ecs::Color { r: 0.8, g: 0.8, b: 1.0, a: 1.0 });
        
        let r = 0.005;
        for j in 0..8 {
            
            let cc = q.sign.abs();

            let ratio = cc * 0.5;
            let colorb = mgfw::ecs::Color { r: 0.1, g: 0.9, b: 0.4, a: 1.0 };
            let colora = mgfw::ecs::Color { r: 0.9, g: 0.1, b: 0.4, a: 1.0 };
            let color = mgfw::ecs::Color { r: colora.r + (colorb.r - colora.r) * ratio, g: colora.g + (colorb.g - colora.g) * ratio, b: colora.b + (colorb.b - colora.b) * ratio, a: 1.0 };

            if 1.0e-6 < ratio {
                c.push(color);
                c.push(color);
            } else {
                c.push(mgfw::ecs::Color { r: 0.1, g: 0.1, b: 0.4, a: 1.0 });
                c.push(mgfw::ecs::Color { r: 0.1, g: 0.1, b: 0.4, a: 1.0 });
            }

            let x0 = track.waypoints[i].x + r * (mgfw::PI as f32 * 2.0 * ((j + 0) as f32 / 8.0)).cos();
            let y0 = track.waypoints[i].y + r * (mgfw::PI as f32 * 2.0 * ((j + 0) as f32 / 8.0)).sin();
            let x1 = track.waypoints[i].x + r * (mgfw::PI as f32 * 2.0 * ((j + 1) as f32 / 8.0)).cos();
            let y1 = track.waypoints[i].y + r * (mgfw::PI as f32 * 2.0 * ((j + 1) as f32 / 8.0)).sin();

            p.push(mgfw::ecs::Position {x: x0, y: y0 });
            p.push(mgfw::ecs::Position {x: x1, y: y1 });
        }
    }
}

pub fn init_race(cache: &mut game::GameData, heap: &mut game::GameDataHeap, world: &mut mgfw::ecs::World, track_idx: usize) {

    cache.track_data.dead = false;
    cache.track_data.race_over = false;
    cache.track_data.race_over_timer = 0.0;
    cache.track_data.move_up = false;
    cache.track_data.move_down = false;
    cache.track_data.move_left = false;
    cache.track_data.move_right = false;
    cache.track_data.countdown = 4.0;
    cache.track_data.ui_timer = 0.0;
    cache.track_data.cur_track = track_idx;

    world.clear();
    world.parse_world("assets/world.dat");

    world.entity_set_visibility(3, true);
    world.entity_set_visibility(4, true);
    world.entity_set_visibility(5, true);
    world.entity_set_visibility(6, true);

    world.entity_set_text(2, format!("Track: {}", match track_idx {
        0 => "Alpha",
        1 => "Gamma",
        2 => "Delta",
        3 => "Theta",
        4 => "Lambda",
        5 => "Omicron",
        6 => "Rho",
        7 => "Sigma",
        8 => "Psi",
        9 => "Omega",
        _ => "INVALID",
    }));
    
    cache.track_data.track = heap.track_ref[track_idx].clone();
   
    let mut p: Vec<mgfw::ecs::Position> = Vec::new();
    let mut c: Vec<mgfw::ecs::Color> = Vec::new();

    let mut center = Point { x: 0.0, y: 0.0 };

    get_track_geometry(&cache.track_data.track, &mut p, &mut c, &mut center);

    let e = world.new_entity();
    cache.track_data.track_ent = e;


    let q = cache.track_data.track.quads[0];
    let deltal = Point { x: (q.pnts[1].x + q.pnts[2].x) * 0.5, y: (q.pnts[1].y + q.pnts[2].y) * 0.5 };
    let deltar = Point { x: (q.pnts[0].x + q.pnts[3].x) * 0.5, y: (q.pnts[0].y + q.pnts[3].y) * 0.5 };
    
    let dx = (deltal.x - deltar.x) * 0.5 * 0.3;
    let dy = (deltal.y - deltar.y) * 0.5 * 0.3;

    let basex = (cache.track_data.track.quads[0].pnts[0].x + cache.track_data.track.quads[0].pnts[1].x + cache.track_data.track.quads[0].pnts[2].x + cache.track_data.track.quads[0].pnts[3].x) / 4.0;
    let basey = (cache.track_data.track.quads[0].pnts[0].y + cache.track_data.track.quads[0].pnts[1].y + cache.track_data.track.quads[0].pnts[2].y + cache.track_data.track.quads[0].pnts[3].y) / 4.0;

    let endpoint = Point { x: q.pnts[0].x - q.pnts[3].x, y: q.pnts[0].y - q.pnts[3].y };

    cache.track_data.player.position.x = basex - dx * 1.0;
    cache.track_data.player.position.y = basey - dy * 1.0;

    cache.track_data.player.angle = mgfw::PI as f32 * 0.5 + (endpoint.y).atan2(endpoint.x);
    cache.track_data.camera = cache.track_data.player.position;

    cache.track_data.scale = 1.0;

    world.entity_set_position_xy(e, -cache.track_data.camera.x, -cache.track_data.camera.y);
    world.entity_set_scale_xy(e, cache.track_data.scale, cache.track_data.scale);
    world.entity_set_line_buffer(e, &p, &c);
    world.entity_set_visibility(e, true);
    world.entity_set_projection(e, mgfw::ecs::PROJECTION_MODE_PERSPECTIVE);

    let e = world.new_entity();
    cache.track_data.minimap_ent = e;
    cache.track_data.minimap_scale = 16.0;

    world.entity_set_position_xy(e, 0.0, 12.0);
    world.entity_set_scale_xy(e, cache.track_data.minimap_scale, cache.track_data.minimap_scale);
    world.entity_set_line_buffer(e, &p, &c);
    world.entity_set_visibility(e, true);


    let e = world.new_entity();
    cache.track_data.player.track_ent = e;

    p.clear();
    c.clear();

    let r = 0.005;
    
    let pr = mgfw::PI as f32 * -0.5;
    let x0 = r * (pr + mgfw::PI as f32 * 2.0 * ((0 + 0) as f32 / 5.0)).cos();
    let y0 = r * (pr + mgfw::PI as f32 * 2.0 * ((0 + 0) as f32 / 5.0)).sin();
    let x1 = r * (pr + mgfw::PI as f32 * 2.0 * ((0 + 2) as f32 / 5.0)).cos();
    let y1 = r * (pr + mgfw::PI as f32 * 2.0 * ((0 + 2) as f32 / 5.0)).sin();
    let x2 = r * (pr + mgfw::PI as f32 * 2.0 * ((0 + 3) as f32 / 5.0)).cos();
    let y2 = r * (pr + mgfw::PI as f32 * 2.0 * ((0 + 3) as f32 / 5.0)).sin();
    let xc = r * (pr + mgfw::PI as f32 * 2.0 * ((0.0 + 2.5) as f32 / 5.0)).cos() * 0.5;
    let yc = r * (pr + mgfw::PI as f32 * 2.0 * ((0.0 + 2.5) as f32 / 5.0)).sin() * 0.5;

    p.push(mgfw::ecs::Position {x: x0, y: y0 });
    p.push(mgfw::ecs::Position {x: x1, y: y1 });
    p.push(mgfw::ecs::Position {x: x1, y: y1 });
    p.push(mgfw::ecs::Position {x: xc, y: yc });
    p.push(mgfw::ecs::Position {x: xc, y: yc });
    p.push(mgfw::ecs::Position {x: x2, y: y2 });
    p.push(mgfw::ecs::Position {x: x2, y: y2 });
    p.push(mgfw::ecs::Position {x: x0, y: y0 });

    c.push(mgfw::ecs::Color { r: 1.0, g: 1.0, b: 0.0, a: 1.0 });
    c.push(mgfw::ecs::Color { r: 1.0, g: 1.0, b: 0.0, a: 1.0 });
    c.push(mgfw::ecs::Color { r: 1.0, g: 1.0, b: 0.0, a: 1.0 });
    c.push(mgfw::ecs::Color { r: 1.0, g: 1.0, b: 0.0, a: 1.0 });
    c.push(mgfw::ecs::Color { r: 1.0, g: 1.0, b: 0.0, a: 1.0 });
    c.push(mgfw::ecs::Color { r: 1.0, g: 1.0, b: 0.0, a: 1.0 });
    c.push(mgfw::ecs::Color { r: 1.0, g: 1.0, b: 0.0, a: 1.0 });
    c.push(mgfw::ecs::Color { r: 1.0, g: 1.0, b: 0.0, a: 1.0 });

    world.entity_set_position_xy(e, 0.0, 0.0);
    world.entity_set_scale_xy(e, cache.track_data.scale, cache.track_data.scale);
    world.entity_set_line_buffer(e, &p, &c);
    world.entity_set_visibility(e, true);
    world.entity_set_projection(e, mgfw::ecs::PROJECTION_MODE_PERSPECTIVE);

    let e = world.new_entity();
    cache.track_data.player.minimap_ent = e;

    world.entity_set_position_xy(e, cache.track_data.camera.x * cache.track_data.minimap_scale, cache.track_data.camera.y * cache.track_data.minimap_scale + 12.0);
    world.entity_set_scale_xy(e, cache.track_data.minimap_scale * 64.0, cache.track_data.minimap_scale * 64.0);
    world.entity_set_line_buffer(e, &p, &c);
    world.entity_set_visibility(e, true);

    for n in 0..MAX_NPCS {
        c.clear();

        let color = match n {
            0 => mgfw::ecs::Color { r: 1.0, g: 0.5, b: 0.0, a: 1.0 },
            1 => mgfw::ecs::Color { r: 0.0, g: 0.5, b: 1.0, a: 1.0 },
            2 => mgfw::ecs::Color { r: 0.0, g: 1.0, b: 0.0, a: 1.0 },
            _ => mgfw::ecs::Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 },
        };

        for _i in 0..8 {
            c.push(color);
        }
        
        cache.track_data.npcs[n].angle = cache.track_data.player.angle;

        let e = world.new_entity();
        cache.track_data.npcs[n].track_ent = e;
        world.entity_set_position_xy(e, 0.0, 0.0);
        world.entity_set_scale_xy(e, cache.track_data.scale, cache.track_data.scale);
        world.entity_set_line_buffer(e, &p, &c);
        world.entity_set_visibility(e, true);
        world.entity_set_projection(e, mgfw::ecs::PROJECTION_MODE_PERSPECTIVE);

        
        let e = world.new_entity();
        cache.track_data.npcs[n].minimap_ent = e;
        world.entity_set_position_xy(e, cache.track_data.camera.x * cache.track_data.minimap_scale, cache.track_data.camera.y * cache.track_data.minimap_scale);
        world.entity_set_scale_xy(e, cache.track_data.minimap_scale * 64.0, cache.track_data.minimap_scale * 64.0);
        world.entity_set_line_buffer(e, &p, &c);
        world.entity_set_visibility(e, true);

        let pos = match n {
            0 => Point { x: basex - dx * 2.0, y: basey - dy * 2.0 },
            1 => Point { x: basex + dx * 1.0, y: basey + dy * 1.0 },
            2 => Point { x: basex + dx * 2.0, y: basey + dy * 2.0 },
            _ => Point { x: basex - dx * 0.0, y: basey - dy * 0.0 },
        };

        cache.track_data.npcs[n].position = pos;
        cache.track_data.npcs[n].velocity = Point { x: 0.0, y: 0.0 };
        cache.track_data.npcs[n].next_waypoint = 1;
        cache.track_data.npcs[n].hint_left = false;
        cache.track_data.npcs[n].hint_right = false;
        cache.track_data.npcs[n].hint_up = 0.0;
        cache.track_data.npcs[n].skill = (1.05 - 0.25 * world.rnd()) * 1.07_f32.powf(track_idx as f32);
        cache.track_data.npcs[n].drive = 1.0;
        cache.track_data.npcs[n].start_time = -0.5 * world.rnd() as f64;
        cache.track_data.npcs[n].lap = 1;
        cache.track_data.npcs[n].lap_timer = 0.0;
        cache.track_data.npcs[n].place = MAX_NPCS + 1;
        cache.track_data.npcs[n].damage = 0.0;
        cache.track_data.npcs[n].quadrant = 0;

        let p = Point { x: cache.track_data.npcs[n].position.x - cache.track_data.camera.x, y: cache.track_data.npcs[n].position.y - cache.track_data.camera.y };
        let r = Point { x: p.x * (-cache.track_data.player.angle).cos() - p.y * (-cache.track_data.player.angle).sin(), y: p.x * (-cache.track_data.player.angle).sin() + p.y * (-cache.track_data.player.angle).cos() };
        world.entity_set_position_xy(cache.track_data.npcs[n].track_ent, r.x, r.y);
    }

    cache.track_data.player.place = MAX_NPCS + 1;
    cache.track_data.player.next_waypoint = 1;
    cache.track_data.player.lap = 1;
    cache.track_data.player.lap_timer = 0.0;
    cache.track_data.player.damage = 0.0;
    cache.track_data.damage_pulse = 0.0;
    cache.track_data.player.velocity = Point { x: 0.0, y: 0.0 };
    cache.track_data.player.best_timer = 0.0;
    cache.track_data.player.quadrant = 0;
        

    p.clear();
    c.clear();

    //

    let e = world.new_entity();
    cache.track_data.damage_ent = e;

    world.entity_set_position_xy(e, 320.0, 384.0);
    
    p.push(mgfw::ecs::Position { x: -1.0, y: 0.0 });
    p.push(mgfw::ecs::Position { x: 1.0, y: 0.0 });
    p.push(mgfw::ecs::Position { x: -1.0, y: -1.0 });

    p.push(mgfw::ecs::Position { x: -1.0, y: -1.0 });
    p.push(mgfw::ecs::Position { x: 1.0, y: 0.0 });
    p.push(mgfw::ecs::Position { x: 1.0, y: -1.0 });

    c.push(mgfw::ecs::Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
    c.push(mgfw::ecs::Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
    c.push(mgfw::ecs::Color { r: 1.0, g: 0.0, b: 0.0, a: 0.0 });

    c.push(mgfw::ecs::Color { r: 1.0, g: 0.0, b: 0.0, a: 0.0 });
    c.push(mgfw::ecs::Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
    c.push(mgfw::ecs::Color { r: 1.0, g: 0.0, b: 0.0, a: 0.0 });

    world.entity_set_triangle_buffer(e, &p, &c);
    world.entity_set_visibility(e, true);
    world.entity_set_scale_xy(e, 320.0, 0.0);
    world.entity_set_alpha(e, 0.0);

    cache.track_data.player.quadrant = 0;

    //

    let e = world.new_entity();
    cache.track_data.destroyed_ent = e;

    world.entity_set_text(e, String::from("\\\\ GAME OVER //"));
    world.entity_set_position_xy(e, 320.0 - world.text_get_width(e) as f32 * 2.0, 240.0);
    world.entity_set_scale_xy(e, 4.0, 3.0);
    world.entity_set_color_rgba(e, 0.1, 0.1, 0.1, 1.0);
    world.entity_set_visibility(e, false);

    let e = world.new_entity();
    world.entity_set_text(e, String::from("// SHIP DESTROYED \\\\"));
    world.entity_set_position_xy(e, 320.0 - world.text_get_width(e) as f32 * 2.0, 300.0);
    world.entity_set_scale_xy(e, 4.0, 3.0);
    world.entity_set_color_rgba(e, 0.1, 0.1, 0.1, 1.0);
    world.entity_set_visibility(e, false);

    //

    let e = world.new_entity();
    cache.track_data.race_over_ent = e;
    world.entity_set_billboard(e, String::from("assets/square-b.png"));
    world.entity_set_position_xy(e, 320.0, 192.0);
    world.entity_set_scale_xy(e, 640.0, 384.0);
    world.entity_set_visibility(e, false);

    let e = world.new_entity();
    world.entity_set_text(e, String::from("\\\\ RACE OVER //"));
    world.entity_set_position_xy(e, 320.0 - world.text_get_width(e) as f32 * 0.5 * 3.0, 100.0);
    world.entity_set_scale_xy(e, 3.0, 2.0);
    world.entity_set_visibility(e, false);

}


// this gets called by MGFW with input events
#[rustfmt::skip]
pub fn event(
    cache: &mut game::GameData, heap: &mut game::GameDataHeap, world: &mut mgfw::ecs::World, event_id: u8) -> bool {

    match event_id {
        mgfw::EVENT_INPUT_KEYBOARD_RELEASED_ESCAPE => {
            if cache.track_data.dead {
                menu::build_menu_main(cache, world);
                return true;
            } else {
                menu::build_menu_track(cache, heap, world);
                return true;
            }
        },
        mgfw::EVENT_INPUT_KEYBOARD_RELEASED_SPACE => {
            if cache.track_data.dead {
                menu::build_menu_main(cache, world);
                return true;
            }
        },
        mgfw::EVENT_INPUT_KEYBOARD_RELEASED_UP => {
            cache.track_data.move_up = false;
        }
        mgfw::EVENT_INPUT_KEYBOARD_RELEASED_DOWN => {
            cache.track_data.move_down = false;
        }
        mgfw::EVENT_INPUT_KEYBOARD_RELEASED_LEFT => {
            cache.track_data.move_left = false;
        }
        mgfw::EVENT_INPUT_KEYBOARD_RELEASED_RIGHT => {
            cache.track_data.move_right = false;    
        }
        mgfw::EVENT_INPUT_KEYBOARD_PRESSED_UP => {
            cache.track_data.move_up = true;
        }
        mgfw::EVENT_INPUT_KEYBOARD_PRESSED_DOWN => {
            cache.track_data.move_down = true;
        }
        mgfw::EVENT_INPUT_KEYBOARD_PRESSED_LEFT => {
            cache.track_data.move_left = true;
        }
        mgfw::EVENT_INPUT_KEYBOARD_PRESSED_RIGHT => {
            cache.track_data.move_right = true;    
        }
        _ => (),
    }

    true
}


#[rustfmt::skip]
pub fn update(cache: &mut game::GameData, heap: &mut game::GameDataHeap, world: &mut mgfw::ecs::World) -> bool {
    let mut expect_blown = false;
    
    let dt = 1.0 / 1200.0;

    cache.track_data.ui_timer -= dt;
    if 0.0 > cache.track_data.ui_timer {
        cache.track_data.ui_timer = 0.1;
        let minutes = (cache.track_data.player.lap_timer / 60.0).floor();
        let mut mpad = "0";
        if minutes >= 10.0 { mpad = ""; }
        let seconds = (cache.track_data.player.lap_timer - minutes * 60.0).floor();
        let mut spad = "0";
        if seconds >= 10.0 { spad = ""; }
        world.entity_set_text(12, format!("Lap Time: {}{}:{}{}", mpad, minutes, spad, seconds));
        world.entity_set_position_xy(12, 638.0 - world.text_get_width(12) as f32, 1.0);

        let minutes = (cache.track_data.player.best_timer / 60.0).floor();
        let mut mpad = "0";
        if minutes >= 10.0 { mpad = ""; }
        let seconds = (cache.track_data.player.best_timer - minutes * 60.0).floor();
        let mut spad = "0";
        if seconds >= 10.0 { spad = ""; }
        world.entity_set_text(13, format!("Best Lap: {}{}:{}{}", mpad, minutes, spad, seconds));
        world.entity_set_position_xy(13, 638.0 - world.text_get_width(13) as f32, 20.0);

        if -0.99 > cache.track_data.countdown {
            if 10 > cache.track_data.player.lap {
                world.entity_set_text(11, format!("Lap: {}", cache.track_data.player.lap));
                world.entity_set_position_xy(17, 320.0 + world.text_get_width(11) as f32 * 0.5 * 3.0, 3.0);
                world.entity_set_visibility(17, true);
            } else {
                world.entity_set_text(11, format!("FINAL LAP"));
                world.entity_set_visibility(17, false);
            }
            world.entity_set_position_xy(11, 320.0 - world.text_get_width(11) as f32 * 0.5 * 3.0, 1.0);
            world.entity_set_visibility(11, true);

            world.entity_set_text(15, format!("Place: {}", cache.track_data.player.place));
            world.entity_set_position_xy(15, 320.0 - world.text_get_width(15) as f32 * 0.5, 30.0);
            world.entity_set_visibility(15, true);
        }

        world.entity_set_text(14, format!("Speed: {} km/h", ((cache.track_data.player.velocity.x * cache.track_data.player.velocity.x + cache.track_data.player.velocity.y * cache.track_data.player.velocity.y).sqrt() * 500.0).ceil()));
        world.entity_set_position_xy(14, 638.0 - world.text_get_width(14) as f32, 40.0);

        let mut dmg = (cache.track_data.player.damage * 100.0).floor() as i32;
        if 100 < dmg { dmg = 100; }
        world.entity_set_text(16, format!("Damage: {} %", dmg));
        world.entity_set_position_xy(16, 638.0 - world.text_get_width(16) as f32, 60.0);
        expect_blown = true;
    }

    cache.track_data.damage_pulse += dt * cache.track_data.player.damage as f64 * 10.0;
    world.entity_set_scale_xy(cache.track_data.damage_ent, 320.0, 384.0 * cache.track_data.player.damage * 1.5);
    world.entity_set_alpha(cache.track_data.damage_ent, cache.track_data.player.damage * (0.8 + 0.2 * cache.track_data.damage_pulse.sin()) as f32);

    if -1.0 < cache.track_data.countdown {
        cache.track_data.countdown -= dt;
        if cache.track_data.countdown < 3.0 {
            world.entity_set_visibility(7, true);
        }
        if cache.track_data.countdown < 2.0 {
            world.entity_set_visibility(8, true);
        }
        if cache.track_data.countdown < 1.0 {
            world.entity_set_visibility(9, true);
        }
        if cache.track_data.countdown < 0.0 {
            world.entity_set_visibility(10, true);
        }
    } else {
        for i in 3..11 {
            world.entity_set_visibility(i, false);
        }
    }

    if 0.0 < cache.track_data.countdown {
        cache.track_data.move_up = false;
        cache.track_data.move_left = false;
        cache.track_data.move_right = false;
    }

    // player
    let precamera = cache.track_data.player.position;

    let d: f32 = 0.0005;
    if cache.track_data.move_up && cache.track_data.player.damage < 1.0 {
        let mag = 1.0 * 1.08_f32.powf((cache.track_data.player.thrust - 1) as f32);
        let v = Point { x: 0.0, y: -mag };

        let r = Point { x: v.x * (cache.track_data.player.angle).cos() - v.y * (cache.track_data.player.angle).sin(), y: v.y * (cache.track_data.player.angle).cos() + v.x * (cache.track_data.player.angle).sin() };

        cache.track_data.player.velocity.x += r.x * d;
        cache.track_data.player.velocity.y += r.y * d;
    }
    
    let da = 0.001;
    let mag = 1.0 * 1.08_f32.powf((cache.track_data.player.steering - 1) as f32);
    if cache.track_data.move_left && cache.track_data.player.damage < 1.0 { cache.track_data.player.angle -= da * mag; }//println!("{}", cache.track_data.angle); }
    if cache.track_data.move_right && cache.track_data.player.damage < 1.0 { cache.track_data.player.angle += da * mag; }//println!("{}", cache.track_data.angle); }
    
    cache.track_data.player.velocity.x -= cache.track_data.player.velocity.x * dt as f32;
    cache.track_data.player.velocity.y -= cache.track_data.player.velocity.y * dt as f32;

    cache.track_data.player.position.x += cache.track_data.player.velocity.x * dt as f32;
    cache.track_data.player.position.y += cache.track_data.player.velocity.y * dt as f32;

    //cache.track_data.velocity.x -= cache.track_data.velocity.x * dt;

    let mut p: Vec<mgfw::ecs::Position> = Vec::new();
    let mut c: Vec<mgfw::ecs::Color> = Vec::new();

    let mut curidx = cache.track_data.track.num_quads;

    let mut curwall = 4;
    let mut overlap = Point {x: 0.0, y: 0.0};
    let mut overlapdot = 0.0;

    for k in 0..cache.track_data.track.num_quads {

        let i = (cache.track_data.player.next_waypoint + cache.track_data.track.num_quads - 3 + k) % cache.track_data.track.num_quads;

        let q = cache.track_data.track.quads[i];

        let mut cnt = 0;
        let mut wall = 4;
        let mut olap = Point {x: 0.0, y: 0.0};
        let mut odot = 0.0;

        for j in 0..4 {

            let base = q.pnts[j];
            let next = q.pnts[(j + 1) % 4];

            let v_a = Point { x: next.x - base.x, y: next.y - base.y };
            let v_n = Point { x: -v_a.y, y: v_a.x };
            let mag = (v_n.x * v_n.x + v_n.y * v_n.y).sqrt();
            let u_n = Point { x: v_n.x / mag, y: v_n.y / mag };
            let v_b = Point { x: cache.track_data.player.position.x - base.x, y: cache.track_data.player.position.y - base.y };

            if v_n.x * v_b.x + v_n.y * v_b.y < 0.0 {
                cnt += 1;
            
                if j % 2 == 1 {
                    let v_c = Point { x: cache.track_data.player.position.x - (base.x - u_n.x * 0.004), y: cache.track_data.player.position.y - (base.y - u_n.y * 0.004) };

                    let dot = u_n.x * v_c.x + u_n.y * v_c.y;
                    if dot > 0.0 {
                        wall = j;
                        olap.x = u_n.x * dot;
                        olap.y = u_n.y * dot;
                        odot = dot / (v_c.x * v_c.x + v_c.y * v_c.y).sqrt();
                    }
                }
            }
        }

        if 4 == cnt { curidx = i; curwall = wall; overlap = olap; overlapdot = odot; break; };

    }


    if curidx != cache.track_data.track.num_quads && !cache.track_data.race_over {

        let quadrant = (curidx as f32 / (cache.track_data.track.num_quads as f32 / 4.0)).floor() as usize;
        if quadrant != cache.track_data.player.quadrant {
            if (0 == cache.track_data.player.quadrant && 1 == quadrant) || 
                (1 == cache.track_data.player.quadrant && 2 == quadrant) ||
                (2 == cache.track_data.player.quadrant && 3 == quadrant) ||
                (3 == cache.track_data.player.quadrant && 0 == quadrant) {
                    cache.track_data.player.quadrant = quadrant;
                if 0 == quadrant {
                    if 1 == cache.track_data.player.lap {
                        cache.track_data.player.best_timer = cache.track_data.player.lap_timer;
                    } else {
                        if cache.track_data.player.best_timer > cache.track_data.player.lap_timer {
                            cache.track_data.player.best_timer = cache.track_data.player.lap_timer;
                        }
                    }
                    cache.track_data.player.lap_timer = 0.0;
                    cache.track_data.player.lap += 1;
                    if 11 == cache.track_data.player.lap {
                        cache.track_data.race_over = true;
                        cache.track_data.race_over_timer = 1.0;
                    }
                }
            }
        }

        let nq = cache.track_data.track.num_quads;
        let p0 = cache.track_data.player.next_waypoint + cache.track_data.player.lap * nq;
        let mut place = 1 + MAX_NPCS;
        for n in 0..MAX_NPCS {
            if cache.track_data.npcs[n].next_waypoint + cache.track_data.npcs[n].lap * nq - 1 < p0 {
                place -= 1;
            }
        }
        cache.track_data.player.place = place;

        let q = cache.track_data.track.quads[curidx];

        for j in 0..4 {
            p.push(mgfw::ecs::Position { x: q.pnts[j].x, y: q.pnts[j].y });
            p.push(mgfw::ecs::Position { x: q.pnts[(j + 1) % 4].x, y: q.pnts[(j + 1) % 4].y });

            if j == curwall {
                c.push(mgfw::ecs::Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 });
                c.push(mgfw::ecs::Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 });
            }
            else {
                c.push(mgfw::ecs::Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
                c.push(mgfw::ecs::Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
            }
        }

        for j in 0..4 {

            let base = q.pnts[j];
            let next = q.pnts[(j + 1) % 4];

            let v_a = Point { x: next.x - base.x, y: next.y - base.y };
            let v_n = Point { x: -v_a.y, y: v_a.x };
            let mag = (v_n.x * v_n.x + v_n.y * v_n.y).sqrt();
            let u_n = Point { x: v_n.x / mag, y: v_n.y / mag };

            p.push(mgfw::ecs::Position { x: base.x + v_a.x * 0.5, y: base.y + v_a.y * 0.5 });
            p.push(mgfw::ecs::Position { x: base.x + v_a.x * 0.5 + u_n.x * 0.02, y: base.y + v_a.y * 0.5 + u_n.y * 0.02 });

            if j == curwall {
                c.push(mgfw::ecs::Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 });
                c.push(mgfw::ecs::Color { r: 0.0, g: 0.0, b: 1.0, a: 1.0 });
            }
            else {
                c.push(mgfw::ecs::Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
                c.push(mgfw::ecs::Color { r: 1.0, g: 0.0, b: 0.0, a: 1.0 });
            }
            
        }

        cache.track_data.player.next_waypoint = (curidx + 1) % cache.track_data.track.num_quads;
        
    }

    let oscale: f32 = 1.5;

    if 4 != curwall {
        cache.track_data.player.position.x -= overlap.x * oscale; cache.track_data.player.position.y -= overlap.y * oscale;
        cache.track_data.player.velocity.x = (cache.track_data.player.position.x - precamera.x) * (0.99 - overlapdot * 2.0) / dt as f32;
        cache.track_data.player.velocity.y = (cache.track_data.player.position.y - precamera.y) * (0.99 - overlapdot * 2.0) / dt as f32;
        let mag = 1.0 * 1.08_f32.powf((cache.track_data.player.shield - 1) as f32);
        let dmg = (overlap.x * overlap.x + overlap.y * overlap.y).sqrt() * 1000.0;

        if !cache.track_data.race_over {
            cache.track_data.player.damage += dmg / mag;
        }

        if cache.track_data.player.damage >= 1.0 {
            cache.track_data.dead = true;
            world.entity_set_visibility(cache.track_data.destroyed_ent, true);
            world.entity_set_visibility(cache.track_data.destroyed_ent + 1, true);
        }
    }

    cache.track_data.camera = cache.track_data.player.position;

    world.entity_set_position_xy(cache.track_data.track_ent, -cache.track_data.camera.x, -cache.track_data.camera.y);

    world.entity_set_angle(cache.track_data.track_ent, cache.track_data.player.angle);
    
    world.entity_set_position_xy(cache.track_data.player.minimap_ent, cache.track_data.camera.x * cache.track_data.minimap_scale, cache.track_data.camera.y * cache.track_data.minimap_scale + 12.0);
    world.entity_set_angle(cache.track_data.player.minimap_ent, cache.track_data.player.angle);

    if 0.0 > cache.track_data.countdown {
        cache.track_data.player.lap_timer += dt;
    }

    // NPC
    for n in 0..MAX_NPCS {
        let precamera = cache.track_data.npcs[n].position;

        if cache.track_data.countdown > cache.track_data.npcs[n].start_time {
            cache.track_data.npcs[n].hint_up = 0.0;
            cache.track_data.npcs[n].hint_left = false;
            cache.track_data.npcs[n].hint_right = false;
        } else {
            cache.track_data.npcs[n].lap_timer += dt;
        }

        cache.track_data.npcs[n].drive = 1.0 + 0.1 * (cache.track_data.npcs[n].lap_timer * 0.1 + n as f64).sin() as f32;

        let mut d: f32 = 0.0005;
        d = d * cache.track_data.npcs[n].hint_up;
        
        let v = Point { x: 0.0, y: -1.0 };

        let r = Point { x: v.x * (cache.track_data.npcs[n].angle).cos() - v.y * (cache.track_data.npcs[n].angle).sin(), y: v.y * (cache.track_data.npcs[n].angle).cos() + v.x * (cache.track_data.npcs[n].angle).sin() };

        cache.track_data.npcs[n].velocity.x += r.x * d;
        cache.track_data.npcs[n].velocity.y += r.y * d;
        
        let da = 0.002 * cache.track_data.npcs[n].skill as f32;
        if cache.track_data.npcs[n].hint_left { cache.track_data.npcs[n].angle -= da; }//println!("{}", cache.track_data.angle); }
        if cache.track_data.npcs[n].hint_right { cache.track_data.npcs[n].angle += da; }//println!("{}", cache.track_data.angle); }
        
        cache.track_data.npcs[n].velocity.x -= cache.track_data.npcs[n].velocity.x * dt as f32;
        cache.track_data.npcs[n].velocity.y -= cache.track_data.npcs[n].velocity.y * dt as f32;

        cache.track_data.npcs[n].position.x += cache.track_data.npcs[n].velocity.x * dt as f32;
        cache.track_data.npcs[n].position.y += cache.track_data.npcs[n].velocity.y * dt as f32;

        //cache.track_data.velocity.x -= cache.track_data.velocity.x * dt;

        let mut curidx = cache.track_data.track.num_quads;

        let mut curwall = 4;
        let mut overlap = Point {x: 0.0, y: 0.0};
        let mut overlapdot = 0.0;

        for k in 0..cache.track_data.track.num_quads {

            let i = (cache.track_data.npcs[n].next_waypoint + cache.track_data.track.num_quads - 3 + k) % cache.track_data.track.num_quads;

            let q = cache.track_data.track.quads[i];

            let mut cnt = 0;
            let mut wall = 4;
            let mut olap = Point {x: 0.0, y: 0.0};
            let mut odot = 0.0;

            for j in 0..4 {

                let base = q.pnts[j];
                let next = q.pnts[(j + 1) % 4];

                let v_a = Point { x: next.x - base.x, y: next.y - base.y };
                let v_n = Point { x: -v_a.y, y: v_a.x };
                let mag = (v_n.x * v_n.x + v_n.y * v_n.y).sqrt();
                let u_n = Point { x: v_n.x / mag, y: v_n.y / mag };
                let v_b = Point { x: cache.track_data.npcs[n].position.x - base.x, y: cache.track_data.npcs[n].position.y - base.y };

                if v_n.x * v_b.x + v_n.y * v_b.y < 0.0 {
                    cnt += 1;
                
                    if j % 2 == 1 {
                        let v_c = Point { x: cache.track_data.npcs[n].position.x - (base.x - u_n.x * 0.005), y: cache.track_data.npcs[n].position.y - (base.y - u_n.y * 0.005) };

                        let dot = u_n.x * v_c.x + u_n.y * v_c.y;
                        if dot > 0.0 {
                            wall = j;
                            olap.x = u_n.x * dot;
                            olap.y = u_n.y * dot;
                            odot = dot / (v_c.x * v_c.x + v_c.y * v_c.y).sqrt();
                        }
                    }
                }
            }

            if 4 == cnt { curidx = i; curwall = wall; overlap = olap; overlapdot = odot; break; };

        }


        if curidx != cache.track_data.track.num_quads {

            let quadrant = (curidx as f32 / (cache.track_data.track.num_quads as f32 / 4.0)).floor() as usize;
            if quadrant != cache.track_data.npcs[n].quadrant {
                if (0 == cache.track_data.npcs[n].quadrant && 1 == quadrant) || 
                    (1 == cache.track_data.npcs[n].quadrant && 2 == quadrant) ||
                    (2 == cache.track_data.npcs[n].quadrant && 3 == quadrant) ||
                    (3 == cache.track_data.npcs[n].quadrant && 0 == quadrant) {
                    cache.track_data.npcs[n].quadrant = quadrant;
                    if 0 == quadrant {
                        if 1 == cache.track_data.npcs[n].lap {
                            cache.track_data.npcs[n].best_timer = cache.track_data.npcs[n].lap_timer;
                        } else {
                            if cache.track_data.npcs[n].best_timer > cache.track_data.npcs[n].lap_timer {
                                cache.track_data.npcs[n].best_timer = cache.track_data.npcs[n].lap_timer;
                            }
                        }
                        cache.track_data.npcs[n].lap_timer = 0.0;
                        cache.track_data.npcs[n].lap += 1;
                    }
                }
            }

            cache.track_data.npcs[n].next_waypoint = (curidx + 1) % cache.track_data.track.num_quads;
            let cidx = cache.track_data.npcs[n].next_waypoint;
            let nidx = (cidx + 1) % cache.track_data.track.num_quads;

            let v = Point { x: 0.0, y: -1.0 };

            let da = 0.0005;
            let cam_delta = Point { x: v.x * (cache.track_data.npcs[n].angle).cos() - v.y * (cache.track_data.npcs[n].angle).sin(), y: v.y * (cache.track_data.npcs[n].angle).cos() + v.x * (cache.track_data.npcs[n].angle).sin() };
            let cam_delta_left = Point { x: v.x * (cache.track_data.npcs[n].angle - da).cos() - v.y * (cache.track_data.npcs[n].angle - da).sin(), y: v.y * (cache.track_data.npcs[n].angle - da).cos() + v.x * (cache.track_data.npcs[n].angle - da).sin() };
            let cam_delta_right = Point { x: v.x * (cache.track_data.npcs[n].angle + da).cos() - v.y * (cache.track_data.npcs[n].angle + da).sin(), y: v.y * (cache.track_data.npcs[n].angle + da).cos() + v.x * (cache.track_data.npcs[n].angle + da).sin() };
            
            let mut sum_delta = Point { x: 0.0, y: 0.0 };
            let lookahead = 2 + ((cache.track_data.npcs[n].velocity.x * cache.track_data.npcs[n].velocity.x + cache.track_data.npcs[n].velocity.y * cache.track_data.npcs[n].velocity.y).sqrt() * 30.0).ceil() as usize;
            for j in 0..lookahead {
                let sidx = (nidx + j) % cache.track_data.track.num_quads;
                sum_delta.x += (cache.track_data.track.waypoints[sidx].x - cache.track_data.npcs[n].position.x) * (j as f32 / (lookahead as f32 * 1.5));
                sum_delta.y += (cache.track_data.track.waypoints[sidx].y - cache.track_data.npcs[n].position.y) * (j as f32 / (lookahead as f32 * 1.5));
            }

            let smag = (sum_delta.x * sum_delta.x + sum_delta.y * sum_delta.y).sqrt();
            sum_delta.x /= smag;
            sum_delta.y /= smag;

            let camdot = sum_delta.x as f64 * cam_delta.x as f64 + sum_delta.y as f64 * cam_delta.y as f64;
            let camdot_left = sum_delta.x as f64 * cam_delta_left.x as f64 + sum_delta.y as f64 * cam_delta_left.y as f64;
            let camdot_right = sum_delta.x as f64 * cam_delta_right.x as f64 + sum_delta.y as f64 * cam_delta_right.y as f64;
            
            let mut vdelta = Point {x: cache.track_data.npcs[n].velocity.x, y: cache.track_data.npcs[n].velocity.y };
            let vmag = (vdelta.x * vdelta.x + vdelta.y * vdelta.y).sqrt();
            if 0.1 < vmag {
                vdelta.x /= vmag;
                vdelta.y /= vmag;
            } else {
                vdelta = cam_delta;
            }

            let vdot = vdelta.x as f64 * cam_delta.x as f64 + vdelta.y as f64 * cam_delta.y as f64;

            cache.track_data.npcs[n].hint_left = false;
            cache.track_data.npcs[n].hint_right = false;
            cache.track_data.npcs[n].hint_up = (camdot * camdot * vdot * vdot * cache.track_data.npcs[n].skill as f64 * cache.track_data.npcs[n].drive as f64) as f32;
            if camdot_left > camdot { cache.track_data.npcs[n].hint_left = true; }
            if camdot_right > camdot { cache.track_data.npcs[n].hint_right = true; }

        }

        let oscale: f32 = 1.5;

        if 4 != curwall {
            cache.track_data.npcs[n].position.x -= overlap.x * oscale; cache.track_data.npcs[n].position.y -= overlap.y * oscale;
            cache.track_data.npcs[n].velocity.x = (cache.track_data.npcs[n].position.x - precamera.x) * (0.999 - overlapdot * 2.0) / dt as f32;
            cache.track_data.npcs[n].velocity.y = (cache.track_data.npcs[n].position.y - precamera.y) * (0.999 - overlapdot * 2.0) / dt as f32;
        }

        world.entity_set_position_xy(cache.track_data.npcs[n].minimap_ent, cache.track_data.npcs[n].position.x * cache.track_data.minimap_scale, cache.track_data.npcs[n].position.y * cache.track_data.minimap_scale + 12.0);
        world.entity_set_angle(cache.track_data.npcs[n].minimap_ent, cache.track_data.npcs[n].angle);

        let p = Point { x: cache.track_data.npcs[n].position.x - cache.track_data.camera.x, y: cache.track_data.npcs[n].position.y - cache.track_data.camera.y };
        let r = Point { x: p.x * (-cache.track_data.player.angle).cos() - p.y * (-cache.track_data.player.angle).sin(), y: p.x * (-cache.track_data.player.angle).sin() + p.y * (-cache.track_data.player.angle).cos() };
        world.entity_set_position_xy(cache.track_data.npcs[n].track_ent, r.x, r.y);

        if 0.0 > cache.track_data.countdown {
            cache.track_data.npcs[n].lap_timer += dt;
        }
    }

    if cache.track_data.race_over {
        world.entity_set_visibility(cache.track_data.race_over_ent, true);
        world.entity_set_alpha(cache.track_data.race_over_ent, 1.0 - cache.track_data.race_over_timer as f32);
        world.entity_set_visibility(cache.track_data.race_over_ent+1, true);
        world.entity_set_alpha(cache.track_data.race_over_ent+1, 1.0 - cache.track_data.race_over_timer as f32);
        
        cache.track_data.race_over_timer -= dt;
        if 0.0 > cache.track_data.race_over_timer {
            if 1 == cache.track_data.player.place && 9 > cache.track_data.cur_track {
                cache.track_data.track_locked[cache.track_data.cur_track + 1] = false;
            }
            menu::build_menu_results(cache, world);
        }
    }

    expect_blown
}
