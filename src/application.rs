use crate::platform::Window;

#[derive(Debug)]
pub struct Application {
    window: Window,
}

impl Application {
    pub fn new() -> Application {
        let window = Window::new("Rust IDE", 800, 600);
        Application { window }
    }

    pub fn run(&self) {
        loop {
            self.window.update();
        }
    }
}
