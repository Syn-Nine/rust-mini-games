use crate::mgfw;
use rand;
use rand::prelude::*;
use std::process::exit;

#[derive(Default)]
pub struct GameDataHeap {
    // WARNING: Anything below this line is not in cache!
}

const BOARD_X: usize = 25;
const BOARD_Y: usize = 17;
const BOARD_SZ: usize = BOARD_X * BOARD_Y;
const MAX_GEMS: usize = 20;

pub struct GameData {
    pub heap: *mut GameDataHeap,
    frame: u8,
    ready: bool,
    level: usize,
    player_idx: usize,
    player_tidx: usize,
    player_ent: usize,
    centaur_idx: usize,
    centaur_tidx: usize,
    centaur_ent: usize,
    player_pos: mgfw::ecs::Position,
    centaur_pos: mgfw::ecs::Position,
    map: [u8; BOARD_SZ],
    player_timer: f64,
    centaur_timer: f64,
    player_gems: usize,
    gem_count: usize,
    gems: [usize; MAX_GEMS],
    gem_ent_start: usize,
    pickup_ent_start: usize,
    tileset: usize,
    tilemap: [u16; BOARD_SZ],
    tilemap2: [u16; BOARD_SZ],
    tileent: usize,
    tileent2: usize,
    transition_state: u8,
    transition_timer: f64,
    game_over_entity_start: usize,
    win: bool,
    gameover: bool,
    player_dir: usize,
    player_moving: bool,
    player_holding_up: bool,
    player_holding_down: bool,
    player_holding_left: bool,
    player_holding_right: bool,
    centaur_dir: usize,
    centaur_moving: bool,
}

fn world_x(idx: usize) -> usize {
    idx % BOARD_X
}

fn world_y(idx: usize) -> usize {
    (idx - (idx % BOARD_X)) / BOARD_X
}

fn world_idx(x: usize, y: usize) -> usize {
    y * BOARD_X + x
}

#[rustfmt::skip]
pub fn initialize(cache: &mut GameData, _heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    world.parse_world("assets/world.dat");

    // init tileset entity
    cache.tileset = world.new_entity();
    world.entity_set_tileset(cache.tileset, String::from("assets/tiles.png"), 128, 96, 16, 12);

    // allocate tilemap layer 1 entity
    cache.tileent = world.new_entity();
    world.entity_set_scale_xy(cache.tileent, 16.0, 12.0);
    world.entity_set_position_xy(cache.tileent, 0.0, 24.0);
    world.entity_set_visibility(cache.tileent, true);

    // allocate gem entities
    for i in 0..MAX_GEMS {
        let e = world.new_entity();
        world.entity_set_scale_xy(e, 12.0, 12.0);
        if 0 == i {
            cache.gem_ent_start = e;
        }
    }

    // allocate centaur entity
    cache.centaur_ent = world.new_entity();
    world.entity_set_billboard(cache.centaur_ent, String::from("assets/centaur.png"));
    world.entity_set_scale_xy(cache.centaur_ent, 32.0, 24.0);
    world.entity_set_visibility(cache.centaur_ent, true);

    // allocate player entity
    cache.player_ent = world.new_entity();
    world.entity_set_billboard(cache.player_ent, String::from("assets/player.png"));
    world.entity_set_scale_xy(cache.player_ent, 32.0, 24.0);
    world.entity_set_visibility(cache.player_ent, true);

    // allocate tilemap layer 2 entity
    cache.tileent2 = world.new_entity();
    world.entity_set_scale_xy(cache.tileent2, 16.0, 12.0);
    world.entity_set_position_xy(cache.tileent2, 0.0, 24.0);
    world.entity_set_visibility(cache.tileent2, true);
    
    // allocate pickup gem animations
    for i in 0..MAX_GEMS {
        let e = world.new_entity();
        world.entity_set_billboard(e, String::from("assets/gem.png"));
        world.entity_set_scale_xy(e, 12.0, 12.0);
        if 0 == i {
            cache.pickup_ent_start = e;
        }
    }

    // allocate win/lose transition entities
    // create entities for win/lose popup
    let e = world.new_entity();
    cache.game_over_entity_start = e;

    world.entity_set_billboard(e, String::from("assets/square-b.png"));
    world.entity_set_visibility(e, false);
    world.entity_set_position_xy(e, 200.0, 114.0);
    world.entity_set_scale_xy(e, 400.0, 228.0);
    world.entity_set_alpha(e, 0.8);

    let e = world.new_entity();
    world.entity_set_text(e, String::from("Level Up!"));
    world.entity_set_visibility(e, false);
    world.entity_set_position_xy(e, 200.0, 100.0);
    world.entity_set_scale_xy(e, 2.0, 2.0);
    world.entity_set_color_rgba(e, 0.5, 0.5, 1.0, 1.0);

    let e = world.new_entity();
    world.entity_set_text(e, String::from("Game Over!"));
    world.entity_set_visibility(e, false);
    world.entity_set_position_xy(e, 200.0, 90.0);
    world.entity_set_scale_xy(e, 2.0, 2.0);
    world.entity_set_color_rgba(e, 0.5, 0.5, 1.0, 1.0);

    let e = world.new_entity();
    world.entity_set_text(e, String::from("Press Space to Restart"));
    world.entity_set_visibility(e, false);
    world.entity_set_position_xy(e, 200.0, 130.0);
    world.entity_set_scale_xy(e, 1.0, 1.0);
    world.entity_set_color_rgba(e, 0.5, 0.5, 1.0, 1.0);

    let e = world.new_entity();
    world.entity_set_text(e, String::from("You Win!"));
    world.entity_set_visibility(e, false);
    world.entity_set_position_xy(e, 200.0, 100.0);
    world.entity_set_scale_xy(e, 2.0, 2.0);
    world.entity_set_color_rgba(e, 0.5, 0.5, 1.0, 1.0);

    
    // center win/gameover text
    for i in 0..4 {
        let idx = cache.game_over_entity_start + i + 1;
        let p = world.entity_get_position(idx);
        let w = world.text_get_width(idx);
        let s = world.entity_get_scale(idx);
        world.entity_set_position_xy(idx, p.x - w as f32 * 0.5 * s.x, p.y);
    }
    
    cache.level = 1;
    cache.win = false;
    cache.gameover = false;
    init_level(cache, world);
}

fn init_level(cache: &mut GameData, world: &mut mgfw::ecs::World) {
    // create map
    init_map(cache);

    // init player
    cache.player_gems = 0;
    cache.player_idx = insert_entrance(cache, world);
    cache.player_tidx = cache.player_idx;
    cache.player_pos = mgfw::ecs::Position {
        x: world_x(cache.player_idx) as f32,
        y: world_y(cache.player_idx) as f32,
    };

    // init centaur
    cache.centaur_idx = loop {
        let idx = world.rnd_range(0..(BOARD_SZ / 2));
        if 0 == cache.map[idx] {
            break idx;
        }
    };

    cache.centaur_tidx = cache.centaur_idx;
    cache.centaur_pos = mgfw::ecs::Position {
        x: world_x(cache.centaur_idx) as f32,
        y: world_y(cache.centaur_idx) as f32,
    };
    cache.centaur_dir = world.rnd_range(0..4);
    cache.centaur_timer = 0.0;
    cache.centaur_moving = false;

    cache.player_dir = 0;
    cache.player_moving = false;
    cache.player_holding_up = false;
    cache.player_holding_down = false;
    cache.player_holding_left = false;
    cache.player_holding_right = false;
    cache.player_timer = 0.0;

    // init gems
    init_gems(cache, world, MAX_GEMS - cache.level * 2 + 1);

    // update tilemap entities
    world.entity_set_tilemap(
        cache.tileent,
        cache.tileset,
        BOARD_X,
        &Vec::from(cache.tilemap),
    );
    world.entity_set_tilemap(
        cache.tileent2,
        cache.tileset,
        BOARD_X,
        &Vec::from(cache.tilemap2),
    );

    // update player/centaur entities
    update_entity_positions(cache, world);

    // update hud
    update_hud(cache, world);
}

fn insert_entrance(cache: &mut GameData, world: &mut mgfw::ecs::World) -> usize {
    let edge = get_level_edges(cache.level);

    let width = BOARD_X - edge.0 * 2;

    let idx = loop {
        let x = world.rnd_range(0..(width / 2)) + (BOARD_X - width) / 2;
        let idx = world_idx(x, BOARD_Y - edge.1 - 3);
        if 0 == cache.map[idx] {
            break idx;
        }
    };

    cache.map[idx + BOARD_X] = 0;
    cache.map[idx + BOARD_X * 2] = 0;
    cache.tilemap[idx + BOARD_X] = 2;
    cache.tilemap[idx + BOARD_X * 2] = 10;

    world.entity_set_tilemap(
        cache.tileent,
        cache.tileset,
        BOARD_X,
        &Vec::from(cache.tilemap),
    );

    idx + BOARD_X * 2
}

fn insert_exit(cache: &mut GameData, world: &mut mgfw::ecs::World) {
    let edge = get_level_edges(cache.level);

    let idx = loop {
        let x = world.rnd_range(0..(BOARD_X / 2)) + BOARD_X / 4;
        let idx = world_idx(x, 2 + edge.1);
        if 0 == cache.map[idx] {
            break idx;
        }
    };

    cache.map[idx - BOARD_X] = 0;
    cache.map[idx - BOARD_X * 2] = 0;

    cache.tilemap[idx - BOARD_X] = 10;
    cache.tilemap[idx - BOARD_X * 2] = 2;

    world.entity_set_tilemap(
        cache.tileent,
        cache.tileset,
        BOARD_X,
        &Vec::from(cache.tilemap),
    );
}

fn init_gems(cache: &mut GameData, world: &mut mgfw::ecs::World, count: usize) {
    let edge = get_level_edges(cache.level);

    for i in 0..MAX_GEMS {
        let e = cache.gem_ent_start + i;
        world.entity_set_visibility(e, false);

        let e = cache.pickup_ent_start + i;
        world.entity_set_velocity_xy(e, 0.0, 0.0);
        world.entity_set_alpha(e, 1.0);
    }

    let mut vgems: Vec<usize> = Vec::new();
    let mut hgems: Vec<usize> = Vec::new();

    for _g in 0..count {
        loop {
            let idx = world.rnd_range((BOARD_X * 2)..(BOARD_SZ - BOARD_X * 2));
            if world_y(idx) < BOARD_Y - edge.1 - 2 {
                if 0 == cache.map[idx] {
                    if world.rnd() < 0.5 {
                        vgems.push(idx);
                    } else {
                        hgems.push(idx);
                    }
                    break;
                }
            }
        }
    }

    vgems.sort();
    hgems.sort();

    cache.gem_count = 0;
    for gem in hgems {
        let e = cache.gem_ent_start + cache.gem_count;
        world.entity_set_billboard(e, String::from("assets/gem-hidden.png"));
        world.entity_set_visibility(e, true);
        let x = (16.0 * world_x(gem) as f32 + 8.0 + (world.rnd() - 0.5) * 8.0).round();
        let y = (24.0 + 12.0 * world_y(gem) as f32 + 6.0 + (world.rnd() - 0.5) * 4.0).round();
        world.entity_set_position_xy(e, x, y);
        cache.gems[cache.gem_count] = gem;
        cache.gem_count += 1;
    }

    for gem in vgems {
        let e = cache.gem_ent_start + cache.gem_count;
        world.entity_set_billboard(e, String::from("assets/gem.png"));
        world.entity_set_visibility(e, true);
        let x = (16.0 * world_x(gem) as f32 + 8.0 + (world.rnd() - 0.5) * 8.0).round();
        let y = (24.0 + 12.0 * world_y(gem) as f32 + 6.0 + (world.rnd() - 0.5) * 4.0).round();
        world.entity_set_position_xy(e, x, y);
        cache.gems[cache.gem_count] = gem;
        cache.gem_count += 1;
    }
}

fn get_level_edges(level: usize) -> (usize, usize) {
    let short = (level - (level % 3)) / 3;
    let narrow = (level - 1) - short;
    (narrow, short)
}

fn init_map(cache: &mut GameData) {
    if 10 < cache.level {
        cache.level = 10;
    }
    let edge = get_level_edges(cache.level);

    let lhs = 0 + edge.0;
    let rhs = BOARD_X - edge.0;
    let width = rhs - lhs;
    let ths = 0 + edge.1;
    let bhs = BOARD_Y - edge.1;
    let height = bhs - ths;

    cache.map = [1; BOARD_SZ];

    let mut map = [1 as u16; BOARD_SZ];
    let mut map2 = [0 as u16; BOARD_SZ];

    let mut rng = rand::thread_rng();

    for y in (ths + 2)..(bhs - 2) {
        for x in (lhs + 1)..(rhs - 1) {
            let t = 17 + 8 * ((x + y + rng.gen::<f32>().round() as usize) % 2);
            let idx = world_idx(x, y);
            map[idx] = t as u16;
            cache.map[idx] = 0;
        }
    }

    for x in (lhs + 1)..(rhs - 1) {
        let idx = world_idx(x, ths + 1);
        map[idx] = 9;
    }

    for x in 0..BOARD_X {
        let idx = world_idx(x, BOARD_Y - 1);
        map[idx] = 9;
    }

    let maze_width = (width - 1) / 2;
    let maze_width_2 = maze_width * 2;
    let maze_height = (height - 3) / 2;
    let maze_sz = maze_width * maze_height;
    let maze = gen_maze(
        maze_width,
        &vec![0; maze_sz],
        rng.gen_range(0..maze_sz) as usize,
        0.25,
        &mut rng,
    );

    draw_maze(maze_width, &maze);

    let mut temp = vec![0; maze_sz * 4];
    let c = maze_width_2;

    for y in 0..maze_height {
        for x in 0..maze_width {
            let val = maze[y * maze_width + x];
            let idx = y * 2 * maze_width_2 + x * 2;

            match val {
                0b0001 => {
                    temp[idx + 1] = 1;
                    temp[idx + c] = 1;
                    temp[idx + c + 1] = 1;
                } // print!("^"),
                0b0010 => {
                    temp[idx + 1] = 1;
                    temp[idx] = 1;
                    temp[idx + c + 1] = 1;
                } // print!("v"),
                0b0100 => {
                    temp[idx + 1] = 1;
                    temp[idx + c] = 1;
                    temp[idx + c + 1] = 1;
                } // print!("<"),
                0b1000 => {
                    temp[idx] = 1;
                    temp[idx + c] = 1;
                    temp[idx + c + 1] = 1;
                } // print!(">"),
                0b0011 => {
                    temp[idx + 1] = 1;
                    temp[idx + c + 1] = 1;
                } // print!("│"),
                0b0101 => {
                    temp[idx + 1] = 1;
                    temp[idx + c] = 1;
                    temp[idx + c + 1] = 1;
                } // print!("┘"),
                0b0110 => {
                    temp[idx + 1] = 1;
                    temp[idx + c + 1] = 1;
                } // print!("┐"),
                0b0111 => {
                    temp[idx + 1] = 1;
                    temp[idx + c + 1] = 1;
                } // print!("┤"),
                0b1001 => {
                    temp[idx + c] = 1;
                    temp[idx + c + 1] = 1;
                } // print!("└"),
                0b1010 => {
                    temp[idx + c + 1] = 1;
                } // print!("┌"),
                0b1011 => {
                    temp[idx + c + 1] = 1;
                } // print!("├"),
                0b1100 => {
                    temp[idx + c] = 1;
                    temp[idx + c + 1] = 1;
                } // print!("─"),
                0b1101 => {
                    temp[idx + c] = 1;
                    temp[idx + c + 1] = 1;
                } // print!("┴"),
                0b1110 => {
                    temp[idx + c + 1] = 1;
                } // print!("┬"),
                0b1111 => {
                    temp[idx + c + 1] = 1;
                } // print!("┼"),
                _ => (),
            }
        }
    }

    let mut counter = 0;
    for _i in 0..(13 - cache.level) {
        let idx = loop {
            let x = rng.gen_range(0..(maze_width)) + maze_width / 2;
            let y = rng.gen_range(0..(maze_height)) + maze_height / 2;
            let tidx: usize = y * maze_width_2 + x;
            if 1 == temp[tidx] {
                break tidx;
            }
            counter += 1;
            if 100 < counter {
                break 0;
            }
        };
        if 0 < idx {
            temp[idx] = 0;
        }
    }

    for y in 0..(height - 4) {
        for x in 0..(width - 2) {
            let midx = world_idx(x + 1 + lhs, y + 2 + ths);
            let midxp = world_idx(x + 1 + lhs, y + 1 + ths);
            let tidx = y * maze_width_2 + x;
            if 1 == temp[tidx] {
                map[midx] = 26;
                map2[midxp] = 18;
                cache.map[midx] = 1;
            }
        }
    }

    cache.tilemap = map;
    cache.tilemap2 = map2;
}

fn update_hud(cache: &GameData, world: &mut mgfw::ecs::World) {
    world.entity_set_text(0, format!("Level: {}", cache.level));
    world.entity_set_text(
        1,
        format!("Gems: {}/{}", cache.player_gems, cache.gem_count),
    );
    let w = world.text_get_width(1);
    world.entity_set_position_xy(1, 400.0 - w as f32 - 4.0, 5.0);
}

fn update_entity_positions(cache: &GameData, world: &mut mgfw::ecs::World) {
    //let x = world_x(cache.centaur_idx);
    //let y = world_y(cache.centaur_idx);
    let x = cache.centaur_pos.x;
    let y = cache.centaur_pos.y;
    world.entity_set_position_xy(
        cache.centaur_ent,
        16.0 * x as f32 + 8.0,
        24.0 + 12.0 * y as f32,
    );

    //let x = world_x(cache.player_idx);
    //let y = world_y(cache.player_idx);
    let x = cache.player_pos.x;
    let y = cache.player_pos.y;
    world.entity_set_position_xy(
        cache.player_ent,
        16.0 * x as f32 + 8.0,
        24.0 + 12.0 * y as f32,
    );
}

fn is_position_available_xy(cache: &GameData, x: usize, y: usize) -> bool {
    if x > 24 || y > 16 {
        return false;
    }
    is_position_available(cache, world_idx(x, y))
}

fn is_position_available(cache: &GameData, idx: usize) -> bool {
    if idx > cache.map.len() {
        return false;
    }
    0 == cache.map[idx]
}

// this gets called by MGFW with input events
#[rustfmt::skip]
pub fn event(cache: &mut GameData, _heap: &mut GameDataHeap, world: &mut mgfw::ecs::World, event_id: u8) -> bool {

    if 2 == cache.transition_state {
        if cache.win {
            exit(0);
        }
        else if cache.gameover && event_id == mgfw::EVENT_INPUT_KEYBOARD_RELEASED_SPACE {
            cache.transition_state = 3;
            cache.transition_timer = 0.5;
            cache.level = 1;
            init_level(cache, world);
        }
    }

    if 0 != cache.transition_state { 
        cache.player_holding_up = false;
        cache.player_holding_down = false;
        cache.player_holding_left = false;
        cache.player_holding_right = false;
        return false;
    }

    match event_id {
        mgfw::EVENT_INPUT_KEYBOARD_RELEASED_ESCAPE => exit(0),
        mgfw::EVENT_INPUT_KEYBOARD_PRESSED_UP => {
            cache.player_holding_up = true;
        },
        mgfw::EVENT_INPUT_KEYBOARD_RELEASED_UP => { cache.player_holding_up = false; },
        mgfw::EVENT_INPUT_KEYBOARD_PRESSED_DOWN => {
            cache.player_holding_down = true;
        },
        mgfw::EVENT_INPUT_KEYBOARD_RELEASED_DOWN => { cache.player_holding_down = false; },
        mgfw::EVENT_INPUT_KEYBOARD_PRESSED_LEFT => {
            cache.player_holding_left = true;
        },
        mgfw::EVENT_INPUT_KEYBOARD_RELEASED_LEFT => { cache.player_holding_left = false; },
        mgfw::EVENT_INPUT_KEYBOARD_PRESSED_RIGHT => {
            cache.player_holding_right = true;
        },
        mgfw::EVENT_INPUT_KEYBOARD_RELEASED_RIGHT => { cache.player_holding_right = false; },
        /*mgfw::EVENT_INPUT_KEYBOARD_RELEASED_SPACE => {
            cache.transition_state = 1;
            cache.transition_timer = 0.5;
        },*/
        _ => ()
    }

    true
}

pub fn check_gem_collect(cache: &mut GameData, world: &mut mgfw::ecs::World) {
    if cache.gem_count == cache.player_gems {
        return;
    }

    for i in 0..cache.gem_count {
        if cache.gems[i] == cache.player_idx {
            //let e = world.clone_entity(cache.gem_ents[i]);
            let prev = cache.gem_ent_start + i;
            let e = cache.pickup_ent_start + i;
            let pos = world.entity_get_position(prev);
            world.entity_set_position(e, pos);
            world.entity_set_visibility(e, true);
            let vy = -48.0 * (1.0 + world.rnd());
            world.entity_set_velocity_xy(e, 0.0, vy);
            world.entity_set_visibility(prev, false);
            world.entity_set_alpha(e, 1.0);

            cache.gems[i] = 0;
            cache.player_gems += 1;
            update_hud(cache, world);

            if 10 > cache.level {
                if cache.gem_count == cache.player_gems {
                    insert_exit(cache, world);
                }
            } else {
                cache.transition_state = 1;
                cache.transition_timer = 0.5;
                cache.win = true;
            }
        }
    }
}

pub fn shutdown(_cache: &mut GameData, heap: &mut GameDataHeap) {
    // deallocate and overwrite existing memory
    *heap = GameDataHeap::default();

    // re-box and consume
    //let _temp = unsafe { Box::from_raw(cache.heap) };
}

// this gets called by MGFW at 1200hz
#[rustfmt::skip]
pub fn update(cache: &mut GameData, _heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) -> bool {
    let mut expect_blown = false;
    cache.frame = (cache.frame + 1) % 128;

    let dt = 1.0 / 1200.0;

    if !cache.ready {
        if 127 == cache.frame {
            cache.ready = true;
        }
        return false;
    }

    cache.centaur_timer -= dt;
    cache.player_timer -= dt;

    // Amortize workload
    if 0 == cache.frame % 4 {
        update_player_movement(cache, world);
        update_centaur_movement(cache, world);
    }

    if 0 == cache.frame {
        if 0.01 > world.rnd() {
            cache.centaur_dir = (cache.centaur_dir + world.rnd_range(0..4)) % 4;
        }
    }
    else if 64 == cache.frame {
        check_gem_collect(cache, world);
        for g in 0..MAX_GEMS {
            let gidx = g + cache.pickup_ent_start;
            let vel = world.entity_get_velocity(gidx);
            if -1.0e-6 > vel.y {
                let alpha = world.entity_get_alpha(gidx) - (dt * 128.0) as f32;
                if 0.0 > alpha {
                    world.entity_set_visibility(gidx, false);
                } else {
                    world.entity_set_alpha(gidx, alpha);
                }
            }
        }
        if cache.player_idx == cache.centaur_idx && !cache.gameover && !cache.win {
            cache.gameover = true;
            cache.transition_state = 1;
            cache.transition_timer = 0.5;
        }
    }

    if 0 != cache.transition_state {
        cache.transition_timer -= dt;
        if 0.0 > cache.transition_timer {
            if (!cache.win && !cache.gameover) || ((cache.win || cache.gameover) && 2 > cache.transition_state) || 2 < cache.transition_state {
                cache.transition_state = (1 + cache.transition_state) % 4;
            }
            
            cache.transition_timer = 0.5;
            if 2 == cache.transition_state && !cache.win && !cache.gameover {
                cache.level += 1;
                init_level(cache, world);
            }
            else if 0 == cache.transition_state && cache.gameover {
                cache.gameover = false;
            }
        }

        match cache.transition_state {
            1 => {
                let alpha = 1.0 - cache.transition_timer * 2.0;
                world.entity_set_alpha(cache.game_over_entity_start, alpha as f32);
                world.entity_set_visibility(cache.game_over_entity_start, true);
                
                if !cache.win && !cache.gameover {
                    world.entity_set_alpha(cache.game_over_entity_start + 1, alpha as f32);
                    world.entity_set_visibility(cache.game_over_entity_start + 1, true);
                }
                else if cache.win {
                    world.entity_set_alpha(cache.game_over_entity_start + 4, alpha as f32);
                    world.entity_set_visibility(cache.game_over_entity_start + 4, true);
                }
                else if cache.gameover {
                    world.entity_set_alpha(cache.game_over_entity_start + 2, alpha as f32);
                    world.entity_set_visibility(cache.game_over_entity_start + 2, true);
                    world.entity_set_alpha(cache.game_over_entity_start + 3, alpha as f32);
                    world.entity_set_visibility(cache.game_over_entity_start + 3, true);
                }
            },
            2 => {
                world.entity_set_alpha(cache.game_over_entity_start, 1.0);
                
                if !cache.win && !cache.gameover {
                    world.entity_set_alpha(cache.game_over_entity_start + 1, 1.0);
                }
                else if cache.win {
                    world.entity_set_alpha(cache.game_over_entity_start + 4, 1.0);
                }
                else if cache.gameover {
                    world.entity_set_alpha(cache.game_over_entity_start + 2, 1.0);
                    world.entity_set_alpha(cache.game_over_entity_start + 3, 1.0);
                }
            }
            3 => {
                let alpha = cache.transition_timer * 2.0;
                world.entity_set_alpha(cache.game_over_entity_start, alpha as f32);
                
                if !cache.win && !cache.gameover {
                    world.entity_set_alpha(cache.game_over_entity_start + 1, alpha as f32);
                }
                else if cache.win {
                    world.entity_set_alpha(cache.game_over_entity_start + 4, alpha as f32);
                }
                else if cache.gameover {
                    world.entity_set_alpha(cache.game_over_entity_start + 2, alpha as f32);
                    world.entity_set_alpha(cache.game_over_entity_start + 3, alpha as f32);
                }
            },
            _ => {
                world.entity_set_visibility(cache.game_over_entity_start, false);
                
                if !cache.win && !cache.gameover {
                    world.entity_set_visibility(cache.game_over_entity_start + 1, false);
                }
                else if cache.win {
                    world.entity_set_visibility(cache.game_over_entity_start + 4, false);
                }
                else if cache.gameover {
                    world.entity_set_visibility(cache.game_over_entity_start + 2, false);
                    world.entity_set_visibility(cache.game_over_entity_start + 3, false);
                }
            }
        }
    }

    expect_blown
}

fn update_centaur_movement(cache: &mut GameData, world: &mut mgfw::ecs::World) {
    let cx = world_x(cache.centaur_idx);
    let cy = world_y(cache.centaur_idx);

    let edge = get_level_edges(cache.level);

    if cache.centaur_tidx == cache.centaur_idx && cx > 0 && cy > 0 {
        match cache.centaur_dir {
            0 => {
                if cy > edge.1 + 2 {
                    let idx = world_idx(cx, cy - 1);
                    if is_position_available(cache, idx) {
                        cache.centaur_tidx = idx;
                    }
                }
            }
            1 => {
                if cy < BOARD_Y - edge.1 - 4 {
                    let idx = world_idx(cx, cy + 1);
                    if is_position_available(cache, idx) {
                        cache.centaur_tidx = idx;
                    }
                }
            }
            2 => {
                let idx = world_idx(cx - 1, cy);
                if is_position_available(cache, idx) {
                    cache.centaur_tidx = idx;
                }
            }
            3 => {
                let idx = world_idx(cx + 1, cy);
                if is_position_available(cache, idx) {
                    cache.centaur_tidx = idx;
                }
            }
            _ => (),
        }

        if cache.centaur_tidx == cache.centaur_idx {
            cache.centaur_dir = (cache.centaur_dir + world.rnd_range(0..4)) % 4;
        }
    }

    let s = 0.005 + 0.02 * (cache.player_gems as f32 / cache.gem_count as f32);

    if cache.centaur_idx != cache.centaur_tidx {
        let tx = world_x(cache.centaur_tidx);
        let ty = world_y(cache.centaur_tidx);

        if ty < cy {
            let mut y = cache.centaur_pos.y - s;
            if y < ty as f32 {
                y = ty as f32;
                cache.centaur_idx = cache.centaur_tidx;
            }
            cache.centaur_pos.y = y;
        } else if ty > cy {
            let mut y = cache.centaur_pos.y + s;
            if y > ty as f32 {
                y = ty as f32;
                cache.centaur_idx = cache.centaur_tidx;
            }
            cache.centaur_pos.y = y;
        } else if tx < cx {
            let mut x = cache.centaur_pos.x - s;
            if x < tx as f32 {
                x = tx as f32;
                cache.centaur_idx = cache.centaur_tidx;
            }
            cache.centaur_pos.x = x;
        } else if tx > cx {
            let mut x = cache.centaur_pos.x + s;
            if x > tx as f32 {
                x = tx as f32;
                cache.centaur_idx = cache.centaur_tidx;
            }
            cache.centaur_pos.x = x;
        }

        update_entity_positions(cache, world);
    }
}

fn update_player_movement(cache: &mut GameData, world: &mut mgfw::ecs::World) {
    let px = world_x(cache.player_idx);
    let py = world_y(cache.player_idx);

    if cache.player_tidx == cache.player_idx && px > 0 && py > 0 {
        if cache.player_holding_up {
            let idx = world_idx(px, py - 1);
            if is_position_available(cache, idx) {
                cache.player_tidx = idx;
            }
        } else if cache.player_holding_down {
            let idx = world_idx(px, py + 1);
            if is_position_available(cache, idx) {
                cache.player_tidx = idx;
            }
        } else if cache.player_holding_left {
            let idx = world_idx(px - 1, py);
            if is_position_available(cache, idx) {
                cache.player_tidx = idx;
            }
        } else if cache.player_holding_right {
            let idx = world_idx(px + 1, py);
            if is_position_available(cache, idx) {
                cache.player_tidx = idx;
            }
        }
    }

    let s = 0.02;

    if cache.player_idx != cache.player_tidx {
        let tx = world_x(cache.player_tidx);
        let ty = world_y(cache.player_tidx);

        if ty < py {
            let mut y = cache.player_pos.y - s;
            if y < ty as f32 {
                y = ty as f32;
                cache.player_idx = cache.player_tidx;
            }
            cache.player_pos.y = y;
        } else if ty > py {
            let mut y = cache.player_pos.y + s;
            if y > ty as f32 {
                y = ty as f32;
                cache.player_idx = cache.player_tidx;
            }
            cache.player_pos.y = y;
        } else if tx < px {
            let mut x = cache.player_pos.x - s;
            if x < tx as f32 {
                x = tx as f32;
                cache.player_idx = cache.player_tidx;
            }
            cache.player_pos.x = x;
        } else if tx > px {
            let mut x = cache.player_pos.x + s;
            if x > tx as f32 {
                x = tx as f32;
                cache.player_idx = cache.player_tidx;
            }
            cache.player_pos.x = x;
        }

        update_entity_positions(cache, world);

        check_gem_collect(cache, world);

        let edge = get_level_edges(cache.level);

        if edge.1 == world_y(cache.player_idx) {
            cache.transition_state = 1;
            cache.transition_timer = 0.5;
        }
    }
}

pub fn gen_maze(
    cols: usize,
    mask: &Vec<u8>,
    start_idx: usize,
    weight: f32,
    rng: &mut ThreadRng,
) -> Vec<u8> {
    let sz = mask.len();
    let rows = (sz - (sz % cols)) / cols;

    // temp vars
    let mut parent: Vec<i32> = vec![-1; sz];
    let mut nchild: Vec<i32> = vec![0; sz];
    let mut maze: Vec<u8> = vec![0; sz];

    for idx in 0..sz {
        if 0 != mask[idx] {
            parent[idx] = -2;
            maze[idx] = 0b10000;
        }
    }

    for i in 0..sz {
        let idx = (i + start_idx) % sz;
        if -1 == parent[idx] {
            let start = idx;
            let mut cur = idx;

            parent[cur] = start as i32;
            nchild[cur] += 1;

            let mut prev: (i32, i32) = (0, 0);

            let mut keepgoing = true;

            while cur != start || (cur == start && keepgoing) {
                let cx = cur % cols;
                let cy = (cur - (cur % cols)) / cols;

                // is there an open neighbor?
                let mut opening: Vec<(i32, i32)> = Vec::new();

                if cy > 0 && -1 == parent[cx + (cy - 1) * cols] {
                    opening.push((0, -1));
                }
                if cy < rows - 1 && -1 == parent[cx + (cy + 1) * cols] {
                    opening.push((0, 1));
                }
                if cx > 0 && -1 == parent[cx - 1 + cy * cols] {
                    opening.push((-1, 0));
                }
                if cx < cols - 1 && -1 == parent[cx + 1 + cy * cols] {
                    opening.push((1, 0));
                }

                if !opening.is_empty() {
                    let mut nid: usize = (rng.gen::<f32>() * opening.len() as f32) as usize;
                    if rng.gen::<f32>() < weight {
                        if 0 == prev.0 && 0 != prev.1 {
                            for j in 0..opening.len() {
                                if prev.0 == opening[j].0 && prev.1 == opening[j].1 {
                                    nid = j;
                                    break;
                                }
                            }
                        } else if 0 != prev.0 && 0 == prev.1 {
                            for j in 0..opening.len() {
                                if prev.0 == opening[j].0 && prev.1 == opening[j].1 {
                                    nid = j;
                                    break;
                                }
                            }
                        }
                    }
                    let nidx = (cx as i32 + opening[nid].0) as usize
                        + (cy as i32 + opening[nid].1) as usize * cols;
                    prev = opening[nid];
                    parent[nidx] = cur as i32;
                    nchild[cur] += 1;
                    let mut flag: u8 = 0;
                    if 0 == opening[nid].0 {
                        if -1 == opening[nid].1 {
                            flag = 0b0001;
                        }
                        if 1 == opening[nid].1 {
                            flag = 0b0010;
                        }
                    } else if 0 == opening[nid].1 {
                        if -1 == opening[nid].0 {
                            flag = 0b0100;
                        }
                        if 1 == opening[nid].0 {
                            flag = 0b1000;
                        }
                    }
                    maze[cur] = maze[cur] | flag;
                    cur = nidx;
                    flag = 0;
                    if 0 == opening[nid].0 {
                        if 1 == opening[nid].1 {
                            flag = 0b0001;
                        }
                        if -1 == opening[nid].1 {
                            flag = 0b0010;
                        }
                    } else if 0 == opening[nid].1 {
                        if 1 == opening[nid].0 {
                            flag = 0b0100;
                        }
                        if -1 == opening[nid].0 {
                            flag = 0b1000;
                        }
                    }
                    maze[cur] = maze[cur] | flag;
                } else {
                    cur = parent[cur] as usize;
                }

                keepgoing = false;
                if cur == start {
                    if !opening.is_empty() {
                        opening.clear();
                    }

                    if cy > 0 && -1 == parent[cx + (cy - 1) * cols] {
                        opening.push((0, -1));
                    }
                    if cy < rows - 1 && -1 == parent[cx + (cy + 1) * cols] {
                        opening.push((0, 1));
                    }
                    if cx > 0 && -1 == parent[cx - 1 + cy * cols] {
                        opening.push((-1, 0));
                    }
                    if cx < cols - 1 && -1 == parent[cx + 1 + cy * cols] {
                        opening.push((1, 0));
                    }

                    if !opening.is_empty() {
                        keepgoing = true;
                    }
                }
            }
        }
    }

    maze
}

pub fn draw_maze(cols: usize, maze: &Vec<u8>) {
    let sz = maze.len();
    let rows = (sz - (sz % cols)) / cols;
    println!("Map Maze:");

    for y in 0..rows {
        for x in 0..cols {
            let val = maze[y * cols + x];
            match val {
                0b0001 => print!("^"),
                0b0010 => print!("v"),
                0b0100 => print!("<"),
                0b1000 => print!(">"),
                0b0011 => print!("│"),
                0b0101 => print!("┘"),
                0b0110 => print!("┐"),
                0b0111 => print!("┤"),
                0b1001 => print!("└"),
                0b1010 => print!("┌"),
                0b1011 => print!("├"),
                0b1100 => print!("─"),
                0b1101 => print!("┴"),
                0b1110 => print!("┬"),
                0b1111 => print!("┼"),
                0b10000 => print!("▒"),
                _ => print!("o"),
            }
        }
        println!();
    }
}
