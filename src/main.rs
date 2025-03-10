use log::{debug, info};
use std::collections::HashMap;
use std::ptr::addr_eq;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalPosition;
use winit::error::ExternalError;
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{CursorIcon, ResizeDirection, Window, WindowId};

#[derive(Default)]
struct App {
    windows: HashMap<WindowId, Window>,
    mouse: Option<PhysicalPosition<f64>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        info!("App Resumed!");
        let window_attributes = winit::window::WindowAttributes::default()
            .with_title("My Window")
            .with_decorations(false)
            .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));

        let window = event_loop
            .create_window(window_attributes)
            .expect("Failed to create window");
        self.windows.insert(window.id(), window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let border_threshold = 10.0;
        let title_bar_thickness = 55.0;

        match event {
            WindowEvent::CloseRequested => {
                info!("Window {:?} closed", window_id);
                event_loop.exit();
            }
            WindowEvent::Resized(physical_size) => {
                info!("Window {:?} resized to: {:?}", window_id, physical_size);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if state == ElementState::Pressed {
                    if let Some(position) = self.mouse {
                        if let Some(window) = self.windows.get(&window_id) {

                            let size = window.inner_size();

                            if position.x < border_threshold && position.y < border_threshold {
                                window
                                    .drag_resize_window(ResizeDirection::NorthWest)
                                    .expect("Failed to resize window");
                            } else if position.x > size.width as f64 - border_threshold && position.y < border_threshold {
                                window
                                    .drag_resize_window(ResizeDirection::NorthEast)
                                    .expect("Failed to resize window");
                            } else if position.x < border_threshold && position.y > size.height as f64 - border_threshold {
                                window
                                    .drag_resize_window(ResizeDirection::SouthWest)
                                    .expect("Failed to resize window");
                            } else if position.x > size.width as f64 - border_threshold && position.y > size.height as f64 - border_threshold {
                                window
                                    .drag_resize_window(ResizeDirection::SouthEast)
                                    .expect("Failed to resize window");
                            } else if position.x < border_threshold {
                                window
                                    .drag_resize_window(ResizeDirection::West)
                                    .expect("Failed to resize window");
                            } else if position.x > size.width as f64 - border_threshold {
                                window
                                    .drag_resize_window(ResizeDirection::East)
                                    .expect("Failed to resize window");
                            } else if position.y < border_threshold {
                                window
                                    .drag_resize_window(ResizeDirection::North)
                                    .expect("Failed to resize window");
                            } else if position.y > size.height as f64 - border_threshold {
                                window
                                    .drag_resize_window(ResizeDirection::South)
                                    .expect("Failed to resize window");
                            } else if position.y < title_bar_thickness {
                                window.drag_window().expect("Failed to grab drag window");
                            }
                        }
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse = Some(position);

                if let Some(window) = self.windows.get(&window_id) {
                    let size = window.inner_size();

                    if position.x < border_threshold && position.y < border_threshold {
                        window.set_cursor(CursorIcon::NwResize);
                    } else if position.x > size.width as f64 - border_threshold && position.y < border_threshold {
                        window.set_cursor(CursorIcon::NeResize);
                    } else if position.x < border_threshold && position.y > size.height as f64 - border_threshold {
                        window.set_cursor(CursorIcon::SwResize);
                    } else if position.x > size.width as f64 - border_threshold && position.y > size.height as f64 - border_threshold {
                        window.set_cursor(CursorIcon::SeResize);
                    } else if position.x < border_threshold {
                        window.set_cursor(CursorIcon::WResize);
                    } else if position.x > size.width as f64 - border_threshold {
                        window.set_cursor(CursorIcon::EResize);
                    } else if position.y < border_threshold {
                        window.set_cursor(CursorIcon::NResize);
                    } else if position.y > size.height as f64 - border_threshold {
                        window.set_cursor(CursorIcon::SResize);
                    } else {
                        window.set_cursor(CursorIcon::Default);
                    }
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        //info!("Event loop about to wait!");
    }

    fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        info!("App suspended!");
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        info!("App is exiting!");
    }
}

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let mut app = App::default();

    event_loop
        .run_app(&mut app)
        .expect("Failed to run event loop");
}
