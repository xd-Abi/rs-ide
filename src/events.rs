use std::sync::Mutex;
use tracing::error;

/// Represents an event that can be handled by the application.
///
/// Events include window events, mouse input, and keyboard input.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Event {
    WindowClose,
    WindowResized(u32, u32),
    MouseMove(i32, i32),
    MouseDown(MouseButton),
    MouseRelease(MouseButton),
    KeyDown(Key),
    KeyRelease(Key),
}

/// Represents different mouse buttons that can be pressed or released.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    XButton1,
    XButton2,
}

/// Represents different keyboard keys that can be pressed or released.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    // Number Keys 0-9
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,

    // Special Keys
    Escape,
    Tab,
    CapsLock,
    Shift,
    Ctrl,
    LeftSuper,
    RightSuper,
    Alt,
    Space,
    Enter,
    Backspace,

    // Function Keys (F1 - F12)
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    // Arrow Keys
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    ArrowDown,
}

/// A thread-safe event queue for handling input events.
///
/// The event queue stores events (such as window resizes, key presses, and mouse movements)
/// and allows them to be processed by the application loop.
#[derive(Debug)]
pub struct EventQueue {
    queue: Mutex<Vec<Event>>,
}

impl EventQueue {
    /// Creates a new, empty event queue.
    pub fn new() -> Self {
        EventQueue {
            queue: Mutex::new(Vec::new()),
        }
    }

    /// Pushes a new event onto the queue.
    ///
    /// If the queue is locked by another thread, an error is logged, but execution continues.
    ///
    /// # Arguments
    ///
    /// * `event` - The event to be added to the queue.
    pub fn push(&self, event: Event) {
        if let Ok(mut queue) = self.queue.lock() {
            queue.push(event);
        } else {
            error!("Failed to lock EventQueue for push");
        }
    }

    /// Drains all events from the queue and returns them as a `Vec<Event>`.
    ///
    /// This function clears the queue after retrieving all events.
    /// If the queue is locked by another thread, an error is logged, and an empty vector is returned.
    ///
    /// # Returns
    ///
    /// A vector containing all the events that were in the queue.
    pub fn drain(&self) -> Vec<Event> {
        if let Ok(mut queue) = self.queue.lock() {
            queue.drain(..).collect()
        } else {
            error!("Failed to lock EventQueue for drain");
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    /// Tests if the queue correctly adds and retrieves a single event.
    #[test]
    fn test_push_and_drain() {
        let queue = EventQueue::new();

        queue.push(Event::WindowClose);
        let events = queue.drain();

        assert_eq!(events.len(), 1);
        assert_eq!(events[0], Event::WindowClose);

        // The queue should be empty after draining
        assert_eq!(queue.drain().len(), 0);
    }

    /// Tests if multiple events are handled correctly.
    #[test]
    fn test_multiple_events() {
        let queue = EventQueue::new();

        queue.push(Event::MouseMove(10, 20));
        queue.push(Event::KeyDown(Key::A));
        queue.push(Event::WindowResized(800, 600));

        let events = queue.drain();
        assert_eq!(events.len(), 3);
        assert_eq!(events[0], Event::MouseMove(10, 20));
        assert_eq!(events[1], Event::KeyDown(Key::A));
        assert_eq!(events[2], Event::WindowResized(800, 600));

        // Queue should be empty after draining
        assert_eq!(queue.drain().len(), 0);
    }

    /// Tests that draining the queue multiple times works correctly.
    #[test]
    fn test_drain_empty_queue() {
        let queue = EventQueue::new();

        // First drain: Should return an empty Vec.
        assert_eq!(queue.drain().len(), 0);

        queue.push(Event::KeyDown(Key::F1));

        // Second drain: Should return 1 event.
        assert_eq!(queue.drain().len(), 1);

        // Third drain: Should return an empty Vec again.
        assert_eq!(queue.drain().len(), 0);
    }

    /// Tests thread safety by running multiple threads that push events at the same time.
    #[test]
    fn test_concurrent_access() {
        let queue = Arc::new(EventQueue::new());

        let queue1 = Arc::clone(&queue);
        let queue2 = Arc::clone(&queue);

        let thread1 = std::thread::spawn(move || {
            for _ in 0..100 {
                queue1.push(Event::MouseDown(MouseButton::Left));
            }
        });

        let thread2 = std::thread::spawn(move || {
            for _ in 0..100 {
                queue2.push(Event::KeyDown(Key::A));
            }
        });

        thread1.join().unwrap();
        thread2.join().unwrap();

        // Make sure all 200 events were added.
        let events = queue.drain();
        assert_eq!(events.len(), 200);
    }
}
