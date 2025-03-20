use crate::events::{Event, EventQueue, Key};
use crate::platform;
use crate::platform::{PlatformWindow, Window};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tracing::info;

pub struct App {
    event_queue: Arc<EventQueue>,
    window: Window,
}

impl App {
    pub fn new() -> Self {
        platform::bootstrap();
        let event_queue = Arc::new(EventQueue::new());
        let window = Window::new("Hello", event_queue.clone());

        App {
            event_queue,
            window,
        }
    }

    pub fn run(&mut self) {
        for event in self.event_queue.drain() {
            self.handle_event(event);
        }

        self.window.update();
        thread::sleep(Duration::from_secs(1))
    }

    pub fn handle_event(&mut self, event: Event) {
        info!("{:?}", event);
    }
}

impl Drop for App {
    fn drop(&mut self) {
        platform::shutdown();
    }
}
