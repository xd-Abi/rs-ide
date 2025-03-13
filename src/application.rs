use crate::platform::{Event, Window};
use std::cell::RefCell;
use std::rc::Rc;
use std::thread::sleep;
use std::time::{Duration, Instant};
use tracing::info;

#[derive(Debug)]
pub struct Application {
    window: Rc<RefCell<Window>>,
    running: bool,
}

impl Application {
    pub fn new() -> Self {
        Application {
            window: Window::new("Rust IDE", 800, 600),
            running: true,
        }
    }

    pub fn init(&mut self) {
        let window = Rc::clone(&self.window);
        let mut app_ref = self as *mut Self;

        window.borrow_mut().set_event_callback(move |event| {
            unsafe { (*app_ref).handle_event(event); }
        });
    }

    pub fn run(&self) {
        let target_fps = 1.0 / 30.0;
        let frame_time = Duration::from_secs_f64(target_fps);

        while self.running {
            let start = Instant::now();

            self.window.borrow().update();

            let elapsed = start.elapsed();
            if elapsed < frame_time {
                sleep(frame_time - elapsed);
            }
        }
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::WindowClose => {
                self.running = false;
            }
            Event::WindowResized(w, h) => {
                info!("Window resized to {}x{}", w, h);
            }
            _ => {}
        }
    }
}
