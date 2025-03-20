use crate::events::{Event, EventQueue, Key};
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
        let event_queue = Arc::new(EventQueue::new());
        let window = Window::new(event_queue.clone());

        App {
            event_queue,
            window,
        }
    }

    pub fn run(&mut self) {
        loop {
            for event in self.event_queue.drain() {
                self.handle_event(event);
            }

            self.window.update();
            thread::sleep(Duration::from_secs(1))
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        info!("{:?}", event);
    }
}

pub struct Window {
    event_queue: Arc<EventQueue>,
}

impl Window {
    pub fn new(event_queue: Arc<EventQueue>) -> Self {
        Window { event_queue }
    }

    pub fn update(&self) {
        self.event_queue.push(Event::KeyRelease(Key::W));
    }
}
