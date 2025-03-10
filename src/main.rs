use platform::{Window, WindowTrait};

mod platform;

fn main() {
    let window = Window::new("Rust IDE", 800, 600);

    loop {
        if window.is_close_requested() {
            break;
        }

        window.update();
    }
}
