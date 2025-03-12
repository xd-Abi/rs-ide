use crate::config;
use crate::config::{Config, WindowConfig};
use crate::logging::info;
use crate::renderer::Renderer;
use crate::window::Window;
use std::time::Instant;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

pub struct App {
    config: Config,
    window: Option<Window>,
    renderer: Renderer,
    last_frame: Instant,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let window = Window::new("Rust IDE", &self.config.window, event_loop);

        self.renderer = Renderer::new(&window);
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let now = Instant::now();
        let delta = now - self.last_frame;
        self.last_frame = now;

        if let Some(window) = self.window.as_mut() {
            self.renderer
                .update(delta, window_id, window, event.clone());
            window.update(event_loop, window_id, event.clone());
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = self.window.as_mut() {
            self.renderer.about_to_wait(&window);
            window.about_to_wait();
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        // Happens only in android
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        info!("Exiting application...");
        let window = self.window.take().expect("Window was not initialized");
        let window_size = window.get_size();
        let window_position = window.get_position();

        self.config.window = WindowConfig {
            width: window_size.width,
            height: window_size.height,
            x: window_position.x,
            y: window_position.y,
        };

        config::save(&self.config).expect("Failed to save configuration");
        self.window = None;
    }
}

impl App {
    pub fn new(config: Config) -> Self {
        App {
            config,
            window: None,
            last_frame: Instant::now(),
            renderer: Renderer::default(),
        }
    }
}
