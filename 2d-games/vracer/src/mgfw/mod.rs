pub mod cache;
pub mod ecs;
mod fonts;
mod support;

use crate::game::GameWrapper;
use cache::CacheManager;
use std::collections::VecDeque;
use support::Gl;

#[allow(unused_imports)]
use glutin::event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent};
use glutin::event_loop::EventLoop;
use glutin::window::Icon;
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

#[allow(dead_code)]
pub const PI: f64 = 3.1415926535897932384626433;
const WINDOW_SCALE: f64 = 2.0;

#[allow(dead_code)]
pub fn deg2rad(val: f32) -> f32 {
    val * PI as f32 / 180.0
}

struct CoreData {
    initialized: bool,
    running: bool,
    shutdown: bool,
    start_time: std::time::Instant,
    last_update: std::time::Instant,
    last_render: std::time::Instant,
    last_physics: std::time::Instant,
    blown_update_frames: usize,
    blown_update_frames_expected: usize,
    blown_update_frames_significant: usize,
    count_update_frames: usize,
    blown_render_frames: usize,
    count_render_frames: usize,
    completed_first_frame: bool,
    update_frame_load: f64,
    render_frame_load: f64,
    scale_factor: f64,
}

#[allow(dead_code)]
pub const EVENT_INVALID: u8 = 0;
pub const EVENT_INPUT_MOUSE_BUTTON_UP: u8 = 1;

pub const EVENT_INPUT_KEYBOARD_PRESSED_ESCAPE: u8 = 20;
pub const EVENT_INPUT_KEYBOARD_PRESSED_UP: u8 = 21;
pub const EVENT_INPUT_KEYBOARD_PRESSED_DOWN: u8 = 22;
pub const EVENT_INPUT_KEYBOARD_PRESSED_LEFT: u8 = 23;
pub const EVENT_INPUT_KEYBOARD_PRESSED_RIGHT: u8 = 24;
pub const EVENT_INPUT_KEYBOARD_PRESSED_SPACE: u8 = 25;

pub const EVENT_INPUT_KEYBOARD_RELEASED_ESCAPE: u8 = 120;
pub const EVENT_INPUT_KEYBOARD_RELEASED_UP: u8 = 121;
pub const EVENT_INPUT_KEYBOARD_RELEASED_DOWN: u8 = 122;
pub const EVENT_INPUT_KEYBOARD_RELEASED_LEFT: u8 = 123;
pub const EVENT_INPUT_KEYBOARD_RELEASED_RIGHT: u8 = 124;
pub const EVENT_INPUT_KEYBOARD_RELEASED_SPACE: u8 = 125;

pub const EVENT_INPUT_KEYBOARD_RELEASED_PGUP: u8 = 126;
pub const EVENT_INPUT_KEYBOARD_RELEASED_PGDN: u8 = 127;
pub const EVENT_INPUT_KEYBOARD_RELEASED_BKSPC: u8 = 128;


#[allow(dead_code)]
pub struct Core {
    data: *mut CoreData,
    // WARNING: Anything below this line is not in cache!
    pub windowed_context: glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>,
    gl: std::boxed::Box<Gl>,
    game: std::boxed::Box<GameWrapper>,
    cache: std::boxed::Box<CacheManager>,
    world: std::boxed::Box<ecs::World>,
    render_system: std::boxed::Box<ecs::RenderSystem>,
    physics_system: std::boxed::Box<ecs::PhysicsSystem>,
    easing_system: std::boxed::Box<ecs::EasingSystem>,
    events: std::boxed::Box<VecDeque<u8>>,
}

impl Core {
    pub fn new(title: &str, xres: i32, yres: i32, el: &EventLoop<()>) -> Core {
        log(format!("Constructing MGFW Core"));

        // Construct a new RGB ImageBuffer with the specified width and height.
        let icon: image::RgbaImage = image::open("assets/mgfw/mgfw_64_trim.ico")
            .unwrap()
            .to_rgba8();
        let w = icon.dimensions().0 as u32;
        let h = icon.dimensions().1 as u32;
        let b = Some(Icon::from_rgba(icon.into_vec(), w, h).unwrap());

        // img.into_raw().as_ptr() as *const _,

        let window = WindowBuilder::new()
            .with_title(title)
            .with_resizable(false)
            .with_window_icon(b)
            .with_inner_size(glutin::dpi::LogicalSize::new(
                xres as f64 * WINDOW_SCALE,
                yres as f64 * WINDOW_SCALE,
            ));

        let windowed_context = ContextBuilder::new()
            .with_vsync(true)
            .with_multisampling(2)
            .build_windowed(window, &el)
            .unwrap();

        let windowed_context = unsafe { windowed_context.make_current().unwrap() };

        let scale_factor = windowed_context.window().scale_factor();
        windowed_context.window().set_cursor_visible(false);

        let start_time = std::time::Instant::now();

        let gl = Box::new(support::load(
            &windowed_context.context(),
            xres,
            yres,
            (scale_factor * WINDOW_SCALE) as f32,
        ));
        let mut cache = Box::new(CacheManager::new());

        // force clear the display buffers
        gl.clear_frame();
        windowed_context.swap_buffers().unwrap();
        gl.clear_frame();
        windowed_context.swap_buffers().unwrap();

        let sz_bytes = std::mem::size_of::<CoreData>();
        let data = cache.allocate(sz_bytes) as *mut CoreData;
        unsafe {
            *data = CoreData {
                running: false,
                last_update: start_time,
                last_render: start_time,
                last_physics: start_time,
                initialized: false,
                shutdown: false,
                blown_update_frames: 0,
                blown_update_frames_expected: 0,
                blown_update_frames_significant: 0,
                count_update_frames: 0,
                blown_render_frames: 0,
                count_render_frames: 0,
                completed_first_frame: false,
                start_time,
                update_frame_load: 0.0,
                render_frame_load: 0.0,
                scale_factor,
            };
        }

        let world = Box::new(ecs::World::new(&mut cache));
        let render_system = Box::new(ecs::RenderSystem::new(&mut cache, &gl));
        let physics_system = Box::new(ecs::PhysicsSystem::new(&mut cache));
        let easing_system = Box::new(ecs::EasingSystem::new(&mut cache));
        let game = Box::new(GameWrapper::new(&mut cache));
        let events = Box::new(VecDeque::new());

        cache.print_loading();

        Core {
            windowed_context,
            gl,
            data,
            game,
            cache,
            world,
            render_system,
            physics_system,
            easing_system,
            events,
        }
    }

    pub fn check_events(&mut self, event: &glutin::event::Event<()>) -> bool {
        let cache = unsafe { &mut *(self.data.offset(0)) };

        if !cache.initialized {
            self.initialize();
        }

        let mut ret = true;
        //log(format!("{:?}", event));
        match event {
            Event::LoopDestroyed => ret = false,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                    cache.scale_factor = *scale_factor;
                }
                WindowEvent::Resized(physical_size) => self.windowed_context.resize(*physical_size),
                WindowEvent::CloseRequested => ret = false,
                WindowEvent::CursorMoved { position, .. } => {
                    self.update_mouse_xy(
                        (position.x / (cache.scale_factor * WINDOW_SCALE)) as i32,
                        (position.y / (cache.scale_factor * WINDOW_SCALE)) as i32,
                    );
                }
                WindowEvent::MouseInput { state, button, .. } => {
                    self.update_mouse_button(button, state);
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    self.update_keyboard_input(&input);
                }
                _ => (),
            },
            Event::RedrawRequested(_) => self.render(std::time::Instant::now()),
            _ => (),
        }

        if ret {
            self.update();
        } else {
            if !cache.shutdown {
                self.shutdown();
            }
        }
        ret
    }

    fn update_keyboard_input(&mut self, input: &KeyboardInput) {
        if ElementState::Pressed == input.state {
            match input.virtual_keycode {
                Some(VirtualKeyCode::Escape) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_PRESSED_ESCAPE)
                }
                Some(VirtualKeyCode::Up) => self.events.push_back(EVENT_INPUT_KEYBOARD_PRESSED_UP),
                Some(VirtualKeyCode::Down) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_PRESSED_DOWN)
                }
                Some(VirtualKeyCode::Left) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_PRESSED_LEFT)
                }
                Some(VirtualKeyCode::Right) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_PRESSED_RIGHT)
                }
                Some(VirtualKeyCode::Space) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_PRESSED_SPACE)
                }
                _ => (),
            }
        } else if ElementState::Released == input.state {
            match input.virtual_keycode {
                Some(VirtualKeyCode::Escape) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_ESCAPE)
                }
                Some(VirtualKeyCode::Up) => self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_UP),
                Some(VirtualKeyCode::Down) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_DOWN)
                }
                Some(VirtualKeyCode::Left) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_LEFT)
                }
                Some(VirtualKeyCode::Right) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_RIGHT)
                }
                Some(VirtualKeyCode::Space) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_SPACE)
                }
                Some(VirtualKeyCode::PageUp) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_PGUP)
                }
                Some(VirtualKeyCode::PageDown) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_PGDN)
                }
                Some(VirtualKeyCode::Back) => {
                    self.events.push_back(EVENT_INPUT_KEYBOARD_RELEASED_BKSPC)
                }
                _ => (),
            }
        }
    }

    fn update_mouse_xy(&mut self, x: i32, y: i32) {
        self.world.mouse_x = x;
        self.world.mouse_y = y;
    }

    fn update_mouse_button(
        &mut self,
        button: &glutin::event::MouseButton,
        state: &glutin::event::ElementState,
    ) {
        //let cache = unsafe { &mut *(self.data.offset(0)) };
        if MouseButton::Left == *button && ElementState::Released == *state {
            //log(format!("mouse clicked at {}, {}", cache.mouse_x, cache.mouse_y);

            // insert message into the input FIFO
            self.events.push_back(EVENT_INPUT_MOUSE_BUTTON_UP);
        }
    }

    fn initialize(&mut self) {
        let cache = unsafe { &mut *(self.data.offset(0)) };

        self.game.initialize(&mut self.world);
        cache.initialized = true;
        let ms = std::time::Instant::now()
            .duration_since(cache.start_time)
            .as_micros() as f32
            / 1000.0;

        log(format!("Initialization Complete {:.2} ms", ms));

        const INIT_LIMIT: f32 = 1000.0;
        if ms > INIT_LIMIT {
            log(format!(
                "WARNING: blown Initilization time limit ({:} ms)",
                INIT_LIMIT
            ));
        }
    }

    fn render(&mut self, start_time: std::time::Instant) {
        self.gl.clear_frame();
        self.render_system.render(&self.gl, &self.world, start_time);
    }

    fn shutdown(&mut self) {
        let cache = unsafe { &mut *(self.data.offset(0)) };

        self.game.shutdown();

        if 0 < cache.blown_update_frames_significant {
            log(format!(
                "Blown Update frames: Total: {}, Sig: {} ({}%), Expected: ({}%)",
                cache.blown_update_frames,
                cache.blown_update_frames_significant,
                (cache.blown_update_frames_significant as f32 * 100.0
                    / cache.blown_update_frames as f32) as i32,
                (cache.blown_update_frames_expected as f32 * 100.0
                    / cache.blown_update_frames as f32) as i32
            ));
        }

        if 10 < cache.blown_render_frames {
            log(format!(
                "WARNING: {} blown Render frames ({}%)",
                cache.blown_render_frames,
                (cache.blown_render_frames as f32 * 100.0 / cache.count_render_frames as f32)
                    as i32
            ));
        }

        log(format!(
            "Avg Update frame loading: {}%",
            (cache.update_frame_load * 100.0 / cache.count_render_frames as f64) as i32
        ));
        log(format!(
            "Avg Render frame loading: {}%",
            (cache.render_frame_load * 100.0 / cache.count_render_frames as f64) as i32
        ));
        cache.shutdown = true;
    }

    fn update(&mut self) {
        let cache = unsafe { &mut *(self.data.offset(0)) };

        if !cache.running {
            cache.last_update = std::time::Instant::now();
            cache.last_render = std::time::Instant::now();
            cache.running = true;

            // pre-update for lazy loading
            self.game.update(&mut self.world, 0);
            self.physics_system.update(&mut self.world, 0);
            self.render_system.update(&self.gl, &mut self.world);
            self.easing_system.update(&mut self.world, 0);
        }

        // inner update loop
        let mut loop_counter = 0;
        const UPDATE_DT: u128 = 833; // microseconds
        loop {
            let delta = std::time::Instant::now().duration_since(cache.last_update);

            // break out of loop if stuck or finished
            loop_counter += 1;
            if UPDATE_DT > delta.as_micros() || loop_counter > 100 {
                break;
            }

            cache.last_update += std::time::Duration::from_micros(UPDATE_DT as u64);
            let timer_start = std::time::Instant::now();

            let mut expect_blown = false;

            // update game
            expect_blown |= self.game.update(&mut self.world, UPDATE_DT);

            // update systems
            if 0 == cache.count_update_frames % 1 {
                // priority 1 systems
            }

            if 0 == cache.count_update_frames % 2 {
                // priority 2 systems
                expect_blown |= self.render_system.update(&self.gl, &mut self.world);
            }

            if 1 == cache.count_update_frames % 4 {
                // priority 3 systems
                expect_blown |= self.physics_system.update(&mut self.world, UPDATE_DT * 4);
                cache.last_physics = std::time::Instant::now();

                if let Some(val) = self.events.pop_front() {
                    expect_blown |= self.game.event(&mut self.world, val);
                }

                expect_blown |= self.easing_system.update(&mut self.world, UPDATE_DT * 4);

                /*if cfg!(debug_assertions) {
                    // artificial jitter
                    if rand::random::<f32>() < 0.01 {
                        let now = std::time::Instant::now();
                        let delta = (rand::random::<f32>() * 30.0) as u128;
                        loop {
                            if std::time::Instant::now().duration_since(now).as_millis() > delta {
                                break;
                            }
                        }
                        expect_blown = true;
                    }
                }*/
            }

            let delta = std::time::Instant::now()
                .duration_since(timer_start)
                .as_micros();
            if UPDATE_DT < delta {
                if expect_blown {
                    cache.blown_update_frames_expected += 1;
                }
                if UPDATE_DT * 3 < delta {
                    cache.blown_update_frames_significant += 1;
                }
                cache.blown_update_frames += 1;
            }
            cache.count_update_frames += 1;
            cache.update_frame_load += delta as f64 / UPDATE_DT as f64;
        }

        // outter render loop
        let delta = std::time::Instant::now().duration_since(cache.last_render);

        const RENDER_DT: u128 = 16666; // microseconds

        if RENDER_DT < delta.as_micros() {
            cache.last_render = std::time::Instant::now();

            // render frame
            self.render(cache.last_physics);

            let delta = std::time::Instant::now()
                .duration_since(cache.last_render)
                .as_micros();
            if RENDER_DT < delta {
                cache.blown_render_frames += 1;
            }
            cache.count_render_frames += 1;
            cache.render_frame_load += delta as f64 / RENDER_DT as f64;

            self.windowed_context.swap_buffers().unwrap();

            if !cache.completed_first_frame {
                cache.completed_first_frame = true;
                let ms = std::time::Instant::now()
                    .duration_since(cache.start_time)
                    .as_micros() as f32
                    / 1000.0;

                log(format!("Time to first Render frame: {:.2} ms", ms));

                const FIRST_FRAME_LIMIT: f32 = 4000.0;
                if ms > FIRST_FRAME_LIMIT {
                    log(format!(
                        "WARNING: blown time to first Render frame limit ({} ms)",
                        FIRST_FRAME_LIMIT
                    ));
                }
            }
        }
    }
}

pub fn log(output: String) {
    if cfg!(debug_assertions) {
        println!("{}", output);
    }
}
