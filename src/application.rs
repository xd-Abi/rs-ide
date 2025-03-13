use std::thread::sleep;
use std::time::{Duration, Instant};
use tracing::info;
use crate::platform::{Event, Window};

#[derive(Debug)]
pub struct Application {
    window: Box<Window>,
}

impl Application {
    pub fn new() -> Application {
        let mut app = Application {
            window: Window::new("Rust IDE", 800, 600),
        };

        app.window.set_event_callback(move |event| {
            match event {
                Event::WindowResized(width, height) => {
                    println!("Application received WindowResized event: {}x{}", width, height);
                }
            }
        });
        app
    }

    pub fn run(&self) {
        let target_fps = 1.0 / 30.0;
        let frame_time = Duration::from_secs_f64(target_fps);

        loop {
            let start = Instant::now();

            self.window.update();

            let elapsed = start.elapsed();
            if elapsed < frame_time {
                sleep(frame_time - elapsed);
            }
        }
    }

    fn on_event(&mut self, event: Event) {
        match event {
            Event::WindowResized(width, height) => {
                println!("Application received WindowResized event: {}x{}", width, height);
            }
        }
    }
}
