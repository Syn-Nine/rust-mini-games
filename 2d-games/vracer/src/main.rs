mod game;
mod mgfw;

const TITLE: &str = "V-Racer";
const XRES: i32 = 640;
const YRES: i32 = 384;

fn main() {
    let el = glutin::event_loop::EventLoop::new();
    let mut core = mgfw::Core::new(TITLE, XRES, YRES, &el);

    el.run(move |event, _, control_flow| {
        if !core.check_events(&event) {
            *control_flow = glutin::event_loop::ControlFlow::Exit;
        }
    });
}
