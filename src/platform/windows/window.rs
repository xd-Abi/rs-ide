use crate::events::{Event, EventQueue, Key};
use crate::platform::PlatformWindow;
use std::sync::Arc;

#[derive(Debug)]
pub struct WindowsWindow {
    event_queue: Arc<EventQueue>,
}

impl PlatformWindow for WindowsWindow {
    fn new(title: &str, event_queue: Arc<EventQueue>) -> Self {
        WindowsWindow { event_queue }
    }

    fn update(&self) {
        self.event_queue.push(Event::KeyRelease(Key::W));
    }
}
