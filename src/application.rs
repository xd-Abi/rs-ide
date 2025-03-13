use crate::platform::{Event, Window};
use std::thread::sleep;
use std::time::{Duration, Instant};
use tracing::info;

#[derive(Debug)]
pub struct Application {
    window: Box<Window>,
    running: bool,
}

impl Application {
    pub fn new() -> Self {
        info!("Initializing application...");
        Application {
            window: Window::new("Rust IDE", 800, 600),
            running: true,
        }
    }

    pub fn init(&mut self) {
        let app_ref = self as *mut Self;

        // @TODO: Fix event callback to avoid unsafe code
        self.window.set_event_callback(move |event| {
            unsafe { (*app_ref).handle_event(event); }
        });
    }

    pub fn run(&self) {
        let target_fps = 1.0 / 30.0;
        let frame_time = Duration::from_secs_f64(target_fps);

        while self.running {
            let start = Instant::now();

            self.window.update();

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
            _ => {}
        }
    }
}
