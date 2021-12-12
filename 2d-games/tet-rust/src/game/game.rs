use crate::mgfw;

#[derive(Default)]
pub struct GameDataHeap {
    // WARNING: Anything below this line is not in cache!
}

const BOARD_X: usize = 10;
const BOARD_Y: usize = 20;
const BOARD_SZ: usize = BOARD_X * BOARD_Y;

pub struct GameData {
    pub heap: *mut GameDataHeap,
    frame: u8,
    ready: bool,
    game_timer: f64,
    board: [usize; BOARD_SZ],
    board_entity_start: usize,
    level: usize,
    tetra_base: [usize; 7 * 4 * 4 * 4],
    piece: [usize; 4 * 4],
    cursor: usize,
    cursor_entity_start: usize,
    cursor_telegraph_start: usize,
    next_block: usize,
    curr_block: usize,
    block_rotation: usize,
    move_timer: f64,
    gen_block: bool,
    gen_block_timer: f64,
    row_counter_entity: usize,
    row_counter: usize,
    particle_entity_start: usize,
    game_over_entity_start: usize,
    level_up_timer: f64,
    level_up_lock: bool,
    game_over: bool,
    game_over_timer: f64,
    level_rows: usize,
}

#[rustfmt::skip]
pub fn initialize(cache: &mut GameData, _heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {

    world.parse_world("assets/world.dat");
    
    cache.level = 1;
    cache.frame = 0;
    cache.ready = false;
    cache.game_timer = 0.0;
    cache.move_timer = cache.game_timer + 0.1;
    cache.block_rotation = 0;
    cache.gen_block = false;
    cache.gen_block_timer = 0.0;
    cache.row_counter = 0;
    cache.level_up_timer = 0.0;
    cache.game_over_timer = 0.0;
    cache.game_over = false;
    cache.level_up_lock = false;
    cache.level_rows = 10;

    // create level progress bar entity
    cache.row_counter_entity = world.new_entity();
    world.entity_set_billboard(cache.row_counter_entity, String::from("assets/square-w.png"));
    world.entity_set_color_rgba(cache.row_counter_entity, 0.2, 1.0, 0.2, 1.0);
    world.entity_set_visibility(cache.row_counter_entity, true);
    
    // create game board square entities
    for y in 0..BOARD_Y {
        for x in 0..BOARD_X {
            let bidx: usize = y * BOARD_X + x;
            cache.board[bidx] = 0;
            let e = world.new_entity();
            if 0 == bidx {
                cache.board_entity_start = e;
            }
            world.entity_set_position(e, mgfw::ecs::Position { x: (24 + 16 * x) as f32, y: (72 + y * 16) as f32 });
            world.entity_set_scale(e, mgfw::ecs::Scale { x: 16.0, y: 16.0 });
        }
    }

    // create telegraph block entities
    for i in 0..5 {
        let e = world.new_entity();
        if 0 == i {
            cache.cursor_telegraph_start = e;
        }
        world.entity_set_scale(e, mgfw::ecs::Scale { x: 16.0, y: 16.0 });
        world.entity_set_billboard(e, String::from("assets/block-w.png"));
        world.entity_set_alpha(e, 0.4);
        world.entity_set_visibility(e, false);
    }

    // create cursor block entities
    for b in 0..7 {
        for i in 0..5 {
            let e = world.new_entity();
            if 0 == i && 0 == b{
                cache.cursor_entity_start = e;
            }
            world.entity_set_scale(e, mgfw::ecs::Scale { x: 16.0, y: 16.0 });
            world.entity_set_visibility(e, false);
            update_entity_block(world, e, b + 1);
        }
    }

    // reserve entities for exploding blocks when clearing rows
    for i in 0..40 {
        let e = world.new_entity();
        if 0 == i {
            cache.particle_entity_start = e;
        }
        world.entity_set_visibility(e, false);
        world.entity_set_scale(e, mgfw::ecs::Scale { x: 16.0, y: 16.0 });
    }

    // create entities for win/lose popup
    let e = world.new_entity();
    cache.game_over_entity_start = e;

    world.entity_set_billboard(e, String::from("assets/square-b.png"));
    world.entity_set_visibility(e, false);
    world.entity_set_position_xy(e, 96.0, 200.0);
    world.entity_set_scale_xy(e, 192.0, 400.0);
    world.entity_set_alpha(e, 0.8);

    let e = world.new_entity();
    world.entity_set_text(e, String::from("Level Up!"));
    world.entity_set_visibility(e, false);
    world.entity_set_position_xy(e, 96.0, 180.0);
    world.entity_set_scale_xy(e, 2.0, 2.0);
    world.entity_set_color_rgba(e, 0.5, 0.5, 1.0, 1.0);

    let e = world.new_entity();
    world.entity_set_text(e, String::from("Game Over!"));
    world.entity_set_visibility(e, false);
    world.entity_set_position_xy(e, 96.0, 160.0);
    world.entity_set_scale_xy(e, 2.0, 2.0);
    world.entity_set_color_rgba(e, 0.5, 0.5, 1.0, 1.0);

    let e = world.new_entity();
    world.entity_set_text(e, String::from("Press Space to Restart"));
    world.entity_set_visibility(e, false);
    world.entity_set_position_xy(e, 96.0, 200.0);
    world.entity_set_scale_xy(e, 1.0, 1.5);
    world.entity_set_color_rgba(e, 0.5, 0.5, 1.0, 1.0);

    // center win/gameover text
    for i in 0..3 {
        let idx = cache.game_over_entity_start + i + 1;
        let p = world.entity_get_position(idx);
        let w = world.text_get_width(idx);
        let s = world.entity_get_scale(idx);
        world.entity_set_position_xy(idx, p.x - w as f32 * 0.5 * s.x, p.y);
    }
    
    // update board entities
    update_board(cache, world);

    // define puzzle block shapes
    cache.tetra_base = [
        // rotation 0
        1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0,
        3, 0, 0, 0, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 3, 0, 3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 4, 0, 0, 4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        5, 5, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // rotation 1
        0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0,
        1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
        0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0,
        0, 3, 3, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0,
        0, 3, 0, 0, 0, 3, 0, 0, 0, 3, 3, 0, 0, 0, 0, 0,
        0, 4, 0, 0, 0, 4, 4, 0, 0, 4, 0, 0, 0, 0, 0, 0,
        5, 5, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // rotation 2
        0, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0,
        0, 1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 3, 3, 3, 0, 0, 0, 3, 0, 0, 0, 0, 0,        
        0, 0, 0, 0, 3, 3, 3, 0, 3, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 4, 4, 4, 0, 0, 4, 0, 0, 0, 0, 0, 0, 
        5, 5, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // rotation 3
        0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0,
        1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
        0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0, 0, 2, 0, 0,
        0, 3, 0, 0, 0, 3, 0, 0, 3, 3, 0, 0, 0, 0, 0, 0,
        3, 3, 0, 0, 0, 3, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0,
        0, 4, 0, 0, 4, 4, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0,
        5, 5, 0, 0, 5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    // create first falling block and update ui for next block
    init_cursor(cache, world, (mgfw::rnd() * 7.0).floor() as usize);
    select_next_block(cache, world);
}

pub fn check_clear(cache: &mut GameData, world: &mut mgfw::ecs::World) {
    let mut e = cache.particle_entity_start;

    // for each board line, check if line has been filled
    for y in 0..BOARD_Y {
        let mut skip = false;
        for x in 0..BOARD_X {
            let idx = y * BOARD_X + x;
            if 0 == cache.board[idx] {
                skip = true;
                break;
            }
        }

        // line found, create particles and slide down prior rows
        if !skip {
            for x in 0..BOARD_X {
                let idx = y * BOARD_X + x;
                update_entity_block(world, e, cache.board[idx]);
                world.entity_set_visibility(e, true);
                world.entity_set_position(
                    e,
                    mgfw::ecs::Position {
                        x: (24 + 16 * x) as f32,
                        y: (72 + y * 16) as f32,
                    },
                );
                world.entity_set_velocity_xy(e, 100.0 * (mgfw::rnd() - 0.5), -100.0);
                world.entity_set_acceleration_xy(e, 0.0, 800.0);
                world.entity_set_alpha(e, 0.5);
                e += 1;
            }
            erase_row(cache, y);
            hide_cursor(cache, world);

            cache.row_counter = cache.row_counter + 1;
            update_progressbar(cache, world);
            
            if cache.level_rows <= cache.row_counter {
                cache.level_up_timer = cache.game_timer + 3.0;
                cache.level += 1;
                cache.level_up_lock = true;
                return;
            }
        }
    }
}

// check if the position in cache overlaps the board edges or another block
pub fn check_constrained(cache: &mut GameData) -> bool {
    if check_constrained_lr(cache) {
        return true;
    }

    let loc = get_cursor_location(cache);

    for y in 0..4 {
        for x in 0..4 {
            let pidx = y * 4 + x;
            if 0 != cache.piece[pidx] {
                let my: i8 = (loc.1 + y) as i8 - 1;
                if 18 < my {
                    return true;
                }
            }
        }
    }

    false
}

// check if the position in cache overlaps the board or another block to the left and right
pub fn check_constrained_lr(cache: &mut GameData) -> bool {
    let loc = get_cursor_location(cache);

    for y in 0..4 {
        for x in 0..4 {
            let pidx = y * 4 + x;
            if 0 != cache.piece[pidx] {
                let mx: i8 = (loc.0 + x) as i8 - 1;
                let my: i8 = (loc.1 + y) as i8 - 1;
                if 0 > mx || 9 < mx {
                    return true;
                }
                let idx = my as usize * BOARD_X + mx as usize;
                if 0 != cache.board[idx] {
                    return true;
                }
            }
        }
    }

    false
}

pub fn check_stick(cache: &mut GameData, world: &mut mgfw::ecs::World) -> bool {
    let loc = get_cursor_location(cache);
    let mut ret = false;

    // is ther another block below us? if so, the piece is stuck
    if check_constrained(cache) || 0 == get_collide_depth(cache) {
        ret = true;

        // copy cursor block to board
        for y in 0..4 {
            for x in 0..4 {
                let pidx = y * 4 + x;
                if 0 != cache.piece[pidx] {
                    let cidx = (loc.1 + y - 1) * BOARD_X + (loc.0 + x - 1);
                    cache.board[cidx] = cache.curr_block + 1;
                }
            }
        }

        // check if any rows can be cleared
        check_clear(cache, world);
        update_board(cache, world);

        // generate the next block
        cache.gen_block = true;
        cache.gen_block_timer = cache.game_timer + 0.3;

        // are we stuck at the top of the board? if so, game over
        if 1 == loc.1 {
            cache.game_over = true;
            cache.game_over_timer = cache.game_timer + 1.0;
        }
    }

    ret
}

// empty the board and update the board's entities
pub fn clear_board(cache: &mut GameData, world: &mut mgfw::ecs::World) {
    for i in 0..BOARD_SZ {
        cache.board[i] = 0;
    }
    update_board(cache, world);
}

// clear a row from the board
pub fn erase_row(cache: &mut GameData, row: usize) {
    for x in 0..BOARD_X {
        for y in 0..BOARD_Y {
            let yy: i8 = row as i8 - y as i8 - 1;
            if 0 <= yy {
                let yidx = yy as usize * BOARD_X + x;
                cache.board[yidx + BOARD_X] = cache.board[yidx];
            }
        }
    }
}

// this gets called by MGFW with input events
#[rustfmt::skip]
pub fn event(cache: &mut GameData, _heap: &mut GameDataHeap, world: &mut mgfw::ecs::World, event_id: u8) -> bool {

    // press spacebar to reset if game-over
    if cache.game_over {
        match event_id {
            mgfw::EVENT_INPUT_KEYBOARD_PRESSED_SPACE => game_reset(cache, world),
            _ => ()
        }
        return false;
    }

    // waiting for block to generate, skip input
    if cache.gen_block || cache.level_up_lock {
        return false;
    }

    // normal input to move/rotate block
    match event_id {
        mgfw::EVENT_INPUT_KEYBOARD_PRESSED_LEFT => move_cursor_left(cache),
        mgfw::EVENT_INPUT_KEYBOARD_PRESSED_RIGHT => move_cursor_right(cache),
        mgfw::EVENT_INPUT_KEYBOARD_PRESSED_DOWN => move_cursor_down(cache),
        mgfw::EVENT_INPUT_KEYBOARD_PRESSED_SPACE => rotate_cursor(cache),
        _ => ()
    }
    update_cursor_entities(cache, world);
    update_telegraph_entities(cache, world);

    // check if our new location/orientation caused us to get stick
    check_stick(cache, world);
    true
}

// reset game state on game-over
pub fn game_reset(cache: &mut GameData, world: &mut mgfw::ecs::World) {
    // wait a little bit of time for input events to clear to keep the game-over popup open
    if cache.game_over_timer < cache.game_timer {
        cache.level = 1;
        world.entity_set_text(1, format!("Level: {}", cache.level));
        world.entity_set_color_rgba(0, 1.0, 1.0, 1.0, 1.0);
        clear_board(cache, world);
        init_cursor(cache, world, (mgfw::rnd() * 7.0).floor() as usize);
        select_next_block(cache, world);
        world.entity_set_visibility(cache.game_over_entity_start + 0, false);
        world.entity_set_visibility(cache.game_over_entity_start + 2, false);
        world.entity_set_visibility(cache.game_over_entity_start + 3, false);
        cache.game_over = false;
        cache.row_counter = 0;
        cache.level_rows = 10;
        update_progressbar(cache, world);
    }
}

// how much room is there between the cursor block and the bocks below it
pub fn get_collide_depth(cache: &mut GameData) -> usize {
    let mut ret: usize = BOARD_Y;
    let mut bot: [i8; 4] = [-(BOARD_Y as i8); 4];
    let loc = get_cursor_location(cache);

    for x in 0..4 {
        for y in 0..4 {
            let pidx = y * 4 + x;
            if 0 != cache.piece[pidx] {
                bot[x] = y as i8;
            }
        }
    }

    let mut top: [usize; 4] = [BOARD_Y; 4];

    for x in 0..4 {
        let xx: i8 = (loc.0 + x) as i8 - 1;
        if 0 <= xx && 9 >= xx {
            for y in loc.1..BOARD_Y {
                let idx = y * BOARD_X + xx as usize;
                if 0 != cache.board[idx] {
                    top[x] = y;
                    break;
                }
            }
        }
    }

    let mut mn: i8 = BOARD_Y as i8;
    for x in 0..4 {
        let delta: i8 = top[x] as i8 - (loc.1 as i8 + bot[x]);
        if mn > delta {
            mn = delta;
        }
    }
    if 0 <= mn {
        ret = mn as usize;
    }

    ret
}

// conver cursor index to x/y coordinates
pub fn get_cursor_location(cache: &GameData) -> (usize, usize) {
    let cx = cache.cursor % BOARD_X;
    let cy = (cache.cursor - cx) / BOARD_X;
    (cx, cy)
}

// hide the cursor, used on game win/lose
pub fn hide_cursor(cache: &mut GameData, world: &mut mgfw::ecs::World) {
    for i in 0..5 * 7 {
        let e = cache.cursor_entity_start + i;
        world.entity_set_visibility(e, false);
    }
}

// reset the cursor to the top of the board using the given block pattern
pub fn init_cursor(cache: &mut GameData, world: &mut mgfw::ecs::World, pattern: usize) {
    cache.curr_block = pattern;
    cache.block_rotation = 0;
    cache.cursor = 15;
    cache.move_timer = cache.game_timer + 1.0;
    let start: usize = 16 * pattern;
    let mut j: usize = 0;
    // there are multiple entities for cursors, one of each pattern, this hides all first
    hide_cursor(cache, world);
    // then this shows the one we care about
    for i in 0..16 {
        cache.piece[i] = cache.tetra_base[start + i];
        if 0 != cache.piece[i] {
            let e = cache.cursor_entity_start + j + 5 * cache.curr_block;
            world.entity_set_visibility(e, true);
            j = j + 1;
        }
    }
    update_cursor_entities(cache, world);
    update_telegraph_entities(cache, world);
}

pub fn move_cursor_down(cache: &mut GameData) {
    if 0 < get_collide_depth(cache) {
        cache.cursor = cache.cursor + BOARD_X;
    }
}

pub fn move_cursor_left(cache: &mut GameData) {
    let temp = cache.cursor;
    let cx = cache.cursor % BOARD_X;

    if 0 < cx {
        cache.cursor = cache.cursor - 1;
        if check_constrained_lr(cache) {
            cache.cursor = temp;
        }
    }
}

pub fn move_cursor_right(cache: &mut GameData) {
    let temp = cache.cursor;
    let cx = cache.cursor % BOARD_X;

    if 9 > cx {
        cache.cursor = cache.cursor + 1;
        if check_constrained_lr(cache) {
            cache.cursor = temp;
        }
    }
}

pub fn rotate_cursor(cache: &mut GameData) {
    let temp = cache.piece;
    cache.block_rotation = (cache.block_rotation + 1) % 4;

    // tetra_base holds the puzzle piece pattern at each orientation
    // rotating the piece just adds an offset in tetra_base
    for y in 0..4 {
        for x in 0..4 {
            let idx = y * 4 + x;
            let pdx = cache.curr_block * 16 + 7 * 16 * cache.block_rotation + idx;
            cache.piece[idx] = cache.tetra_base[pdx];
        }
    }

    // if rotation would break a constraint, undo the rotation
    if check_constrained(cache) {
        cache.piece = temp;
    }
}

// determine what the next block will be and update the ui
pub fn select_next_block(cache: &mut GameData, world: &mut mgfw::ecs::World) {
    loop {
        cache.next_block = (mgfw::rnd() * 7.0).floor() as usize;
        if cache.curr_block == 0 || cache.curr_block == 1 {
            if cache.next_block != 0 && cache.next_block != 1 {
                break;
            }
            continue;
        }
        if cache.next_block != cache.curr_block {
            break;
        }
    }
    for i in 0..7 {
        world.entity_set_visibility(3 + i, false);

        if cache.row_counter < cache.level_rows {
            if i == cache.next_block {
                world.entity_set_visibility(3 + i, true);
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

    if !cache.ready {
        if cache.frame == 127 {
            cache.ready = true;
        }
        return false;
    }

    // Amortize workload
    if 0 == cache.frame {
        if !cache.level_up_lock && !cache.game_over {
            if !cache.gen_block && !check_stick(cache, world) {
                if cache.move_timer < cache.game_timer {
                    cache.cursor += BOARD_X;
                    cache.move_timer = cache.game_timer + 1.0 * f64::powf(0.95, cache.level as f64);
                    update_cursor_entities(cache, world);
                    update_telegraph_entities(cache, world);
                    expect_blown = true;
                }
            }
        }
    }
    else if 32 == cache.frame {
        if cache.level_up_lock {
            world.entity_set_visibility(cache.game_over_entity_start + 0, true);
            world.entity_set_visibility(cache.game_over_entity_start + 1, true);

            if cache.level_up_timer < cache.game_timer {
                cache.level_up_lock = false;
                world.entity_set_visibility(cache.game_over_entity_start + 0, false);
                world.entity_set_visibility(cache.game_over_entity_start + 1, false);
                world.entity_set_text(1, format!("Level: {}", cache.level));
                world.entity_set_color_rgba(0, 0.5 + mgfw::rnd() * 0.5, 0.5 + mgfw::rnd() * 0.5, 0.5 + mgfw::rnd() * 0.5, 1.0);
                clear_board(cache, world);
                init_cursor(cache, world, (mgfw::rnd() * 7.0).floor() as usize);
                select_next_block(cache, world);
                cache.level_rows = 10 + cache.level;
                cache.row_counter = 0;
                update_progressbar(cache, world);
            }
        }
        if cache.game_over {
            world.entity_set_visibility(cache.game_over_entity_start + 0, true);
            world.entity_set_visibility(cache.game_over_entity_start + 2, true);
            world.entity_set_visibility(cache.game_over_entity_start + 3, true);
        }
    }
    else if 64 == cache.frame {
        if !cache.level_up_lock && !cache.game_over {
            if cache.gen_block && cache.gen_block_timer < cache.game_timer {
                init_cursor(cache, world, cache.next_block);
                select_next_block(cache, world);
                cache.gen_block = false;
                expect_blown = true;
            }
        }
    }

    cache.game_timer += 833.0e-6;
    
    expect_blown
}

// update the board cell's entities
pub fn update_board(cache: &mut GameData, world: &mut mgfw::ecs::World) {
    for i in 0..BOARD_SZ {
        let e: usize = cache.board_entity_start + i;
        let b: usize = cache.board[i];
        if 0 == b {
            world.entity_set_visibility(e, false);
        } else {
            update_entity_block(world, e, b);
            world.entity_set_visibility(e, true);
        }
    }
}

// update the entities for the cursor
pub fn update_cursor_entities(cache: &mut GameData, world: &mut mgfw::ecs::World) {
    let mut j: usize = 0;
    let loc = get_cursor_location(cache);
    let cx: f32 = loc.0 as f32 * 16.0 + 8.0;
    let cy: f32 = loc.1 as f32 * 16.0 + 56.0;

    for i in 0..16 {
        if 0 != cache.piece[i] {
            let e = cache.cursor_entity_start + j + 5 * cache.curr_block;
            let ex: f32 = (i % 4) as f32 * 16.0 + cx;
            let ey: f32 = ((i - (i % 4)) / 4) as f32 * 16.0 + cy;
            world.entity_set_position_xy(e, ex, ey);
            let t = cache.cursor_telegraph_start + j;
            world.entity_set_position_xy(t, ex, ey);
            j = j + 1;
        }
    }
}

// updated the entity billboard with the specified image
pub fn update_entity_block(world: &mut mgfw::ecs::World, entity: usize, block: usize) {
    match block {
        1 => world.entity_set_billboard(entity, String::from("assets/block-1.png")),
        2 => world.entity_set_billboard(entity, String::from("assets/block-1.png")),
        3 => world.entity_set_billboard(entity, String::from("assets/block-2.png")),
        4 => world.entity_set_billboard(entity, String::from("assets/block-3.png")),
        5 => world.entity_set_billboard(entity, String::from("assets/block-3.png")),
        6 => world.entity_set_billboard(entity, String::from("assets/block-4.png")),
        7 => world.entity_set_billboard(entity, String::from("assets/block-5.png")),
        _ => (),
    }
}

// update the progress bar entity using the current row cleared counter
pub fn update_progressbar(cache: &mut GameData, world: &mut mgfw::ecs::World) {
    if cache.row_counter > cache.level_rows {
        cache.row_counter = cache.level_rows;
    }
    let xs: f32 = 54.0 * cache.row_counter as f32 / (cache.level_rows as f32);
    world.entity_set_position_xy(cache.row_counter_entity, 25.0 + xs * 0.5, 42.0);
    world.entity_set_scale_xy(cache.row_counter_entity, xs, 3.0);            
}

// update the telegraph block at the bottom showing where the piece would land if dropped
pub fn update_telegraph_entities(cache: &mut GameData, world: &mut mgfw::ecs::World) {
    let depth = get_collide_depth(cache);

    for i in 0..5 {
        let t = cache.cursor_telegraph_start + i;
        world.entity_set_visibility(t, false);
    }

    if 0 == depth {
        return;
    }

    let mut j: usize = 0;
    for i in 0..16 {
        if 0 != cache.piece[i] {
            let cc = cache.cursor + depth * BOARD_X;
            let cx: f32 = (cc % BOARD_X) as f32 * 16.0 + 8.0;
            let cy: f32 = ((cc - (cc % BOARD_X)) / BOARD_X) as f32 * 16.0 + 56.0;
            let ex: f32 = (i % 4) as f32 * 16.0 + cx;
            let ey: f32 = ((i - (i % 4)) / 4) as f32 * 16.0 + cy;
            let t = cache.cursor_telegraph_start + j;
            world.entity_set_position_xy(t, ex, ey);
            world.entity_set_visibility(t, true);
            j = j + 1;
        }
    }
}
