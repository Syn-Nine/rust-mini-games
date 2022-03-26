use crate::mgfw;
use std::process::exit;
use crate::game::game;
use crate::game::track;

pub struct MenuData {
    timer: f64,
    cursor_ent: usize,
    pub menu: usize,
    selection: usize,
    upgrade_start_ent: usize,
    black_ent: usize,
    transition_lock: bool,
    transition_target: usize,
    transition_timer: f64,
    upgrade_available: bool,
    track_geometry_start_ent: usize,
    track_name_ent: usize,
    track_center: [track::Point; 10],
    num_options: usize,
}

pub const MENU_MAIN: usize = 0;
pub const MENU_RESULTS: usize = 1;
pub const MENU_PLAYING: usize = 2;
pub const MENU_UPGRADE: usize = 3;
pub const MENU_TRACK: usize = 4;

#[rustfmt::skip]
pub fn initialize(cache: &mut game::GameData, heap: &mut game::GameDataHeap, world: &mut mgfw::ecs::World) {
    build_menu_main(cache, world);
}

pub fn build_menu_track(cache: &mut game::GameData, heap: &mut game::GameDataHeap, world: &mut mgfw::ecs::World) {

    cache.menu_data.menu = MENU_TRACK;
    cache.menu_data.selection = cache.track_data.cur_track;

    world.clear();
    world.parse_world("assets/world.dat");

    for e in 2..mgfw::ecs::entity::ENTITY_SZ {
        world.entity_set_visibility(e, false);
    }

    world.entity_set_position_xy(0, 320.0, 132.0);
    world.entity_set_position_xy(1, 320.0, 132.0);


    let e = world.new_entity();
    world.entity_set_text(e, String::from("Select Track"));
    world.entity_set_position_xy(e, 320.0 - world.text_get_width(e) as f32 * 0.5 * 3.0, 100.0);
    world.entity_set_scale_xy(e, 3.0, 2.0);
    world.entity_set_visibility(e, true);

    let e = world.new_entity();
    cache.menu_data.track_name_ent = e;
    world.entity_set_text(e, String::from(""));
    world.entity_set_scale_xy(e, 2.0, 1.5);
    world.entity_set_visibility(e, true);

    let e = world.new_entity();
    world.entity_set_text(e, String::from("<<                             >>"));
    world.entity_set_position_xy(e, 320.0 - world.text_get_width(e) as f32 * 0.5 * 2.0, 200.0);
    world.entity_set_scale_xy(e, 2.0, 1.5);
    world.entity_set_visibility(e, true);

    for i in 0..10 {

        let mut p: Vec<mgfw::ecs::Position> = Vec::new();
        let mut c: Vec<mgfw::ecs::Color> = Vec::new();
        let mut center = track::Point { x: 0.0, y: 0.0 };

        track::get_track_geometry(&heap.track_ref[i], &mut p, &mut c, &mut center);

        cache.menu_data.track_center[i] = center;
        
        let e = world.new_entity();
        if 0 == i { cache.menu_data.track_geometry_start_ent = e; }
        
        world.entity_set_line_buffer(e, &p, &c);
        world.entity_set_visibility(e, true);

    }

    let e = world.new_entity();
    cache.menu_data.black_ent = e;
    world.entity_set_billboard(e, String::from("assets/square-b.png"));
    world.entity_set_position_xy(e, 320.0, 192.0);
    world.entity_set_scale_xy(e, 640.0, 384.0);
    world.entity_set_visibility(e, false);
}

pub fn build_menu_main(cache: &mut game::GameData, world: &mut mgfw::ecs::World) {

    cache.menu_data.menu = MENU_MAIN;
    cache.menu_data.selection = 0;

    world.clear();
    world.parse_world("assets/world.dat");

    for e in 2..mgfw::ecs::entity::ENTITY_SZ {
        world.entity_set_visibility(e, false);
    }

    world.entity_set_position_xy(0, 320.0, 132.0);
    world.entity_set_position_xy(1, 320.0, 132.0);

    let mut yy = 150.0;

    cache.menu_data.num_options = 2;

    let e = world.new_entity();
    world.entity_set_text(e, String::from("New Game"));
    world.entity_set_position_xy(e, 230.0, yy);
    world.entity_set_scale_xy(e, 3.0, 2.0);
    world.entity_set_visibility(e, true);

    if 0 != cache.track_data.player.thrust && !cache.track_data.dead {
        cache.menu_data.selection = 1;
        cache.menu_data.num_options = 3;
        yy += 30.0;
        let e = world.new_entity();
        world.entity_set_text(e, String::from("Continue"));
        world.entity_set_position_xy(e, 230.0, yy);
        world.entity_set_scale_xy(e, 3.0, 2.0);
        world.entity_set_visibility(e, true);
    }

    yy += 30.0;
    let e = world.new_entity();
    world.entity_set_text(e, String::from("Exit"));
    world.entity_set_position_xy(e, 230.0, yy);
    world.entity_set_scale_xy(e, 3.0, 2.0);
    world.entity_set_visibility(e, true);

    let e = world.new_entity();
    world.entity_set_text(e, String::from("Controls:"));
    world.entity_set_position_xy(e, 230.0, 250.0);
    world.entity_set_scale_xy(e, 3.0, 2.0);
    world.entity_set_visibility(e, true);
    world.entity_set_color_rgba(e, 0.2, 0.2, 0.3, 1.0);

    let mut control: Vec<&str> = Vec::new();
    control.push("UP:");
    control.push("LEFT/RIGHT:");
    control.push("SPACE:");
    control.push("ESC:");

    let mut val: Vec<&str> = Vec::new();
    val.push("Throttle");
    val.push("Steering");
    val.push("Confirm");
    val.push("Quit Race");
    
    for i in 0..4 {
        let e = world.new_entity();
        world.entity_set_text(e, String::from(control[i]));
        world.entity_set_position_xy(e, 240.0, 280.0 + 16.0 * i as f32);
        world.entity_set_scale_xy(e, 1.0, 1.0);
        world.entity_set_visibility(e, true);
        world.entity_set_color_rgba(e, 0.3, 0.2, 0.5, 1.0);

        let e = world.new_entity();
        world.entity_set_text(e, String::from(val[i]));
        world.entity_set_position_xy(e, 400.0 - world.text_get_width(e) as f32, 280.0 + 16.0 * i as f32);
        world.entity_set_scale_xy(e, 1.0, 1.0);
        world.entity_set_visibility(e, true);
        world.entity_set_color_rgba(e, 0.5, 0.2, 0.3, 1.0);
    }

    let mut p: Vec<mgfw::ecs::Position> = Vec::new();
    let mut c: Vec<mgfw::ecs::Color> = Vec::new();

    let r = 0.5;
    let pr = 0.0;//mgfw::PI as f32 * -0.5;
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
    
    let e = world.new_entity();
    cache.menu_data.cursor_ent = e;
    cache.menu_data.timer = 0.0;

    world.entity_set_position_xy(e, 215.0 + 5.0 * (cache.menu_data.timer * 10.0).sin() as f32, 165.0);
    world.entity_set_scale_xy(e, 32.0, 32.0);
    world.entity_set_line_buffer(e, &p, &c);
    world.entity_set_visibility(e, true);

    let e = world.new_entity();
    world.entity_set_billboard(e, String::from("assets/vracer-logo.png"));
    world.entity_set_position_xy(e, 320.0, 96.0);
    world.entity_set_scale_xy(e, 640.0, 168.0);
    world.entity_set_visibility(e, true);


    let e = world.new_entity();
    cache.menu_data.black_ent = e;
    world.entity_set_billboard(e, String::from("assets/square-b.png"));
    world.entity_set_position_xy(e, 320.0, 192.0);
    world.entity_set_scale_xy(e, 640.0, 384.0);
    world.entity_set_visibility(e, false);
}

pub fn build_menu_results(cache: &mut game::GameData, world: &mut mgfw::ecs::World) {

    cache.menu_data.menu = MENU_RESULTS;
    cache.menu_data.selection = 0;

    world.clear();
    world.parse_world("assets/world.dat");

    for e in 2..mgfw::ecs::entity::ENTITY_SZ {
        world.entity_set_visibility(e, false);
    }

    world.entity_set_position_xy(0, 320.0, 132.0);
    world.entity_set_position_xy(1, 320.0, 132.0);


    let e = world.new_entity();
    world.entity_set_text(e, String::from("\\\\ RACE OVER //"));
    world.entity_set_position_xy(e, 320.0 - world.text_get_width(e) as f32 * 0.5 * 3.0, 100.0);
    world.entity_set_scale_xy(e, 3.0, 2.0);
    world.entity_set_visibility(e, true);

    let e = world.new_entity();
    world.entity_set_text(e, String::from("Track:"));
    world.entity_set_position_xy(e, 170.0 - world.text_get_width(e) as f32 * 0.5 * 2.0, 160.0);
    world.entity_set_scale_xy(e, 2.0, 2.0);
    world.entity_set_visibility(e, true);

    let e = world.new_entity();
    world.entity_set_text(e, format!("{}", match cache.track_data.cur_track {
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
    world.entity_set_position_xy(e, 170.0 - world.text_get_width(e) as f32 * 0.5, 190.0);
    world.entity_set_scale_xy(e, 1.0, 1.0);
    world.entity_set_visibility(e, true);

    let e = world.new_entity();
    world.entity_set_text(e, String::from("Place:"));
    world.entity_set_position_xy(e, 320.0 - world.text_get_width(e) as f32 * 0.5 * 2.0, 160.0);
    world.entity_set_scale_xy(e, 2.0, 2.0);
    world.entity_set_visibility(e, true);
    
    let e = world.new_entity();
    world.entity_set_text(e, format!("#{}", cache.track_data.player.place));
    world.entity_set_position_xy(e, 320.0 - world.text_get_width(e) as f32 * 0.5, 190.0);
    world.entity_set_scale_xy(e, 1.0, 1.0);
    world.entity_set_visibility(e, true);

    let e = world.new_entity();
    world.entity_set_text(e, String::from("Best Lap:"));
    world.entity_set_position_xy(e, 470.0 - world.text_get_width(e) as f32 * 0.5 * 2.0, 160.0);
    world.entity_set_scale_xy(e, 2.0, 2.0);
    world.entity_set_visibility(e, true);


    let minutes = (cache.track_data.player.best_timer / 60.0).floor();
    let mut mpad = "0";
    if minutes >= 10.0 { mpad = ""; }
    let seconds = (cache.track_data.player.best_timer - minutes * 60.0).floor();
    let mut spad = "0";
    if seconds >= 10.0 { spad = ""; }
    
    let e = world.new_entity();
    world.entity_set_text(e, format!("{}{}:{}{}", mpad, minutes, spad, seconds));
    world.entity_set_position_xy(e, 470.0 - world.text_get_width(e) as f32 * 0.5, 190.0);
    world.entity_set_scale_xy(e, 1.0, 1.0);
    world.entity_set_visibility(e, true);

    cache.menu_data.upgrade_available = false;
    let sum = (cache.track_data.player.thrust + cache.track_data.player.steering + cache.track_data.player.shield) as usize;
    if sum < (cache.track_data.cur_track + 2) * 3 && 1 == cache.track_data.player.place {
        if 30 > cache.track_data.player.thrust + cache.track_data.player.steering + cache.track_data.player.shield { cache.menu_data.upgrade_available = true; }
    }

    if cache.menu_data.upgrade_available {
        let e = world.new_entity();
        world.entity_set_text(e, String::from("Ship upgrade available!"));
        world.entity_set_position_xy(e, 320.0 - world.text_get_width(e) as f32 * 0.5, 260.0);
        world.entity_set_scale_xy(e, 1.0, 1.0);
        world.entity_set_visibility(e, true);
        world.entity_set_color_rgba(e, 0.9, 0.5, 0.2, 1.0);
    }

    let e = world.new_entity();
    world.entity_set_text(e, String::from("Press SPACE to continue"));
    world.entity_set_position_xy(e, 320.0 - world.text_get_width(e) as f32 * 0.5, 290.0);
    world.entity_set_scale_xy(e, 1.0, 1.0);
    world.entity_set_visibility(e, true);
    world.entity_set_color_rgba(e, 0.4, 0.3, 0.6, 1.0);

    let e = world.new_entity();
    cache.menu_data.black_ent = e;
    world.entity_set_billboard(e, String::from("assets/square-b.png"));
    world.entity_set_position_xy(e, 320.0, 192.0);
    world.entity_set_scale_xy(e, 640.0, 384.0);
    world.entity_set_visibility(e, false);
}

pub fn build_menu_upgrade(cache: &mut game::GameData, world: &mut mgfw::ecs::World) {

    cache.menu_data.menu = MENU_UPGRADE;
    cache.menu_data.selection = 0;

    world.clear();
    world.parse_world("assets/world.dat");

    for e in 2..mgfw::ecs::entity::ENTITY_SZ {
        world.entity_set_visibility(e, false);
    }

    world.entity_set_position_xy(0, 320.0, 132.0);
    world.entity_set_position_xy(1, 320.0, 132.0);


    let e = world.new_entity();
    cache.menu_data.upgrade_start_ent = e;

    world.entity_set_text(e, String::from("Select Ship Upgrade"));
    world.entity_set_position_xy(e, 320.0 - world.text_get_width(e) as f32 * 0.5 * 3.0, 100.0);
    world.entity_set_scale_xy(e, 3.0, 2.0);
    world.entity_set_visibility(e, true);

    let e = world.new_entity();
    world.entity_set_text(e, String::from("Thrust"));
    world.entity_set_position_xy(e, 160.0, 170.0);
    world.entity_set_scale_xy(e, 2.0, 1.5);
    world.entity_set_visibility(e, true);

    let e = world.new_entity();
    world.entity_set_text(e, String::from("[]]]]]]]]]]]]"));
    world.entity_set_position_xy(e, 330.0, 170.0);
    world.entity_set_scale_xy(e, 2.0, 1.5);
    world.entity_set_visibility(e, true);
    world.entity_set_color_rgba(e, 0.1, 0.1, 0.1, 1.0);

    let e = world.new_entity();
    world.entity_set_position_xy(e, 330.0, 170.0);
    world.entity_set_scale_xy(e, 2.0, 1.5);
    world.entity_set_visibility(e, true);
    world.entity_set_color_rgba(e, 0.2, 0.8, 0.4, 1.0);

    let e = world.new_entity();
    world.entity_set_text(e, String::from("Steering"));
    world.entity_set_position_xy(e, 160.0, 200.0);
    world.entity_set_scale_xy(e, 2.0, 1.5);
    world.entity_set_visibility(e, true);

    let e = world.new_entity();
    world.entity_set_text(e, String::from("[]]]]]]]]]]]]"));
    world.entity_set_position_xy(e, 330.0, 200.0);
    world.entity_set_scale_xy(e, 2.0, 1.5);
    world.entity_set_visibility(e, true);
    world.entity_set_color_rgba(e, 0.1, 0.1, 0.1, 1.0);

    let e = world.new_entity();
    world.entity_set_position_xy(e, 330.0, 200.0);
    world.entity_set_scale_xy(e, 2.0, 1.5);
    world.entity_set_visibility(e, true);
    world.entity_set_color_rgba(e, 0.2, 0.8, 0.4, 1.0);
    
    let e = world.new_entity();
    world.entity_set_text(e, String::from("Shield"));
    world.entity_set_position_xy(e, 160.0, 230.0);
    world.entity_set_scale_xy(e, 2.0, 1.5);
    world.entity_set_visibility(e, true);
    
    let e = world.new_entity();
    world.entity_set_text(e, String::from("[]]]]]]]]]]]]"));
    world.entity_set_position_xy(e, 330.0, 230.0);
    world.entity_set_scale_xy(e, 2.0, 1.5);
    world.entity_set_visibility(e, true);
    world.entity_set_color_rgba(e, 0.1, 0.1, 0.1, 1.0);

    let e = world.new_entity();
    world.entity_set_position_xy(e, 330.0, 230.0);
    world.entity_set_scale_xy(e, 2.0, 1.5);
    world.entity_set_visibility(e, true);
    world.entity_set_color_rgba(e, 0.2, 0.8, 0.4, 1.0);

    

    let mut p: Vec<mgfw::ecs::Position> = Vec::new();
    let mut c: Vec<mgfw::ecs::Color> = Vec::new();

    let r = 0.5;
    let pr = 0.0;//mgfw::PI as f32 * -0.5;
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
    
    let e = world.new_entity();
    cache.menu_data.cursor_ent = e;
    cache.menu_data.timer = 0.0;

    world.entity_set_position_xy(e, 225.0 + 5.0 * (cache.menu_data.timer * 10.0).sin() as f32, 180.0);
    world.entity_set_scale_xy(e, 32.0, 32.0);
    world.entity_set_line_buffer(e, &p, &c);
    world.entity_set_visibility(e, true);

    let e = world.new_entity();
    cache.menu_data.black_ent = e;
    world.entity_set_billboard(e, String::from("assets/square-b.png"));
    world.entity_set_position_xy(e, 320.0, 192.0);
    world.entity_set_scale_xy(e, 640.0, 384.0);
    world.entity_set_visibility(e, false);

    update_upgrade_ui(cache, world);
}

fn update_upgrade_ui(cache: &mut game::GameData, world: &mut mgfw::ecs::World) {
    
    let mut sthrust = String::from("]]");
    for _i in 0..cache.track_data.player.thrust {
        sthrust = format!("{}]", sthrust);
    }
    world.entity_set_text(cache.menu_data.upgrade_start_ent+3, format!("[{}", sthrust));
    
    let mut ssteering = String::from("]]]");
    for _i in 0..cache.track_data.player.steering {
        ssteering = format!("{}]", ssteering);
    }
    world.entity_set_text(cache.menu_data.upgrade_start_ent+6, format!("[{}", ssteering));
    
    let mut sshield = String::from("]");
    for _i in 0..cache.track_data.player.shield {
        sshield = format!("{}]", sshield);
    }
    world.entity_set_text(cache.menu_data.upgrade_start_ent+9, format!("[{}", sshield));
}

// this gets called by MGFW with input events
#[rustfmt::skip]
pub fn event(
    cache: &mut game::GameData, heap: &mut game::GameDataHeap, world: &mut mgfw::ecs::World, event_id: u8) -> bool {

    if cache.menu_data.transition_lock { return false; }

    match event_id {
        mgfw::EVENT_INPUT_KEYBOARD_RELEASED_ESCAPE => {
            match cache.menu_data.menu {
                MENU_RESULTS => {
                    if cache.menu_data.upgrade_available {
                        transition_to(cache, MENU_UPGRADE);
                    } else {
                        transition_to(cache, MENU_MAIN);
                    }
                    return true;
                },
                MENU_TRACK => {
                    transition_to(cache, MENU_MAIN);
                    return true;
                },
                _ => (),
            }            
        }
        mgfw::EVENT_INPUT_KEYBOARD_RELEASED_SPACE => {
            match cache.menu_data.menu {
                MENU_MAIN => {
                    if 0 == cache.menu_data.selection {
                        track::init_player(cache);
                        transition_to(cache, MENU_TRACK); return true;
                    }
                    else if (2 == cache.menu_data.num_options && 1 == cache.menu_data.selection) || (3 == cache.menu_data.num_options && 2 == cache.menu_data.selection) {
                        exit(0);
                    }
                    else {
                        transition_to(cache, MENU_TRACK); return true;
                    }                    
                },
                MENU_RESULTS => {
                    if cache.menu_data.upgrade_available {
                        transition_to(cache, MENU_UPGRADE);
                    } else {
                        transition_to(cache, MENU_TRACK);
                    }
                    return true;
                },
                MENU_UPGRADE => {
                    let mut upgraded = false;
                    match cache.menu_data.selection {
                        0 => if 10 > cache.track_data.player.thrust { cache.track_data.player.thrust += 1; upgraded = true; },
                        1 => if 9 > cache.track_data.player.steering { cache.track_data.player.steering += 1; upgraded = true; },
                        2 => if 11 > cache.track_data.player.shield { cache.track_data.player.shield += 1; upgraded = true; },
                        _ => (),
                    }
                    if upgraded {
                        update_upgrade_ui(cache, world);
                        transition_to(cache, MENU_TRACK);
                    }
                    return true;
                },
                MENU_TRACK => {
                    if !cache.track_data.track_locked[cache.menu_data.selection] {
                        transition_to(cache, MENU_PLAYING);
                        return true;
                    }
                },
                _ => (),
            }            
//
        },
        mgfw::EVENT_INPUT_KEYBOARD_RELEASED_UP => {
            match cache.menu_data.menu {
                MENU_MAIN => { cache.menu_data.selection = (cache.menu_data.selection + cache.menu_data.num_options - 1) % cache.menu_data.num_options; },
                MENU_UPGRADE => { cache.menu_data.selection = (cache.menu_data.selection + 2) % 3; },
                _ => (),
            }
        }
        mgfw::EVENT_INPUT_KEYBOARD_RELEASED_DOWN => {
            match cache.menu_data.menu {
                MENU_MAIN => { cache.menu_data.selection = (cache.menu_data.selection + 1) % cache.menu_data.num_options; },
                MENU_UPGRADE => { cache.menu_data.selection = (cache.menu_data.selection + 1) % 3; },
                _ => (),
            }
        }
        mgfw::EVENT_INPUT_KEYBOARD_RELEASED_LEFT => {
            if MENU_TRACK == cache.menu_data.menu {
                if 0 < cache.menu_data.selection { cache.menu_data.selection -= 1; }
            }
//            cache.move_left = false;
        }
        mgfw::EVENT_INPUT_KEYBOARD_RELEASED_RIGHT => {
            if MENU_TRACK == cache.menu_data.menu {
                if 9 > cache.menu_data.selection { cache.menu_data.selection += 1; }
            }
//            cache.move_right = false;    
        }
        _ => (),
    }

    true
}

fn transition_to (cache: &mut game::GameData, menu: usize) {
    cache.menu_data.transition_lock = true;
    cache.menu_data.transition_target = menu;
    cache.menu_data.transition_timer = 0.75;
}

// this gets called by MGFW at 1200hz
#[rustfmt::skip]
pub fn update(cache: &mut game::GameData, heap: &mut game::GameDataHeap, world: &mut mgfw::ecs::World) -> bool {
    let mut expect_blown = false;
    cache.frame = (cache.frame + 1) % 128;

    let dt = 1.0 / 1200.0;

    cache.menu_data.timer += dt;

    match cache.menu_data.menu {

        MENU_MAIN => world.entity_set_position_xy(cache.menu_data.cursor_ent, 205.0 + 5.0 * (cache.menu_data.timer * 10.0).sin() as f32, 165.0 + 30.0 * cache.menu_data.selection as f32),
        MENU_UPGRADE => world.entity_set_position_xy(cache.menu_data.cursor_ent, 135.0 + 5.0 * (cache.menu_data.timer * 10.0).sin() as f32, 180.0 + 30.0 * cache.menu_data.selection as f32),
        MENU_TRACK => {
            world.entity_set_text(cache.menu_data.track_name_ent, format!("Track: {}", match cache.menu_data.selection {
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
            world.entity_set_position_xy(cache.menu_data.track_name_ent, 320.0 - world.text_get_width(cache.menu_data.track_name_ent) as f32 * 0.5 * 2.0, 300.0);
            world.entity_set_color_rgba(cache.menu_data.track_name_ent, 0.5, 0.7, 1.0, 1.0);

            for i in 0..10 {
                let mut scale: f32 = 10.0;
                if i == cache.menu_data.selection { scale = 20.0; }
                let locked = cache.track_data.track_locked[i];

                let e = cache.menu_data.track_geometry_start_ent + i;

                world.entity_set_position_xy(e, 320.0 + 240.0 * (i as f32 - cache.menu_data.selection as f32) - cache.menu_data.track_center[i].x * scale, 220.0 - cache.menu_data.track_center[i].y * scale);
                world.entity_set_scale_xy(e, scale, scale);
                world.entity_set_color_rgba(e, 1.0, 1.0, 1.0, 1.0);
                if locked {
                    world.entity_set_color_rgba(e, 0.5, 0.2, 0.2, 1.0);
                    if i == cache.menu_data.selection {
                        world.entity_set_text(cache.menu_data.track_name_ent, String::from("LOCKED"));
                        world.entity_set_position_xy(cache.menu_data.track_name_ent, 320.0 - world.text_get_width(cache.menu_data.track_name_ent) as f32 * 0.5 * 2.0, 300.0);
                        world.entity_set_color_rgba(cache.menu_data.track_name_ent, 0.8, 0.2, 0.2, 1.0);
                    }
                }
            }
        }
        _ => (),
    }

    if cache.menu_data.transition_lock {
        world.entity_set_visibility(cache.menu_data.black_ent, true);
        world.entity_set_alpha(cache.menu_data.black_ent, 1.0 - cache.menu_data.transition_timer as f32 / 0.75);

        cache.menu_data.transition_timer -= dt;
        if 0.0 > cache.menu_data.transition_timer {
            cache.menu_data.transition_lock = false;
            world.entity_set_alpha(cache.menu_data.black_ent, 1.0);
            cache.menu_data.menu = cache.menu_data.transition_target;
            match cache.menu_data.menu {
                MENU_MAIN => build_menu_main(cache, world),
                MENU_UPGRADE => build_menu_upgrade(cache, world),
                MENU_PLAYING => track::init_race(cache, heap, world, cache.menu_data.selection),
                MENU_TRACK => build_menu_track(cache, heap, world),
                _ => (),
            }
            expect_blown = true;
        }
    } else {
        world.entity_set_visibility(cache.menu_data.black_ent, false);
    }

    expect_blown
}

