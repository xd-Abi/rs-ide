use crate::config::WindowConfig;
use crate::logging::{debug, error, info, trace, warn};
use glutin::config::{Config, ConfigTemplateBuilder, GlConfig};
use glutin::context::{
    ContextApi, ContextAttributesBuilder, NotCurrentContext, NotCurrentGlContext,
    PossiblyCurrentContext, PossiblyCurrentGlContext, Version,
};
use glutin::display::{Display, GetGlDisplay, GlDisplay};
use glutin::prelude::GlSurface;
use glutin::surface::{Surface, WindowSurface};
use glutin_winit::{DisplayBuilder, GlWindow};
use std::ffi::{CStr, CString};
use std::time::{Duration, Instant};
use winit::dpi::{LogicalSize, PhysicalPosition, PhysicalSize, Position};
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::ActiveEventLoop;
use winit::raw_window_handle::HasWindowHandle;
use winit::window::{CursorIcon, ResizeDirection, WindowId};

#[derive(Debug)]
pub struct Window {
    window: winit::window::Window,
    context: Option<PossiblyCurrentContext>,
    surface: Surface<WindowSurface>,
    mouse: PhysicalPosition<f64>,
    position: PhysicalPosition<i32>,
    size: PhysicalSize<u32>,

    // winit does not support double click, so we track the last time, where a
    // left click happened. This is needed for advanced title bar interactions.
    last_left_click: Instant,
}

#[derive(Debug, Clone, Copy)]
enum WindowInteraction {
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
    North,
    South,
    West,
    East,
    TitleBar,
    None,
}

const BORDER_THRESHOLD: f64 = 10.0;
const TITLE_BAR_THICKNESS: f64 = 55.0;
const DOUBLE_CLICK_THRESHOLD: Duration = Duration::from_millis(300);

impl Window {
    pub fn new(title: &str, config: &WindowConfig, event_loop: &ActiveEventLoop) -> Self {
        info!(title = %title, "Creating window");
        let window_attributes = winit::window::WindowAttributes::default()
            .with_title(title)
            .with_decorations(false)
            .with_position(PhysicalPosition::new(config.x, config.y))
            .with_inner_size(LogicalSize::new(config.width, config.height));

        let template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)
            .with_transparency(false);

        let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes));
        let (window, gl_config) = display_builder
            .build(event_loop, template, Self::gl_config_picker)
            .expect("Failed to build display");

        let window = window.expect("Failed to create window");
        let gl_context = Self::create_gl_context(&window, &gl_config).treat_as_possibly_current();
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
            info!("OpenGL Version: {}", version.to_string_lossy());
        }

        Window {
            window,
            context: Some(gl_context),
            surface: gl_surface,
            mouse: PhysicalPosition::new(0.0, 0.0),
            position: PhysicalPosition::new(config.x, config.y),
            size: PhysicalSize::new(config.width, config.height),
            last_left_click: Instant::now(),
        }
    }

    pub fn update(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        if self.window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::Moved(size) => {
                self.position = size.clone();
            }
            WindowEvent::Resized(size) => {
                if size.width == 0 && size.height == 0 {
                    return;
                }

                self.size = size.clone();
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if state != ElementState::Pressed || button != MouseButton::Left {
                    return;
                }

                // @TODO: Temporary close button top right corner
                if self.mouse.x < 50.0 && self.mouse.y < 50.0 {
                    event_loop.exit();
                    return;
                }

                let size = self.window.inner_size();
                let interaction = WindowInteraction::from_mouse(self.mouse, size);

                match interaction {
                    WindowInteraction::TitleBar => {
                        let now = Instant::now();
                        if now.duration_since(self.last_left_click) < DOUBLE_CLICK_THRESHOLD {
                            self.window.set_maximized(!self.window.is_maximized());
                        }

                        self.last_left_click = now;
                        self.window
                            .drag_window()
                            .expect("Failed to grab drag window")
                    }
                    _ => {
                        if self.window.is_maximized() {
                            return;
                        }

                        if let Some(resize_direction) = interaction.into() {
                            self.window
                                .drag_resize_window(resize_direction)
                                .expect("Failed to resize window");
                        }
                    }
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse = position;

                if self.window.is_maximized() {
                    return;
                }

                let size = self.window.inner_size();
                let interaction = WindowInteraction::from_mouse(self.mouse, size);
                self.window
                    .set_cursor(<WindowInteraction as Into<CursorIcon>>::into(interaction));
            }
            _ => {}
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::ClearColor(0.1, 0.2, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        let context = self.context.as_ref().expect("Failed to get GL context");

        self.window.request_redraw();
        self.surface
            .swap_buffers(context)
            .expect("Failed to swap buffers");
    }

    pub fn get_position(&self) -> PhysicalPosition<i32> {
        self.position
    }

    pub fn get_size(&self) -> PhysicalSize<u32> {
        self.size
    }

    // From glutin example
    fn create_gl_context(window: &winit::window::Window, gl_config: &Config) -> NotCurrentContext {
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
                                .expect("Failed to create context")
                        })
                })
        }
    }

    fn gl_config_picker(configs: Box<dyn Iterator<Item = Config> + '_>) -> Config {
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
}

impl Drop for Window {
    fn drop(&mut self) {
        info!("Dropping window, removing context");
        if let Some(context) = self.context.take() {
            let gl_display = context.display();

            // If using EGL, terminate the display to release resources
            if let Display::Egl(display) = gl_display {
                unsafe {
                    display.terminate();
                }
            }
        }
    }
}

impl WindowInteraction {
    pub fn from_mouse(mouse: PhysicalPosition<f64>, window_size: PhysicalSize<u32>) -> Self {
        let window_width = window_size.width as f64;
        let window_height = window_size.height as f64;

        if mouse.x < BORDER_THRESHOLD && mouse.y < BORDER_THRESHOLD {
            WindowInteraction::NorthWest
        } else if mouse.x > window_width - BORDER_THRESHOLD && mouse.y < BORDER_THRESHOLD {
            WindowInteraction::NorthEast
        } else if mouse.x < BORDER_THRESHOLD && mouse.y > window_height - BORDER_THRESHOLD {
            WindowInteraction::SouthWest
        } else if mouse.x > window_width - BORDER_THRESHOLD
            && mouse.y > window_height - BORDER_THRESHOLD
        {
            WindowInteraction::SouthEast
        } else if mouse.x < BORDER_THRESHOLD {
            WindowInteraction::West
        } else if mouse.x > window_width - BORDER_THRESHOLD {
            WindowInteraction::East
        } else if mouse.y < BORDER_THRESHOLD {
            WindowInteraction::North
        } else if mouse.y > window_height - BORDER_THRESHOLD {
            WindowInteraction::South
        } else if mouse.y < TITLE_BAR_THICKNESS {
            WindowInteraction::TitleBar
        } else {
            WindowInteraction::None
        }
    }
}

impl Into<CursorIcon> for WindowInteraction {
    fn into(self) -> CursorIcon {
        match self {
            WindowInteraction::NorthWest => CursorIcon::NwResize,
            WindowInteraction::NorthEast => CursorIcon::NeResize,
            WindowInteraction::SouthWest => CursorIcon::SwResize,
            WindowInteraction::SouthEast => CursorIcon::SeResize,
            WindowInteraction::West => CursorIcon::WResize,
            WindowInteraction::East => CursorIcon::EResize,
            WindowInteraction::North => CursorIcon::NResize,
            WindowInteraction::South => CursorIcon::SResize,
            WindowInteraction::TitleBar => CursorIcon::Default,
            WindowInteraction::None => CursorIcon::Default,
        }
    }
}

impl Into<Option<ResizeDirection>> for WindowInteraction {
    fn into(self) -> Option<ResizeDirection> {
        match self {
            WindowInteraction::NorthWest => Some(ResizeDirection::NorthWest),
            WindowInteraction::NorthEast => Some(ResizeDirection::NorthEast),
            WindowInteraction::SouthWest => Some(ResizeDirection::SouthWest),
            WindowInteraction::SouthEast => Some(ResizeDirection::SouthEast),
            WindowInteraction::West => Some(ResizeDirection::West),
            WindowInteraction::East => Some(ResizeDirection::East),
            WindowInteraction::North => Some(ResizeDirection::North),
            WindowInteraction::South => Some(ResizeDirection::South),
            _ => None,
        }
    }
}
