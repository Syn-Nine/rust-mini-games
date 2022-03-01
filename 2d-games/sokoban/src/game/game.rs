extern crate xml;

use crate::mgfw;
use std::process::exit;
use std::fs::File;
use std::io::BufReader;
use xml::reader::{EventReader, XmlEvent};



const TILE_WALL: usize = 0;
const TILE_FLOOR: usize = 1;
const TILE_FOLDER: usize = 2;

#[derive(Default)]
pub struct GameDataHeap {
    // WARNING: Anything below this line is not in cache!
    map_data: Vec<usize>,
    files: Vec<(usize, usize)>,
    level_data: Vec<String>,
    flame_idx: Vec<usize>,
    flame_time: Vec<f32>,
}

pub struct GameData {
    pub heap: *mut GameDataHeap,
    frame: u8,
    ready: bool,
    level: usize,
    player_ent: usize,
    player_pos: (usize, usize),
    map_width: usize,
    map_height: usize,
    file_start_ent: usize,
    num_files: usize,
    move_counter: usize,
    push_counter: usize,
    overlay_ent_start: usize,
    overlay_lock: bool,
    overlay_alpha: f32,
    level_alpha: f32,
}

#[rustfmt::skip]
pub fn initialize(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    import_levels(heap);

    cache.level = 0;
    reset(cache, heap, world);
    load_level(cache, heap, world, cache.level);
    append_overlays(cache, world);
    update_ui(cache, world);
    update_overlay(cache, world);
    update_entities(cache, heap, world);
    
}

fn load_level(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World, idx: usize) {

    cache.level = idx % heap.level_data.len();

    let map = heap.level_data[cache.level].as_str();
    let mut y: usize = 0;
    let mut x: usize;

    let mut walls: Vec<(usize, usize)> = Vec::new();
    let mut floors: Vec<(usize, usize)> = Vec::new();
    let mut folders: Vec<(usize, usize)> = Vec::new();
    let mut files: Vec<(usize, usize)> = Vec::new();
    let mut player: (usize, usize) = (0, 0);

    let mut width: usize = 0;
    let mut height: usize = 0;

    for line in map.lines() {
        x = 0;
        for c in line.chars() {
            match c {
                '#' => { walls.push((x, y)); floors.push((x, y)); },
                '.' => { folders.push((x, y)); floors.push((x, y)); },
                'B' => { folders.push((x, y)); files.push((x, y)); floors.push((x, y)); },
                '*' => { folders.push((x, y)); files.push((x, y)); floors.push((x, y)); },
                '$' => { files.push((x, y)); floors.push((x, y)); }, 
                'b' => { files.push((x, y)); floors.push((x, y)); },
                '@' => { player = (x, y); floors.push((x, y)); },
                'p' => { player = (x, y); floors.push((x, y)); },
                'P' => { player = (x, y); folders.push((x, y)); },
                '+' => { player = (x, y); folders.push((x, y)); },
                ' ' => floors.push((x, y)),
                '-' => floors.push((x, y)),
                '_' => floors.push((x, y)),
                _ => (),
            }
            x += 1;
            if x > width { width = x; }
        }
        y += 1;
        height = y;
    }

    cache.map_width = width;
    cache.map_height = height;

    heap.map_data = vec![TILE_WALL; width * height];

    let s = get_scale(cache);
    let s2 = s / 2.0;

    let cx = 640.0 / 2.0 - (width as f32 - 0.5) * s2;
    let cy = 384.0 / 2.0 - height as f32 * s2;

    let mut twenty = "";
    if 20 == s.round() as i32 { twenty = "_20"; }

    for f in floors {
        let e = world.new_entity();
        world.entity_set_billboard(e, format!("assets/floor{}.png", twenty));
        if 0 == (f.0 + f.1) % 2 {
            world.entity_set_billboard(e, format!("assets/floor-alt{}.png", twenty));
        }
        world.entity_set_position_xy(e, cx + f.0 as f32 * s + s2, cy + f.1 as f32 * s + s2);
        world.entity_set_scale_xy(e, s, s);

        let idx = f.1 * width + f.0;
        heap.map_data[idx] = TILE_FLOOR;
    }

    for w in walls {
        let e = world.new_entity();
        world.entity_set_billboard(e, format!("assets/clock{}.png", twenty));
        if 0 == (w.0 + w.1) % 2 {
            world.entity_set_billboard(e, format!("assets/flame{}.png", twenty));
            heap.flame_idx.push(e);
            heap.flame_time.push(world.rnd());
        }
        world.entity_set_position_xy(e, cx + w.0 as f32 * s + s2, cy + w.1 as f32 * s + s2);
        world.entity_set_scale_xy(e, s, s);

        let idx = w.1 * width + w.0;
        heap.map_data[idx] = TILE_WALL;
    }

    for f in folders {
        let e = world.new_entity();
        world.entity_set_billboard(e, format!("assets/folder{}.png", twenty));
        world.entity_set_position_xy(e, cx + f.0 as f32 * s + s2, cy + f.1 as f32 * s + s2);
        world.entity_set_scale_xy(e, s, s);

        let idx = f.1 * width + f.0;
        heap.map_data[idx] = TILE_FOLDER;
    }

    cache.file_start_ent = 0;
    cache.num_files = files.len();
    for f in files {
        let e = world.new_entity();
        if 0 == cache.file_start_ent { cache.file_start_ent = e; }
        world.entity_set_billboard(e, format!("assets/file{}.png", twenty));
        world.entity_set_position_xy(e, cx + f.0 as f32 * s + s2, cy + f.1 as f32 * s + s2);
        world.entity_set_scale_xy(e, s, s);

        heap.files.push(f);
    }

    let e = world.new_entity();
    cache.player_ent = e;
    cache.player_pos = player;
    world.entity_set_billboard(e, format!("assets/player{}.png", twenty));
    world.entity_set_scale_xy(e, s, s);

    for i in 0..cache.player_ent+1 {
        world.entity_set_visibility(i, false);
        world.entity_set_alpha(i, 0.0);
    }
}

fn append_overlays(cache: &mut GameData, world: &mut mgfw::ecs::World) {

    let e = world.new_entity();
    cache.overlay_ent_start = e;

    world.entity_set_billboard(e, String::from("assets/square-b.png"));
    world.entity_set_visibility(e, false);
    world.entity_set_position_xy(e, 320.0, 200.0);
    world.entity_set_scale_xy(e, 640.0, 400.0);
    world.entity_set_alpha(e, 1.0);

    let e = world.new_entity();
    world.entity_set_text(e, format!("Level {}", cache.level + 1));
    world.entity_set_visibility(e, false);
    world.entity_set_position_xy(e, 320.0, 160.0);
    world.entity_set_scale_xy(e, 2.0, 2.0);
    world.entity_set_color_rgba(e, 1.0, 0.5, 0.2, 1.0);

    let e = world.new_entity();
    world.entity_set_text(e, String::from("You Win!"));
    world.entity_set_visibility(e, false);
    world.entity_set_position_xy(e, 320.0, 160.0);
    world.entity_set_scale_xy(e, 2.0, 2.0);
    world.entity_set_color_rgba(e, 1.0, 0.5, 0.2, 1.0);

    let e = world.new_entity();
    world.entity_set_text(e, String::from("Press SPACE for next level"));
    world.entity_set_visibility(e, false);
    world.entity_set_position_xy(e, 320.0, 200.0);
    world.entity_set_scale_xy(e, 1.0, 1.0);
    world.entity_set_color_rgba(e, 1.0, 0.5, 0.2, 1.0);

    for i in 0..3 {
        let idx = cache.overlay_ent_start + i + 1;
        let p = world.entity_get_position(idx);
        let w = world.text_get_width(idx);
        let s = world.entity_get_scale(idx);
        world.entity_set_position_xy(idx, p.x - w as f32 * 0.5 * s.x, p.y);
    }
}

fn import_levels(heap: &mut GameDataHeap) {

    let filename = "assets/microban.slc";
    
    println!("Importing level file: {}", filename);
    
    let file = File::open(filename).unwrap();
    let file = BufReader::new(file);

    heap.level_data.clear();

    let mut level_data = String::new();
    
    let parser = EventReader::new(file);
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                if name.local_name.eq("Level") {
                    level_data.clear();
                    /*for a in attributes {
                        match a.name.local_name.as_str() {
                            "Id" => {
                                level = a.value.parse::<usize>().unwrap();
                                level_data.clear();
                                println!("Level: {:?}", level);
                            },
                            _ => (),
                        };
                    }*/
                }
            },
            Ok(XmlEvent::EndElement { name, .. }) => {
                if name.local_name.eq("Level") {
                    //println!("Level Data: \n{}", level_data);
                    heap.level_data.push(level_data.clone());
                }
            },
            Ok(XmlEvent::Characters(ref data)) => {
                if level_data.is_empty() {
                    level_data = format!("{}", data);
                } else {
                    level_data = format!("{}\n{}", level_data, data);
                }
                //println!("chr: {}", data);
            },
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
}

fn reset(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    world.clear();
    world.parse_world("assets/world.dat");

    cache.move_counter = 0;
    cache.push_counter = 0;
    cache.overlay_lock = false;
    cache.overlay_alpha = 0.0;
    cache.level_alpha = 1.0;

    heap.files.clear();
    heap.map_data.clear();
    heap.flame_idx.clear();
    heap.flame_time.clear();
}

fn update_ui(cache: &mut GameData, world: &mut mgfw::ecs::World) {

    world.entity_set_text(0, format!("Level {}", cache.level + 1));
    world.entity_set_text(1, format!("Moves: {}", cache.move_counter));
    world.entity_set_text(2, format!("Pushes: {}", cache.push_counter));
    
    let w = world.text_get_width(1) as f32;
    world.entity_set_position_xy(1, (320.0 - w * 0.5).round(), 1.0);

    let w = world.text_get_width(2) as f32;
    world.entity_set_position_xy(2, 640.0 - 4.0 - w, 1.0);

    world.entity_set_position_xy(3, 4.0, 368.0);

    let w = world.text_get_width(4) as f32;
    world.entity_set_position_xy(4, (185.0 - w * 0.5).round(), 368.0);

    let w = world.text_get_width(5) as f32;
    world.entity_set_position_xy(5, (320.0 - w * 0.5).round(), 368.0);

    let w = world.text_get_width(6) as f32;
    world.entity_set_position_xy(6, (460.0 - w * 0.5).round(), 368.0);

    let w = world.text_get_width(7) as f32;
    world.entity_set_position_xy(7, 640.0 - 4.0 - w, 368.0);

}

fn update_overlay(cache: &mut GameData, world: &mut mgfw::ecs::World) {

    if 1.0e-20 < cache.level_alpha {
        world.entity_set_visibility(cache.overlay_ent_start, true);
        world.entity_set_alpha(cache.overlay_ent_start, cache.level_alpha);

        world.entity_set_visibility(cache.overlay_ent_start + 1, true);
        world.entity_set_alpha(cache.overlay_ent_start + 1, cache.level_alpha);

        return;
    } else {
        world.entity_set_visibility(cache.overlay_ent_start, false);
        world.entity_set_visibility(cache.overlay_ent_start + 1, false);
    }
    
    if 1.0e-20 < cache.overlay_alpha {
        world.entity_set_visibility(cache.overlay_ent_start, true);
        world.entity_set_alpha(cache.overlay_ent_start, cache.overlay_alpha);

        world.entity_set_visibility(cache.overlay_ent_start + 2, true);
        world.entity_set_alpha(cache.overlay_ent_start + 2, cache.overlay_alpha);

        world.entity_set_visibility(cache.overlay_ent_start + 3, true);
        world.entity_set_alpha(cache.overlay_ent_start + 3, cache.overlay_alpha);
    } else {
        world.entity_set_visibility(cache.overlay_ent_start, false);
        world.entity_set_visibility(cache.overlay_ent_start + 2, false);
        world.entity_set_visibility(cache.overlay_ent_start + 3, false);
    }
}

fn get_scale(cache: &mut GameData) -> f32 {
    
    let mut s = 32.0;

    let dx = 600.0 / cache.map_width as f32;
    let dy = 340.0 / cache.map_height as f32;

    if dx < s || dy < s { s = 20.0; }

    s
}

fn update_entities(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    let s = get_scale(cache);
    let s2 = s / 2.0;

    let cx = 640.0 / 2.0 - (cache.map_width as f32 - 0.5) * s2;
    let cy = 384.0 / 2.0 - cache.map_height as f32 * s2;

    world.entity_set_position_xy(
        cache.player_ent,
        cx + cache.player_pos.0 as f32 * s + s2,
        cy + cache.player_pos.1 as f32 * s + s2,
    );

    for i in 0..heap.files.len() {
        let f = heap.files[i];
        world.entity_set_position_xy(
            cache.file_start_ent + i,
            cx + f.0 as f32 * s + s2,
            cy + f.1 as f32 * s + s2,
        );
    }
}

// this gets called by MGFW with input events
#[rustfmt::skip]
pub fn event(
    cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World, event_id: u8) -> bool {

    let mut tgt = cache.player_pos;

    match event_id {
        mgfw::EVENT_INPUT_KEYBOARD_RELEASED_ESCAPE => exit(0),
        mgfw::EVENT_INPUT_KEYBOARD_RELEASED_SPACE => {
            if cache.overlay_lock {
                reset(cache, heap, world);
                load_level(cache, heap, world, cache.level + 1);
                append_overlays(cache, world);
                update_ui(cache, world);
                update_overlay(cache, world);
                update_entities(cache, heap, world);
                return true;
            }
        },
        mgfw::EVENT_INPUT_KEYBOARD_RELEASED_PGUP => {
            reset(cache, heap, world);
            let mut nxt: usize = heap.level_data.len() - 1;
            if 0 < cache.level { nxt = cache.level - 1; }
            load_level(cache, heap, world, nxt);
            append_overlays(cache, world);
            update_ui(cache, world);
            update_overlay(cache, world);
            update_entities(cache, heap, world);
            return true;
        },
        mgfw::EVENT_INPUT_KEYBOARD_RELEASED_PGDN => {
            reset(cache, heap, world);
            load_level(cache, heap, world, cache.level + 1);
            append_overlays(cache, world);
            update_ui(cache, world);
            update_overlay(cache, world);
            update_entities(cache, heap, world);
            return true;
        },
        mgfw::EVENT_INPUT_KEYBOARD_RELEASED_BKSPC => {
            reset(cache, heap, world);
            load_level(cache, heap, world, cache.level);
            append_overlays(cache, world);
            update_ui(cache, world);
            update_overlay(cache, world);
            update_entities(cache, heap, world);
            return true;
        },
        mgfw::EVENT_INPUT_KEYBOARD_RELEASED_UP => {
            if 0 < cache.player_pos.1 {
                tgt.1 -= 1;
            }
        }
        mgfw::EVENT_INPUT_KEYBOARD_RELEASED_DOWN => {
            tgt.1 += 1;
        }
        mgfw::EVENT_INPUT_KEYBOARD_RELEASED_LEFT => {
            if 0 < cache.player_pos.0 {
                tgt.0 -= 1;
            }
        }
        mgfw::EVENT_INPUT_KEYBOARD_RELEASED_RIGHT => {
            tgt.0 += 1;
        }
        _ => (),
    }

    if cache.overlay_lock { return true; }

    if tgt != cache.player_pos {
        try_push_or_move(cache, heap, world, tgt);
    }

    true
}

fn is_block_open(cache: &mut GameData, heap: &mut GameDataHeap, tgt: (usize, usize)) -> bool {

    if tgt.0 >= cache.map_width { return false; }
    if tgt.1 >= cache.map_height { return false; }

    let idx = tgt.1 * cache.map_width + tgt.0;
    if TILE_WALL == heap.map_data[idx] { return false; }

    true    
}

fn is_folder(cache: &mut GameData, heap: &mut GameDataHeap, tgt: (usize, usize)) -> bool {

    if tgt.0 >= cache.map_width { return false; }
    if tgt.1 >= cache.map_height { return false; }

    let idx = tgt.1 * cache.map_width + tgt.0;
    if TILE_FOLDER == heap.map_data[idx] { return true; }

    false
}

fn is_file(heap: &mut GameDataHeap, tgt: (usize, usize)) -> usize {

    for i in 0..heap.files.len() {
        let f = heap.files[i];
        if tgt.0 == f.0 && tgt.1 == f.1 {
            return i + 1;
        }
    }

    0
}

fn try_push_or_move(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World, tgt: (usize, usize)) {

    let cur = cache.player_pos;
    let delta = (tgt.0 as i32 - cur.0 as i32, tgt.1 as i32 - cur.1 as i32);
    let nxt = ((tgt.0 as i32 + delta.0) as usize, (tgt.1 as i32 + delta.1) as usize);

    let tgt_open = is_block_open(cache, heap, tgt);
    let nxt_open = is_block_open(cache, heap, nxt);
    let tgt_file = is_file(heap, tgt);
    let nxt_file = is_file(heap, nxt);
    
    
    // is tgt a wall?
    if tgt_open {

        // is tgt a file?
        if 0 < tgt_file {
            
            // make sure nxt spot is fully open
            if nxt_open && 0 == nxt_file {
                
                heap.files[tgt_file - 1] = nxt; // move file
                cache.player_pos = tgt; // move player
                cache.move_counter += 1;
                cache.push_counter += 1;
            }

        } else {
            cache.player_pos = tgt; // move player
            cache.move_counter += 1;
        }
    }

    update_entities(cache, heap, world);
    
    if check_win(cache, heap) {
        cache.overlay_lock = true;
    }

    update_ui(cache, world);
}

pub fn check_win(cache: &mut GameData, heap: &mut GameDataHeap) -> bool {

    for i in 0..heap.files.len() {
        let f = heap.files[i];
        if !is_folder(cache, heap, f) { return false; }
    }

    true
}

pub fn shutdown(_cache: &mut GameData, heap: &mut GameDataHeap) {
    // deallocate and overwrite existing memory
    *heap = GameDataHeap::default();

    // re-box and consume
    //let _temp = unsafe { Box::from_raw(cache.heap) };
}

// this gets called by MGFW at 1200hz
#[rustfmt::skip]
pub fn update(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) -> bool {
    let mut expect_blown = false;
    cache.frame = (cache.frame + 1) % 128;

    let dt = 1.0 / 1200.0;

    if !cache.ready {
        if 127 == cache.frame {
            cache.ready = true;
        }
        return false;
    }

    if cache.overlay_lock && cache.overlay_alpha < 1.0 - 1.0e-20 {

        cache.overlay_alpha += dt * 2.0;
        if 1.0 < cache.overlay_alpha {
            cache.overlay_alpha = 1.0;
            for i in 0..cache.player_ent+1 {
                world.entity_set_alpha(i, 0.0);
            }
        }
        update_overlay(cache, world);

    } else if !cache.overlay_lock && cache.overlay_alpha > 1.0e-20 {

        cache.overlay_alpha -= dt * 2.0;
        if 0.0 > cache.overlay_alpha { cache.overlay_alpha = 0.0; }
        update_overlay(cache, world);

    } else if cache.level_alpha > 1.0e-20 {

        cache.level_alpha -= dt;
        if 0.0 > cache.level_alpha {
            cache.level_alpha = 0.0;
            for i in 0..cache.player_ent+1 {
                world.entity_set_alpha(i, 1.0);
                world.entity_set_visibility(i, true);
            }
        }
        update_overlay(cache, world);
    }

    for i in 0..heap.flame_idx.len()
    {
        heap.flame_time[i] -= dt;
        if 0.0 > heap.flame_time[i] {
            heap.flame_time[i] += 0.25 + 0.5 * world.rnd();
            let s = world.entity_get_scale(heap.flame_idx[i]);
            world.entity_set_scale_xy(heap.flame_idx[i], s.x * -1.0, s.y);
        }
    }
    

    expect_blown
}
