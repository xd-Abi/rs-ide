use tao::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Rust IDE")
        .with_decorations(false)
        .build(&event_loop).expect("Failed to create window");

    let mut mouse_y_pos = 0.0;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event, ..
        } => match event {
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => {
                // Title bar height
                if mouse_y_pos < 55.0 {
                    window.drag_window().expect("Failed to drag window")
                }
            },
            WindowEvent::CursorMoved { position, .. } => {
                mouse_y_pos = position.y;
            }
            _ => (),
        },
        _ => (),
    });
}
