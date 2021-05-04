use crate::mgfw;

#[derive(Default)]
pub struct GameDataHeap {
    // WARNING: Anything below this line is not in cache!
    _temp: i32,
}

pub struct GameData {
    pub heap: *mut GameDataHeap,
    board_xyv: [f32; 18],
    pscore: i32,
    cscore: i32,
    pulse: f32,
    frame: u8,
    board_state: [u8; 9],
    turn: u8,
    refresh_board: bool,
    refresh_score: bool,
    game_over: bool,
    animating: bool,
    winner: u8,
}

const ENUM_NONE: u8 = 0;
const ENUM_PLAYER: u8 = 1;
const ENUM_COMPUTER: u8 = 2;

#[rustfmt::skip]
pub fn initialize(cache: &mut GameData, _heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) {
    world.parse_world("assets/world.dat");

    // initialize board positions
    for row in 0..3 {
        for col in 0..3 {
            let pos = mgfw::ecs::Position {x: 40.0 + 88.0 * col as f32, y: 104.0 + 88.0 * row as f32 };
            let idx = row * 3 + col;
            cache.board_xyv[idx * 2 + 0] = pos.x;
            cache.board_xyv[idx * 2 + 1] = pos.y;
        }
    }

    // clear the board
    reset_board(cache);
    
    cache.turn = ENUM_COMPUTER;
    if mgfw::rnd() < 0.5 { cache.turn = ENUM_PLAYER; }
}

#[rustfmt::skip]
pub fn update(cache: &mut GameData, _heap: &mut GameDataHeap, world: &mut mgfw::ecs::World) -> bool {
    let mut expect_blown = false;

    cache.frame = (cache.frame + 1) % 128;

    if cache.refresh_board {
        hide_pieces(world);
        for idx in 0..9 {
            let x = cache.board_xyv[idx * 2 + 0];
            let y = cache.board_xyv[idx * 2 + 1];
            match cache.board_state[idx] {
                ENUM_PLAYER => place_piece(idx, 0, x, y, world),
                ENUM_COMPUTER => place_piece(idx, 1, x, y, world),
                _ => (),
            }
        }
        cache.refresh_board = false;
    }
    
    if cache.game_over {
        if !cache.animating {
            for idx in 0..18 {
                let wid = 5 + idx;                
                let yacc = (128.0 + mgfw::rnd() * 64.0) * 12.0;
                let xv = 128.0 * (mgfw::rnd() - 0.5) * 2.0;
                let yv = -128.0 * 2.0 * (1.0 + mgfw::rnd());
                let av = mgfw::deg2rad(xv);
                world.entity_set_angular_velocity(wid, av);
                world.entity_set_angular_velocity(wid + 18, av);
                world.entity_set_velocity(wid, mgfw::ecs::Velocity { x: xv, y: yv });
                world.entity_set_velocity(wid + 18, mgfw::ecs::Velocity { x: xv, y: yv });
                world.entity_set_acceleration(wid, mgfw::ecs::Acceleration{ x: 0.0, y: yacc });
                world.entity_set_acceleration(wid + 18, mgfw::ecs::Acceleration{ x: 0.0, y: yacc });
            }
            match cache.winner {
                ENUM_NONE => {
                    world.entity_set_color_rgba(42, 0.549, 0.549, 0.549, 1.0);
                    world.entity_set_text(44, String::from("DRAW!"));
                }
                ENUM_COMPUTER => {
                    world.entity_set_color_rgba(42, 0.878, 0.549, 0.125, 1.0);
                    world.entity_set_text(44, String::from("COMPUTER WINS!"));
                }
                ENUM_PLAYER => {
                    world.entity_set_color_rgba(42, 0.094, 0.780, 0.643, 1.0);
                    world.entity_set_text(44, String::from("PLAYER WINS!"));
                }
                _ => (),
            }
            world.entity_set_position_xy(44, 128.0 - 0.5 * world.text_get_width(44) as f32, 178.0);
            world.entity_set_visibility(42, true);
            world.entity_set_visibility(43, true);
            world.entity_set_visibility(44, true);
            world.entity_set_alpha_ease(42, 0.5, 0.0, 0.5);
            world.entity_set_alpha_ease(43, 1.0, 0.0, 0.5);
            world.entity_set_alpha_ease(44, 1.0, 0.0, 0.5);
            cache.animating = true;
            expect_blown = true;
        }
        else {
            let mut clear = true;
            for idx in 0..18 {
                if world.entity_get_position(5 + idx).y < 360.0 { clear = false; break; }
            }
            if clear {
                reset_board(cache);
                for idx in 0..36 {
                    world.entity_set_angle(5 + idx, 0.0);
                    world.entity_set_angular_velocity(5 + idx, 0.0);
                    world.entity_set_velocity(5 + idx, mgfw::ecs::Velocity { x: 0.0, y: 0.0 });
                    world.entity_set_acceleration(5 + idx, mgfw::ecs::Acceleration{ x: 0.0, y: 0.0 });
                    world.entity_set_alpha(5 + idx, 0.0);
                }
                cache.game_over = false;
                cache.animating = false;
            }
        }
    }

    // Amortize workload
    if 0 == cache.frame % 8 {
        if ENUM_COMPUTER == cache.turn { computer_turn(cache); }
        cache.pulse += 0.05;
        for idx in 0..9 {
            let rate = 0.1;
            let tgt = 0.80 + 0.1 * (cache.pulse + idx as f32).sin();
            let wtgt = 0.95 + 0.05 * (cache.pulse + idx as f32).cos();
    
            match cache.board_state[idx] {
                ENUM_PLAYER => {
                    let alpha = world.entity_get_alpha(5 + idx + 0 + 0);
                    world.entity_set_alpha(5 + idx + 0 + 0, alpha + (tgt - alpha) * rate);
                    world.entity_set_alpha(5 + idx + 0 + 18, alpha + (wtgt - alpha) * rate);
                }
                ENUM_COMPUTER => {
                    let alpha = world.entity_get_alpha(5 + idx + 9 + 0);
                    world.entity_set_alpha(5 + idx + 9 + 0, alpha + (tgt - alpha) * rate);
                    world.entity_set_alpha(5 + idx + 9 + 18, alpha + (wtgt - alpha) * rate);
                }
                _ => (),
            }
        }
    }

    if cache.refresh_score {
        world.entity_set_text(3, format!("{}", cache.pscore));
        world.entity_set_text(4, format!("{}", cache.cscore));
        world.entity_set_position_xy(3, 252.0 - world.text_get_width(3) as f32, 5.0);
        world.entity_set_position_xy(4, 252.0 - world.text_get_width(4) as f32, 30.0);
        expect_blown = true;
        cache.refresh_score = false;
    }
    expect_blown
}

#[rustfmt::skip]
pub fn event(cache: &mut GameData, _heap: &mut GameDataHeap, world: &mut mgfw::ecs::World, event_id: u8) -> bool {
    if ENUM_PLAYER != cache.turn { return false; } // consume event
    if cache.game_over { return false; } // no clicking during game over animation

    match event_id {
        mgfw::EVENT_INPUT_MOUSE_BUTTON_UP => {
            let mx = world.mouse_x as f32;
            let my = world.mouse_y as f32;
            for idx in 0..9 {
                if ENUM_NONE == cache.board_state[idx] &&
                    mx > cache.board_xyv[idx * 2 + 0] - 40.0 &&
                    mx < cache.board_xyv[idx * 2 + 0] + 40.0 &&
                    my > cache.board_xyv[idx * 2 + 1] - 40.0 &&
                    my < cache.board_xyv[idx * 2 + 1] + 40.0 {
                    cache.board_state[idx] = ENUM_PLAYER;
                    cache.refresh_board = true;
                    check_win(cache);
                }
            }
        }
        _ => (),
    }
    false
}

fn check_win(cache: &mut GameData) {
    cache.turn = 1 + (1 - (cache.turn - 1));
    if match_three(cache, ENUM_PLAYER) {
        cache.pscore += 1;
        cache.game_over = true;
        cache.winner = ENUM_PLAYER;
    } else if match_three(cache, ENUM_COMPUTER) {
        cache.cscore += 1;
        cache.game_over = true;
        cache.winner = ENUM_COMPUTER;
    } else if match_draw(cache) {
        cache.game_over = true;
        cache.winner = ENUM_NONE;
    }
}

#[rustfmt::skip]
fn match_three(cache: &mut GameData, value: u8) -> bool {
    for i in 0..3 {
        if (cache.board_state[i * 3 + 0] == value // row check
            && cache.board_state[i * 3 + 1] == value
            && cache.board_state[i * 3 + 2] == value)
            || (cache.board_state[i + 0] == value // column check
            && cache.board_state[i + 3] == value
            && cache.board_state[i + 6] == value)
        { return true; }
    }
    // cross check
    if cache.board_state[4] == value {
        if (cache.board_state[0] == value && cache.board_state[8] == value)
            || (cache.board_state[2] == value && cache.board_state[6] == value)
        { return true; }
    }
    false
}

#[rustfmt::skip]
fn match_draw(cache: &mut GameData) -> bool {
    for i in 0..9 {
        if ENUM_NONE == cache.board_state[i] { return false; }
    }
    true
}

#[rustfmt::skip]
fn computer_turn(cache: &mut GameData) {
    if cache.game_over { return; } // no clicking during game over animation
    // count the available options
    let mut num_options = 0;
    let mut block_option = 10; // option that will block player win
    let mut win_option = 10; // option that will result in immediate win
    let mut aggressive = false; // flag to limit aggressiveness
    if mgfw::rnd() < 0.5 { aggressive = true; }

    // look for available move options
    for idx in 0..9 {
        if ENUM_NONE == cache.board_state[idx] {
            num_options += 1;
            // check for computer win from this position
            cache.board_state[idx] = ENUM_COMPUTER;
            if match_three(cache, ENUM_COMPUTER) { win_option = idx; }
            // check for player win from this position
            cache.board_state[idx] = ENUM_PLAYER;
            if match_three(cache, ENUM_PLAYER) { block_option = idx; }
            // reset
            cache.board_state[idx] = ENUM_NONE;
        }
    }
    // pick winning move
    if 10 > win_option && aggressive {
        cache.board_state[win_option] = ENUM_COMPUTER;
        cache.refresh_board = true;
        check_win(cache);
        return;
    }
    // block player's win
    if 10 > block_option && aggressive {
        cache.board_state[block_option] = ENUM_COMPUTER;
        cache.refresh_board = true;
        check_win(cache);
        return;
    }
    // random move
    let mut choice = (mgfw::rnd() * num_options as f32).floor() as i32;
    for idx in 0..9 {
        if ENUM_NONE == cache.board_state[idx] {
            if 0 == choice {
                cache.board_state[idx] = ENUM_COMPUTER;
                cache.refresh_board = true;
                check_win(cache);
                return;
            } else {
                choice -= 1;
            }
        }
    }
}

fn reset_board(cache: &mut GameData) {
    for idx in 0..9 {
        cache.board_state[idx] = ENUM_NONE;
    }
    cache.refresh_board = true;
    cache.refresh_score = true;
}

fn hide_pieces(world: &mut mgfw::ecs::World) {
    for idx in 5..=44 {
        world.entity_set_visibility(idx, false);
    }
}

fn place_piece(idx: usize, player: u8, x: f32, y: f32, world: &mut mgfw::ecs::World) {
    let wid = 5 + (idx + player as usize * 9);
    world.entity_set_position_xy(wid, x, y);
    world.entity_set_visibility(wid, true);
    world.entity_set_position_xy(wid + 18, x, y);
    world.entity_set_visibility(wid + 18, true);
}

#[rustfmt::skip]
pub fn shutdown(cache: &mut GameData, _heap: &mut GameDataHeap) {
    // re-box and consume to deallocate memory
    let _temp = unsafe { Box::from_raw(cache.heap) };
}
