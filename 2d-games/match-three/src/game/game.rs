use crate::mgfw;
use std::collections::VecDeque;

#[derive(Default)]
pub struct GameDataHeap {
    // WARNING: Anything below this line is not in cache!
    boom_deque: VecDeque<u8>,
}

struct Icon {
    class: u8,
    entity: u8,
    slot: u8,
    target: mgfw::ecs::Position,
    animating: bool,
}

struct Pickup {
    entity: u8,
    class: u8,
    timer: i32,
    start: mgfw::ecs::Position,
}

pub struct GameData {
    pub heap: *mut GameDataHeap,
    frame: u8,
    hover_idx: i16,
    bomb_idx: u8,
    swap_idx: i16,
    swap_alpha_target: f32,
    hazard_alpha_target: f32,
    click_timer: i16,
    board: [u8; 81],
    icons: [Icon; 81],
    collect: [u8; 81],
    collect_count: [u8; 4],
    animating: bool,
    ready: bool,
    bag: [u8; 80],
    bag_sz: u8,
    level: u8,
    score: i32,
    score_prev: i32,
    pickups: [Pickup; 20],
    game_timer: f64,
    level_timer: f64,
    show_popup: bool,
    popup_timer: f64,
    explosion: bool,
}

const ICON_OPEN: u8 = 99;
const BOMB_INVALID: u8 = 99;
const HOVER_INVALID: i16 = 99;
const SWAP_INVALID: i16 = 999;
const MINI_GEM_IDX: usize = 93;

fn update_image(cache: &mut GameData, idx: usize, world: &mut mgfw::ecs::World) {
    let entity = cache.icons[idx].entity as usize;
    match cache.icons[idx].class {
        0 => world.entity_set_billboard(entity, String::from("assets/gem-blue.png")),
        1 => world.entity_set_billboard(entity, String::from("assets/gem-green.png")),
        2 => world.entity_set_billboard(entity, String::from("assets/gem-red.png")),
        3 => world.entity_set_billboard(entity, String::from("assets/gem-purple.png")),
        4 => world.entity_set_billboard(entity, String::from("assets/stone-head.png")),
        5 => world.entity_set_billboard(entity, String::from("assets/bomb.png")),
        _ => (),
    }
}

fn gen_class(level: u8) -> u8 {
    let mut class = (mgfw::rnd() * 4.0) as u8;
    if mgfw::rnd() < (0.05 + 0.01 * level as f32) {
        class = 4;
    }
    if mgfw::rnd() < 0.1 {
        class = 5;
    }
    class
}

#[rustfmt::skip]
pub fn initialize(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    world.parse_world("assets/world.dat");
    cache.level = 1;

    for i in 0..81 {
        let id = world.new_entity();
        cache.icons[i].entity = id as u8;
        cache.icons[i].class = gen_class(cache.level);
        cache.icons[i].slot = ICON_OPEN;
        world.entity_set_scale_xy(id, 64.0, 64.0);
        world.entity_set_visibility(id, false);
        update_image(cache, i, world);
        //
        cache.board[i] = ICON_OPEN;
    }

    for i in 0..80 {
        let id = world.new_entity();
        world.entity_set_scale_xy(id, 16.0, 16.0);
        world.entity_set_position_xy(id, 640.0 + 8.0 + (id % 4) as f32 * 16.0, 506.0 - (i - (i % 4)) as f32 / 4.0 * 16.0);
        world.entity_set_billboard(id, String::from("assets/gem-blue.png"));
        world.entity_set_visibility(id, false);
    }

    cache.hover_idx = HOVER_INVALID;
    cache.swap_idx = SWAP_INVALID;
    cache.bomb_idx = BOMB_INVALID;
    cache.swap_alpha_target = 0.0;
    cache.animating = false;
    cache.ready = false;
    cache.bag = [0; 80];
    cache.bag_sz = 0;
    cache.score = 0;
    cache.score_prev = 0;
    cache.level_timer = level_clock(cache);
    cache.game_timer = 0.0;
    cache.show_popup = false;
    cache.popup_timer = 0.0;
    cache.explosion = false;
    heap.boom_deque = VecDeque::new();

    world.entity_set_text(89, format!("Level {}", cache.level));
    let ln = world.text_get_width(89) as f32 * 0.5 * 1.5;
    world.entity_set_position_xy(89, 672.0 - ln, 32.0);

    for i in 0..20 {
        cache.pickups[i].timer = -1;
        let id = world.new_entity();
        cache.pickups[i].entity = id as u8;
        world.entity_set_billboard(id, String::from("assets/gem-blue.png"));
        world.entity_set_visibility(id, false);
    }
    
    world.entity_set_text(91, String::from("Click to Begin"));
    world.entity_set_position_xy(91, 320.0 - world.text_get_width(91) as f32 * 0.5, 300.0);
    world.entity_set_visibility(91, true);

    world.entity_set_alpha(92, 0.0);
}

fn level_clock(cache: &GameData) -> f64 {
    60.0 * 5.0 * 0.95f64.powf(cache.level as f64)
}

fn level_up(cache: &mut GameData, world: &mut mgfw::ecs::World) {
    cache.level += 1;
    for i in 0..81 {
        let id = 3 + i;
        cache.icons[i].entity = id as u8;
        cache.icons[i].class = gen_class(cache.level);
        cache.icons[i].slot = ICON_OPEN;
        world.entity_set_visibility(id, false);
        update_image(cache, i, world);
        //
        cache.board[i] = ICON_OPEN;
    }

    cache.level_timer = level_clock(cache);
    cache.game_timer = 0.0;
    cache.show_popup = true;
    cache.popup_timer = 3.0;
    cache.score_prev = cache.score;
    cache.hover_idx = HOVER_INVALID;
    cache.swap_idx = SWAP_INVALID;
    cache.bomb_idx = BOMB_INVALID;
    cache.swap_alpha_target = 0.0;

    world.entity_set_text(91, String::from("Level Up!"));
    world.entity_set_position_xy(91, 320.0 - world.text_get_width(91) as f32 * 0.5, 300.0);

    for i in 0..80 {
        let id = MINI_GEM_IDX + i;
        world.entity_set_visibility(id, false);
    }

    for i in 0..20 {
        cache.pickups[i].timer = -1;
    }

    world.entity_set_text(89, format!("Level {}", cache.level));
    let ln = world.text_get_width(89) as f32 * 0.5 * 1.5;
    world.entity_set_position_xy(89, 672.0 - ln, 32.0);
    cache.bag = [0; 80];
    cache.bag_sz = 0;

    world.entity_set_text(2, format!("{}", cache.score));
    let ln = world.text_get_width(2) as f32 * 0.5 * 1.5;
    world.entity_set_position_xy(2, 672.0 - ln, 590.0);
}

fn time_up(cache: &mut GameData, world: &mut mgfw::ecs::World) {
    for i in 0..81 {
        let id = 3 + i;
        cache.icons[i].entity = id as u8;
        cache.icons[i].class = gen_class(cache.level);
        cache.icons[i].slot = ICON_OPEN;
        world.entity_set_visibility(id, false);
        update_image(cache, i, world);
        //
        cache.board[i] = ICON_OPEN;
    }

    cache.score = cache.score_prev;
    cache.level_timer = level_clock(cache);
    cache.game_timer = 0.0;
    cache.show_popup = true;
    cache.popup_timer = 3.0;
    cache.hover_idx = HOVER_INVALID;
    cache.swap_idx = SWAP_INVALID;
    cache.bomb_idx = BOMB_INVALID;
    cache.swap_alpha_target = 0.0;
    cache.bag = [0; 80];
    cache.bag_sz = 0;

    world.entity_set_text(91, String::from("Time's Up!"));
    world.entity_set_position_xy(91, 320.0 - world.text_get_width(91) as f32 * 0.5, 300.0);

    for i in 0..80 {
        let id = MINI_GEM_IDX + i;
        world.entity_set_visibility(id, false);
    }

    for i in 0..20 {
        cache.pickups[i].timer = -1;
    }

    world.entity_set_text(2, format!("{}", cache.score));
    let ln = world.text_get_width(2) as f32 * 0.5 * 1.5;
    world.entity_set_position_xy(2, 672.0 - ln, 590.0);

    world.entity_set_alpha(84, 0.0);
    world.entity_set_alpha(85, 0.0);
}

pub fn get_hover_idx(mx: i32, my: i32) -> i16 {
    let sx = mx - 32;
    let sy = my - 32;
    if 0 > sx || 0 > sy || 575 < sx || 575 < sy {
        return HOVER_INVALID;
    }
    let ix = (sx - sx % 64) / 64;
    let iy = (sy - sy % 64) / 64;
    (iy * 9 + ix) as i16
}

pub fn get_swap_idx(cache: &mut GameData, hidx: i16, mx: i32, my: i32) -> (i16, f32, f32) {
    if cache.animating {
        return (SWAP_INVALID, 0.0, 0.0);
    }
    if ICON_OPEN == cache.board[hidx as usize] {
        return (SWAP_INVALID, 0.0, 0.0);
    }
    if cache.icons[cache.board[hidx as usize] as usize].animating {
        return (SWAP_INVALID, 0.0, 0.0);
    }
    if cache.icons[cache.board[hidx as usize] as usize].class > 3 {
        return (SWAP_INVALID, 0.0, 0.0);
    }
    let hx = hidx % 9;
    let hy = (hidx - hx) / 9;
    let cx = (hx + 1) as i32 * 64;
    let cy = (hy + 1) as i32 * 64;

    let r = 15 * 15;

    if 0 < hy {
        if ICON_OPEN != cache.board[(hidx - 9) as usize]
            && !cache.icons[cache.board[(hidx - 9) as usize] as usize].animating
            && cache.icons[cache.board[(hidx - 9) as usize] as usize].class < 4
        {
            let sidx = hidx * 4 + 0;
            let dx = cx + 0;
            let dy = cy - 32;
            if (mx - dx) * (mx - dx) + (my - dy) * (my - dy) < r {
                return (sidx, dx as f32, dy as f32);
            }
        }
    }

    if 8 > hy {
        if ICON_OPEN != cache.board[(hidx + 9) as usize]
            && !cache.icons[cache.board[(hidx + 9) as usize] as usize].animating
            && cache.icons[cache.board[(hidx + 9) as usize] as usize].class < 4
        {
            let sidx = hidx * 4 + 1;
            let dx = cx + 0;
            let dy = cy + 32;
            if (mx - dx) * (mx - dx) + (my - dy) * (my - dy) < r {
                return (sidx, dx as f32, dy as f32);
            }
        }
    }

    if 0 < hx {
        if ICON_OPEN != cache.board[(hidx - 1) as usize]
            && !cache.icons[cache.board[(hidx - 1) as usize] as usize].animating
            && cache.icons[cache.board[(hidx - 1) as usize] as usize].class < 4
        {
            let sidx = hidx * 4 + 2;
            let dx = cx - 32;
            let dy = cy + 0;
            if (mx - dx) * (mx - dx) + (my - dy) * (my - dy) < r {
                return (sidx, dx as f32, dy as f32);
            }
        }
    }

    if 8 > hx {
        if ICON_OPEN != cache.board[(hidx + 1) as usize]
            && !cache.icons[cache.board[(hidx + 1) as usize] as usize].animating
            && cache.icons[cache.board[(hidx + 1) as usize] as usize].class < 4
        {
            let sidx = hidx * 4 + 3;
            let dx = cx + 32;
            let dy = cy + 0;
            if (mx - dx) * (mx - dx) + (my - dy) * (my - dy) < r {
                return (sidx, dx as f32, dy as f32);
            }
        }
    }

    (SWAP_INVALID, 0.0, 0.0)
}

fn add_to_bag(cache: &mut GameData, world: &mut mgfw::ecs::World, gem: u8, count: usize) {
    assert!(4 > gem);
    for _i in 0..count {
        if 80 > cache.bag_sz {
            let idx = cache.bag_sz as usize;
            let entity = MINI_GEM_IDX + idx;

            cache.bag[idx] = gem;
            let image = match gem {
                0 => String::from("assets/gem-blue.png"),
                1 => String::from("assets/gem-green.png"),
                2 => String::from("assets/gem-red.png"),
                3 => String::from("assets/gem-purple.png"),
                _ => String::from(""),
            };

            world.entity_set_billboard(entity, image.clone());

            world.entity_set_visibility(entity, true);
            world.entity_set_alpha_ease(entity, 0.0, 1.0, 1.0);

            cache.bag_sz += 1;
            cache.score += ((gem as f32 + 2.0).powf(1.5) * cache.level as f32) as i32;

            let mut first_open: i32 = -1;
            for p in 0..20 {
                if 0 >= cache.pickups[p].timer {
                    first_open = p as i32;
                    break;
                }
            }
            if 0 <= first_open {
                let e = cache.pickups[first_open as usize].entity;
                cache.pickups[first_open as usize].class = gem;
                cache.pickups[first_open as usize].timer = 90;
                cache.pickups[first_open as usize].start = mgfw::ecs::Position {
                    x: world.mouse_x as f32 + 120.0 * (mgfw::rnd() - 0.5),
                    y: world.mouse_y as f32 - 100.0 * mgfw::rnd(),
                };
                world.entity_set_billboard(e as usize, image);
            }
        }
    }

    world.entity_set_text(2, format!("{}", cache.score));
    let ln = world.text_get_width(2) as f32 * 0.5 * 1.5;
    world.entity_set_position_xy(2, 672.0 - ln, 590.0);

    if 80 == cache.bag_sz {
        level_up(cache, world);
    }
}

#[rustfmt::skip]
pub fn update(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) -> bool {
    let expect_blown = false;

    cache.frame = (cache.frame + 1) % 128;

    if 0 < cache.click_timer {
        cache.click_timer -= 1;
    }

    if !cache.ready {
        return false;
    }

    if cache.show_popup {
        cache.popup_timer -= 833.0e-6;
        if 0.0 > cache.popup_timer {
            cache.popup_timer = 0.0;
            cache.show_popup = false;
            world.entity_set_visibility(90, false);
            world.entity_set_visibility(91, false);
        }
        else {
            let mut alpha = (cache.popup_timer * mgfw::PI / 3.0).sin() * 4.0;
            if 1.0 < alpha { alpha = 1.0; }
            world.entity_set_color_rgba(90, 0.2, 0.4, 0.8, 0.5 * alpha as f32);
            world.entity_set_visibility(90, true);
            world.entity_set_visibility(91, true);
        }
        return false;
    }

    cache.explosion = false;
    if !heap.boom_deque.is_empty() {
        cache.explosion = true;
        if world.entity_get_alpha(92) < 1.0e-6 {
            let hidx = heap.boom_deque.pop_front().unwrap();
            let hx = (hidx % 9) as f32 * 64.0 + 64.0;
            let hy = (hidx - (hidx % 9)) as f32 / 9.0 * 64.0 + 64.0;
            world.entity_set_position_xy(92, hx, hy);
            world.entity_set_angle(92, mgfw::rnd() * mgfw::PI as f32 * 2.0);
            world.entity_set_alpha_ease(92, 1.0, 0.0, 0.1);
            world.entity_set_visibility(92, true);
        }
        return false;
    }

    // Amortize workload
    let frame8 = cache.frame % 8;
    if 0 == frame8 {
        // noop

        // fill top row
        if !cache.animating {
            for col in 0..9 {
                if ICON_OPEN == cache.board[col] {
                    // find first available icon to fill
                    for icon in 0..81 {
                        if ICON_OPEN == cache.icons[icon].slot {
                            cache.icons[icon].slot = col as u8;
                            cache.board[col] = icon as u8;
                            let cx = (col + 1) * 64;
                            let cy = 64;
                            let entity = cache.icons[icon].entity as usize;
                            cache.icons[icon].target = mgfw::ecs::Position {
                                x: cx as f32,
                                y: cy as f32,
                            };
                            cache.icons[icon].animating = true;
                            world.entity_set_position_xy(entity, cx as f32, cy as f32 - 64.0);
                            world.entity_set_visibility(entity, true);
                            break;
                        }
                    }
                }
            }
        }

    }
    else if 1 == frame8 {

        // check slide
        for row in 0..8 {
            for col in 0..9 {
                let fidx = row * 9 + col;
                let tidx = fidx + 9;
                if ICON_OPEN == cache.board[tidx] && ICON_OPEN != cache.board[fidx] {
                    let icon = cache.board[fidx] as usize;
                    cache.board[tidx] = icon as u8;
                    cache.board[fidx] = ICON_OPEN;
                    //
                    cache.icons[icon].slot = tidx as u8;
                    let cy = (row + 2) * 64;
                    cache.icons[icon].target.y = cy as f32;
                    cache.icons[icon].animating = true;
                }
            }
        }

    }
    else if 2 == frame8 {

        //world.entity_set_visibility(84, false);
        let hidx = get_hover_idx(world.mouse_x, world.mouse_y);
        cache.swap_alpha_target = 0.0;
        cache.hazard_alpha_target = 0.0;
        if HOVER_INVALID != hidx {
            cache.bomb_idx = BOMB_INVALID;
            if !cache.animating && ICON_OPEN != cache.board[hidx as usize] && 5 == cache.icons[cache.board[hidx as usize] as usize].class && !cache.icons[cache.board[hidx as usize] as usize].animating {
                let hx = hidx % 9;
                let hy = (hidx - hx) / 9;
                world.entity_set_position_xy(85, ((hx + 1) * 64) as f32, ((hy + 1) * 64) as f32);
                cache.hazard_alpha_target = 1.0;
                cache.bomb_idx = hidx as u8;
            }
            let sidx = get_swap_idx(cache, hidx, world.mouse_x, world.mouse_y);
            if SWAP_INVALID != sidx.0 {
                world.entity_set_position_xy(84, sidx.1, sidx.2);
                cache.swap_alpha_target = 1.0;
            }
            cache.swap_idx = sidx.0;
        }
        let a = world.entity_get_alpha(84);
        world.entity_set_alpha(84, a + (cache.swap_alpha_target - a) * 0.05);
        
        let a = world.entity_get_alpha(85);
        world.entity_set_alpha(85, a + (cache.hazard_alpha_target - a) * 0.05);
        
    }
    else if 3 == frame8 {

        // check if ready to collect
        cache.collect = [0; 81];
        for row in 0..9 {
            for col in 0..9 {
                let idx = (row * 9 + col) as i32;
                if ICON_OPEN == cache.board[idx as usize] {
                    continue;
                }
                if cache.icons[cache.board[(idx + 0) as usize] as usize].animating {
                    continue;
                }

                let mut a = 9;
                let mut b = 9;
                let mut c = 9;
                let mut d = 9;
                let e = cache.icons[cache.board[(idx + 0) as usize] as usize].class;

                if 0 < row {
                    if ICON_OPEN != cache.board[(idx - 9) as usize] {
                        if !cache.icons[cache.board[(idx - 9) as usize] as usize].animating &&
                        cache.icons[cache.board[(idx - 9) as usize] as usize].class < 4 {
                            a = cache.icons[cache.board[(idx - 9) as usize] as usize].class;
                        }
                    }
                }
                if 8 > row {
                    if ICON_OPEN != cache.board[(idx + 9) as usize] {
                        if !cache.icons[cache.board[(idx + 9) as usize] as usize].animating &&
                        cache.icons[cache.board[(idx + 9) as usize] as usize].class < 4 {
                            b = cache.icons[cache.board[(idx + 9) as usize] as usize].class;
                        }
                    }
                }
                if 0 < col {
                    if ICON_OPEN != cache.board[(idx - 1) as usize] {
                        if !cache.icons[cache.board[(idx - 1) as usize] as usize].animating &&
                        cache.icons[cache.board[(idx - 1) as usize] as usize].class < 4 {
                            c = cache.icons[cache.board[(idx - 1) as usize] as usize].class;
                        }
                    }
                }
                if 8 > col {
                    if ICON_OPEN != cache.board[(idx + 1) as usize] {
                        if !cache.icons[cache.board[(idx + 1) as usize] as usize].animating &&
                        cache.icons[cache.board[(idx + 1) as usize] as usize].class < 4 {
                            d = cache.icons[cache.board[(idx + 1) as usize] as usize].class;
                        }
                    }
                }

                if a == e && b == e {
                    cache.collect[(idx + 0) as usize] = 1;
                    cache.collect[(idx - 9) as usize] = 1;
                    cache.collect[(idx + 9) as usize] = 1;
                }
                if c == e && d == e {
                    cache.collect[(idx + 0) as usize] = 1;
                    cache.collect[(idx - 1) as usize] = 1;
                    cache.collect[(idx + 1) as usize] = 1;
                }
            }
        }

        cache.collect_count = [0; 4];
        for i in 0..81 {
            if 1 == cache.collect[i] {
                let idx = cache.board[i] as usize;
                let entity = cache.icons[idx].entity as usize;
                cache.collect_count[cache.icons[idx].class as usize] += 1;
                cache.icons[idx].class = gen_class(cache.level);
                cache.icons[idx].slot = ICON_OPEN;
                world.entity_set_visibility(entity, false);
                update_image(cache, idx, world);
                cache.board[i] = ICON_OPEN;
            }
        }

        for i in 0..4 {
            if 0< cache.collect_count[i] {
                add_to_bag(cache, world, i as u8, (cache.collect_count[i] as f32 / 3.0).ceil() as usize);
            }
        }
        
    }
    else if 4 == frame8 {

        // animate movement
        cache.animating = false;
        for i in 0..81 {
            if ICON_OPEN == cache.icons[i].slot {
                continue;
            }
            let mut pos = world.entity_get_position(cache.icons[i].entity as usize);
            let mut dx = (cache.icons[i].target.x - pos.x) * 0.1;
            let mut dy = (cache.icons[i].target.y - pos.y) * 0.1;
            let mx = 10.0;
            if mx < dx {
                dx = mx
            };
            if mx < dy {
                dy = mx
            };
            pos.x += dx;
            pos.y += dy;
            if dx.abs() > 0.1 || dy.abs() > 0.1 {
                cache.animating = true;
            } else {
                cache.icons[i].animating = false;
            }
            world.entity_set_position(cache.icons[i].entity as usize, pos);
        }

    }
    else if 5 == frame8 {

        for p in 0..20 {
            let ety = cache.pickups[p].entity as usize;
            world.entity_set_visibility(ety, false);
            if 0 < cache.pickups[p].timer {
                cache.pickups[p].timer -= 1;
                let ratio: f32 = cache.pickups[p].timer as f32 / 90.0;
                let endx = 672.0;
                let endy = 220.0 + (300.0 * (1.0 - cache.bag_sz as f32 / 80.0));
                let dx = endx - cache.pickups[p].start.x;
                let dy = endy - cache.pickups[p].start.y;
                let x = cache.pickups[p].start.x + dx * (1.0 - ratio);
                let y = cache.pickups[p].start.y + dy * (1.0 - ratio);
                world.entity_set_visibility(ety, true);
                let s = (ratio * mgfw::PI as f32).sin();
                let mut ss = 10.0 * s;
                if ss > 1.0 { ss = 1.0; }
                world.entity_set_scale_xy(ety, ss * 64.0, ss * 64.0);
                world.entity_set_position_xy(ety, x, y - s * 200.0);
                world.entity_set_angular_velocity(ety, 16.0);
                world.entity_set_alpha(ety, s);
            }
        }
    }

    cache.game_timer += 833.0e-6;
    let mut ratio = cache.game_timer / cache.level_timer;
    if 1.0 < ratio { ratio = 1.0; time_up(cache, world); }
    world.entity_set_scale_xy(86, 64.0, 60.0 * ratio as f32);
    world.entity_set_scale_xy(87, 64.0, 60.0 * (1.0 - ratio) as f32);
    world.entity_set_scale_xy(1, 8.0, 60.0 * (1.0 - ratio) as f32);
    world.entity_set_position_xy(86, 640.0, 122.0 + 60.0 * (1.0 - ratio) as f32);
    world.entity_set_position_xy(87, 640.0, 62.0 + 60.0 * ratio as f32);

    expect_blown
}

fn slide(cache: &mut GameData, id: u8, hidx: i16) {
    let hx = hidx % 9;
    let hy = (hidx - hx) / 9;
    let cx = (hx + 1) as i32 * 64;
    let cy = (hy + 1) as i32 * 64;
    cache.icons[id as usize].target = mgfw::ecs::Position {
        x: cx as f32,
        y: cy as f32,
    };
    cache.icons[id as usize].animating = true;
}

fn swap(cache: &mut GameData) {
    let sidx = cache.swap_idx;
    let oidx = sidx % 4;
    let hidx = (sidx - oidx) / 4;

    let src = hidx as usize;
    let dst = match oidx {
        0 => hidx - 9,
        1 => hidx + 9,
        2 => hidx - 1,
        3 => hidx + 1,
        _ => hidx,
    } as usize;

    let a = cache.board[src];
    let b = cache.board[dst];
    cache.board[src] = b;
    cache.board[dst] = a;

    slide(cache, a, dst as i16);
    slide(cache, b, src as i16);
}

fn destroy_block(cache: &mut GameData, world: &mut mgfw::ecs::World, hidx: usize) {
    if ICON_OPEN == cache.board[hidx] {
        return;
    }
    let idx = cache.board[hidx] as usize;
    let entity = cache.icons[idx].entity as usize;
    cache.icons[idx].class = gen_class(cache.level);
    cache.icons[idx].slot = ICON_OPEN;
    world.entity_set_visibility(entity, false);
    update_image(cache, idx, world);
    cache.board[hidx] = ICON_OPEN;
}

#[rustfmt::skip]
pub fn event(cache: &mut GameData, heap: &mut GameDataHeap, world: &mut mgfw::ecs::World, event_id: u8) -> bool {
    // noop

    if mgfw::EVENT_INPUT_MOUSE_BUTTON_UP != event_id { return false; }
    if cache.show_popup { return false; }
    if cache.explosion { return false; }
    //println!("{},{}", world.mouse_x, world.mouse_y);
    if 0 >= cache.click_timer {
        if SWAP_INVALID != cache.swap_idx {
            swap(cache);
            cache.click_timer = 400;
        }
        if BOMB_INVALID != cache.bomb_idx {
            // start fifo
            let mut fifo: std::boxed::Box<VecDeque<u8>> = Box::new(VecDeque::new());
            fifo.push_back(cache.bomb_idx);

            heap.boom_deque.push_back(cache.bomb_idx);

            // iterate fifo
            loop {
                if fifo.is_empty() { break; }
                let hidx = fifo.pop_front().unwrap() as usize;

                let hx = hidx % 9;
                let hy = (hidx - hx) / 9;

                if 0 < hy {
                    if ICON_OPEN != cache.board[(hidx - 9) as usize] {
                        if !cache.icons[cache.board[(hidx - 9) as usize] as usize].animating {

                        if 5 == cache.icons[cache.board[(hidx - 9) as usize] as usize].class  {
                            fifo.push_back(hidx as u8 - 9);
                            heap.boom_deque.push_back(hidx as u8 - 9);
                        }
                        else {
                            if 4 != cache.icons[cache.board[(hidx - 9) as usize] as usize].class { destroy_block(cache, world, hidx - 9); }
                        }
                    }
                    }
                }
                if 8 > hy {
                    if ICON_OPEN != cache.board[(hidx + 9) as usize] {
                        if !cache.icons[cache.board[(hidx + 9) as usize] as usize].animating {
                            if 5 == cache.icons[cache.board[(hidx + 9) as usize] as usize].class  {
                                fifo.push_back(hidx as u8 + 9);
                                heap.boom_deque.push_back(hidx as u8 + 9);
                            }
                            else {
                                if 4 != cache.icons[cache.board[(hidx +9) as usize] as usize].class { destroy_block(cache, world, hidx +9); }
                            }
                        }
                    }
                }
                if 0 < hx {
                    if ICON_OPEN != cache.board[(hidx - 1) as usize] {
                        if !cache.icons[cache.board[(hidx - 1) as usize] as usize].animating {
                            if 5 == cache.icons[cache.board[(hidx - 1) as usize] as usize].class  {
                                fifo.push_back(hidx as u8 - 1);
                                heap.boom_deque.push_back(hidx as u8 - 1);
                            }
                            else {
                                if 4 != cache.icons[cache.board[(hidx - 1) as usize] as usize].class { destroy_block(cache, world, hidx - 1); }
                            }
                        }
                    }
                }
                if 8 > hx {
                    if ICON_OPEN != cache.board[(hidx + 1) as usize] {
                        if !cache.icons[cache.board[(hidx + 1) as usize] as usize].animating {
                            if 5 == cache.icons[cache.board[(hidx +1) as usize] as usize].class  {
                                fifo.push_back(hidx as u8 + 1);
                                heap.boom_deque.push_back(hidx as u8 + 1);
                            }
                            else {
                                if 4 != cache.icons[cache.board[(hidx + 1) as usize] as usize].class { destroy_block(cache, world, hidx + 1); }
                            }
                        }
                    }
                }
                
                destroy_block(cache, world, hidx);
            }
        }
    }
    if !cache.ready {
        world.entity_set_visibility(91, false);
        cache.ready = true;
    }

    false
}

pub fn shutdown(_cache: &mut GameData, heap: &mut GameDataHeap) {
    // deallocate and overwrite existing memory
    *heap = GameDataHeap::default();
    
    // re-box and consume
    //let _temp = unsafe { Box::from_raw(cache.heap) };
}
// 814