extern crate glutin;
extern crate cgmath;
#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;

#[cfg(target_os = "android")]
extern crate android_glue;

pub mod logger;
pub mod time;
pub mod renderer;
//pub mod physics;

use std::rc::Rc;
use std::cell::RefCell;

use logger::Logger;
use time::Time;
use renderer::Renderer;

struct Engine {
    logger: Rc<Logger>,
    time: Rc<RefCell<Time>>,
    glutin_event_loop: Rc<glutin::EventsLoop>,
    window: Rc<glutin::Window>,
    renderer: Renderer,
    //physics: Physics::Physics,
}

impl Engine {
    pub fn new() -> Self {
        let logger = Rc::new(Logger::new());

        info!(logger, "Engine init started");
        info!(logger, "Logger init");

        let time = Rc::new(RefCell::new(Time::new(logger.clone())));

        info!(logger, "Time init");

        let glutin_event_loop = Rc::new(glutin::EventsLoop::new());

        info!(logger, "Event loop init");

        let window = {
            let monitor = glutin::get_primary_monitor();

            #[cfg(not(target_os = "android"))]
            let window = glutin::WindowBuilder::new()
                .with_title("Engine test shit gg")
                .with_fullscreen(monitor)
                .with_vsync()
                .with_multisampling(4u16)
                .with_depth_buffer(24u8)
                .with_gl(glutin::GlRequest::Latest)
                .with_gl_profile(glutin::GlProfile::Core)
                .with_gl_robustness(glutin::Robustness::RobustNoResetNotification)
                .build(&glutin_event_loop)
                .unwrap();

            #[cfg(target_os = "android")]
            let window = glutin::WindowBuilder::new()
                .with_vsync()
                .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGlEs, (3, 2)))
                .build(&glutin_event_loop)
                .unwrap();


            Rc::new(window)
        };

        info!(logger, "Window init");

        let renderer = Renderer::new(window.clone(), logger.clone()).unwrap();

        info!(logger, "Renderer init");

        info!(logger, "Engine init complete");

        Self {
            logger,
            time,
            glutin_event_loop,
            window,
            renderer,
        }
    }

    pub fn run_main_loop(&mut self) {
        info!(self.logger, "Running main loop");

        'main: while self.handle_events() {
            self.time.borrow_mut().update();
            self.renderer.render(self.time.borrow().delta_time());
        }

        info!(self.logger, "Main loop ended");
    }

    fn handle_events(&self) -> bool {
        use glutin::{Event, WindowEvent, ElementState, VirtualKeyCode};

        let mut running = true;

        self.glutin_event_loop.poll_events(|event| {
            match event {
                Event::WindowEvent { event: WindowEvent::Closed, .. } => {
                    running = false;
                },
                Event::WindowEvent { event: WindowEvent::KeyboardInput(ElementState::Pressed, _, Some(VirtualKeyCode::Escape), _), .. } => {
                    running = false;
                }
                _ => ()
            }
        });

        running
    }
}

#[cfg(not(target_os = "android"))]
pub fn load_file(src: &'static str) -> std::io::Result<String> {
    use std::io::{Read, BufReader};
    use std::fs::File;
    use std::path::Path;

    let mut ret = String::new();
    let filename = &Path::new("shaders/gl").join(&src);
    let file = File::open(filename)?;
    let mut buf = BufReader::new(file);
    buf.read_to_string(&mut ret)?;

    Ok(ret)
}

#[cfg(target_os = "android")]
pub fn load_file(src: &'static str) -> std::io::Result<String> {
    use std::path::Path;
    use android_glue;

    let filename = Path::new("es").join(&src);
    let filename = filename.to_str().unwrap();

    let ret = match android_glue::load_asset(filename) {
        Ok(buf) => String::from_utf8(buf).unwrap(),
        _ => panic!("Failed to load asset"),
    };

    Ok(ret)
}

fn main() {
    let mut engine = Engine::new();
    engine.run_main_loop();
}
