use crate::events::{Event, EventQueue};
use crate::platform::{PlatformWindow, Window};
use std::sync::Arc;

#[derive(Debug)]
pub struct App {
    event_queue: Arc<EventQueue>,
    window: Window,
    running: bool,
}

impl App {
    pub fn new() -> Self {
        let event_queue = Arc::new(EventQueue::new());
        let window = Window::new("Hello", event_queue.clone());

        App {
            event_queue,
            window,
            running: true,
        }
    }

    pub fn run(&mut self) {
        while self.running {
            for event in self.event_queue.drain() {
                self.handle_event(event);
            }

            self.window.update();
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::WindowClose => {
                self.running = false;
            }
            _ => {}
        }
    }
}
