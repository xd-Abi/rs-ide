use crate::config;
use crate::config::{Config, WindowConfig};
use crate::window::Window;
use glutin::context::{
    ContextApi, ContextAttributesBuilder, NotCurrentContext, NotCurrentGlContext,
    PossiblyCurrentContext, PossiblyCurrentGlContext, Version,
};
use glutin::display::{Display, GetGlDisplay, GlDisplay};
use glutin::prelude::GlSurface;
use glutin::surface::{Surface, WindowSurface};
use glutin_winit::{DisplayBuilder, GlWindow};
use std::ffi::{CStr, CString};
use tracing::info;
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalPosition};
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::raw_window_handle::HasWindowHandle;
use winit::window::WindowId;
use crate::renderer::Renderer;

#[derive(Debug, Default)]
pub struct App {
    config: Config,
    window: Option<Window>,
    renderer: Renderer
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        self.window = Some(Window::new("Rust IDE", &self.config.window, event_loop));
        self.renderer = Renderer::new();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if let Some(mut window) = self.window.as_mut() {
            window.update(event_loop, window_id, event.clone());
        }

        match event {
            WindowEvent::Resized(size) => unsafe {
                gl::Viewport(
                    0,
                    0,
                    size.width as gl::types::GLsizei,
                    size.height as gl::types::GLsizei,
                );
            },
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = self.window.as_mut() {
            window.draw();
            self.renderer.draw();
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
            ..Default::default()
        }
    }
}
