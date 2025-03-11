mod hitbox;

use crate::hitbox::HitBox;
use glutin::config::{Config, ConfigTemplateBuilder, GetGlConfig, GlConfig};
use glutin::context::{
    ContextApi, ContextAttributesBuilder, NotCurrentContext, NotCurrentGlContext,
    PossiblyCurrentContext, PossiblyCurrentGlContext, Version,
};
use glutin::display::{Display, GetGlDisplay, GlDisplay};
use glutin::prelude::GlSurface;
use glutin::surface::{Surface, WindowSurface};
use glutin_winit::{DisplayBuilder, GlWindow};
use std::ffi::{CStr, CString};
use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalPosition};
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::raw_window_handle::HasWindowHandle;
use winit::window::{CursorIcon, Window, WindowId};

struct WindowData {
    window: Window,
    context: Option<PossiblyCurrentContext>,
    surface: Surface<WindowSurface>,
}

#[derive(Default)]
struct App {
    window: Option<WindowData>,
    mouse: Option<PhysicalPosition<f64>>,
    is_initialized: bool,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.is_initialized {
            return;
        }

        let window_attributes = winit::window::WindowAttributes::default()
            .with_title("My Window")
            .with_decorations(false)
            .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));

        let template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)
            .with_transparency(false);

        let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes));

        let (window, gl_config) =
            match display_builder.build(event_loop, template, gl_config_picker) {
                Ok((window, gl_config)) => (window.expect("Failed to create window"), gl_config),
                Err(_) => {
                    // @TODO: Logging
                    event_loop.exit();
                    return;
                }
            };

        println!("Picked a config with {} samples", gl_config.num_samples());
        let gl_context = create_gl_context(&window, &gl_config).treat_as_possibly_current();
        let surface_attributes = window
            .build_surface_attributes(Default::default())
            .expect("Failed to build surface attributes");

        let gl_surface = unsafe {
            gl_config
                .display()
                .create_window_surface(&gl_config, &surface_attributes)
                .expect("Failed to create window surface")
        };

        gl_context
            .make_current(&gl_surface)
            .expect("Failed to make context current");

        unsafe {
            gl::load_with(|symbol| {
                let c_string = CString::new(symbol).expect("Failed to convert symbol to CString");
                gl_surface.display().get_proc_address(&c_string).cast()
            });

            let version = CStr::from_ptr(gl::GetString(gl::VERSION) as *const i8);
            println!("OpenGL Version: {}", version.to_string_lossy());
        }

        self.is_initialized = true;
        self.window = Some(WindowData {
            window,
            context: Some(gl_context),
            surface: gl_surface,
        });
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let border_threshold = 10.0;
        let title_bar_thickness = 55.0;

        match event {
            WindowEvent::Resized(size) => {
                if size.width == 0 && size.height == 0 {
                    return;
                }

                unsafe {
                    gl::Viewport(
                        0,
                        0,
                        size.width as gl::types::GLsizei,
                        size.height as gl::types::GLsizei,
                    );
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if state == ElementState::Pressed && button == MouseButton::Left {
                    if let Some(position) = self.mouse {
                        if position.x < 50.0 && position.y < 50.0 {
                            event_loop.exit();
                            return
                        }

                        if let Some(WindowData { window, .. }) = &self.window {
                            let size = window.inner_size();
                            let logical_size =
                                LogicalSize::new(size.width as f64, size.height as f64);
                            let hit_box = HitBox::from_position(
                                position,
                                logical_size,
                                border_threshold,
                                title_bar_thickness,
                            );

                            match hit_box {
                                HitBox::TitleBar => {
                                    window.drag_window().expect("Failed to grab drag window")
                                }
                                _ => {
                                    if let Some(resize_direction) = hit_box.into() {
                                        window
                                            .drag_resize_window(resize_direction)
                                            .expect("Failed to resize window");
                                    }
                                }
                            }
                        }
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse = Some(position);

                if let Some(WindowData { window, .. }) = &self.window {
                    let size = window.inner_size();
                    let logical_size = LogicalSize::new(size.width as f64, size.height as f64);
                    let hit_box = HitBox::from_position(
                        position,
                        logical_size,
                        border_threshold,
                        title_bar_thickness,
                    );
                    window.set_cursor(<HitBox as Into<CursorIcon>>::into(hit_box));
                }
            }
            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(&WindowData {
            ref window,
            ref context,
            ref surface,
            ..
        }) = self.window.as_ref()
        {
            let gl_context = context.as_ref().expect("Failed to get GL context");

            // @TODO: Draw
            unsafe {
                gl::ClearColor(0.1, 0.2, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            window.request_redraw();
            surface
                .swap_buffers(&gl_context)
                .expect("Failed to swap buffers");
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        // Happens only in android
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        println!("Exiting");
        let mut window = self.window.take().expect("Window was not initialized");
        let _gl_display = window
            .context
            .take()
            .expect("GL display was not initialized")
            .display();

        if let Display::Egl(display) = _gl_display {
            unsafe {
                display.terminate();
            }
        }

        self.window = None;
    }
}

fn create_gl_context(window: &Window, gl_config: &Config) -> NotCurrentContext {
    let raw_window_handle = window.window_handle().ok().map(|wh| wh.as_raw());

    // The context creation part.
    let context_attributes = ContextAttributesBuilder::new().build(raw_window_handle);

    // Since glutin by default tries to create OpenGL core context, which may not be
    // present we should try gles.
    let fallback_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::Gles(None))
        .build(raw_window_handle);

    // There are also some old devices that support neither modern OpenGL nor GLES.
    // To support these we can try and create a 2.1 context.
    let legacy_context_attributes = ContextAttributesBuilder::new()
        .with_context_api(ContextApi::OpenGl(Some(Version::new(2, 1))))
        .build(raw_window_handle);

    // Reuse the uncurrented context from a suspended() call if it exists, otherwise
    // this is the first time resumed() is called, where the context still
    // has to be created.
    let gl_display = gl_config.display();

    unsafe {
        gl_display
            .create_context(gl_config, &context_attributes)
            .unwrap_or_else(|_| {
                gl_display
                    .create_context(gl_config, &fallback_context_attributes)
                    .unwrap_or_else(|_| {
                        gl_display
                            .create_context(gl_config, &legacy_context_attributes)
                            .expect("failed to create context")
                    })
            })
    }
}

pub fn gl_config_picker(configs: Box<dyn Iterator<Item = Config> + '_>) -> Config {
    configs
        .reduce(|accum, config| {
            let transparency_check = config.supports_transparency().unwrap_or(false)
                & !accum.supports_transparency().unwrap_or(false);

            if transparency_check || config.num_samples() > accum.num_samples() {
                config
            } else {
                accum
            }
        })
        .unwrap()
}

fn main() {
    let event_loop = EventLoop::new().expect("Failed to create event loop");
    let mut app = App::default();

    event_loop
        .run_app(&mut app)
        .expect("Failed to run event loop");
}
