use crate::config::WindowConfig;
use crate::logging::info;
use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextAttributesBuilder, NotCurrentGlContext, PossiblyCurrentContext};
use glutin::display::{Display, GetGlDisplay, GlDisplay};
use glutin::prelude::GlSurface;
use glutin::surface::{Surface, SurfaceAttributesBuilder, WindowSurface};
use glutin_winit::DisplayBuilder;
use std::num::NonZeroU32;
use std::time::{Duration, Instant};
use winit::dpi::{LogicalSize, PhysicalPosition, PhysicalSize};
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
    pub fn new(title: &str, window_config: &WindowConfig, event_loop: &ActiveEventLoop) -> Self {
        info!(title = %title, "Creating window");
        let window_attributes = winit::window::WindowAttributes::default()
            .with_title(title)
            .with_decorations(false)
            .with_position(PhysicalPosition::new(window_config.x, window_config.y))
            .with_inner_size(LogicalSize::new(window_config.width, window_config.height));

        let template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)
            .with_transparency(false);

        let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_attributes));
        let (window, config) = display_builder
            .build(event_loop, template, |mut configs| {
                configs.next().expect("Config missing")
            })
            .expect("Failed to build display");

        let window = window.expect("Failed to create window");
        let window_raw_handle = window
            .window_handle()
            .expect("Failed to get raw window handle")
            .as_raw();
        let context_attributes = ContextAttributesBuilder::new().build(Some(window_raw_handle));
        let context = unsafe {
            config
                .display()
                .create_context(&config, &context_attributes)
                .expect("Failed to create OpenGL Context")
        };

        let surface_attributes = SurfaceAttributesBuilder::<WindowSurface>::new()
            .with_srgb(Some(true))
            .build(
                window_raw_handle,
                NonZeroU32::new(1024).unwrap(),
                NonZeroU32::new(768).unwrap(),
            );

        let surface = unsafe {
            config
                .display()
                .create_window_surface(&config, &surface_attributes)
                .expect("Failed to create OpenGL surface")
        };

        let context = context
            .make_current(&surface)
            .expect("Failed to make context current");

        Window {
            window,
            context: Some(context),
            surface,
            mouse: PhysicalPosition::new(0.0, 0.0),
            position: PhysicalPosition::new(window_config.x, window_config.y),
            size: PhysicalSize::new(window_config.width, window_config.height),
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

    pub fn about_to_wait(&self) {
        self.window.request_redraw();
    }

    pub fn swap_buffers(&self) {
        let context = self.context.as_ref().expect("Failed to get GL context");

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

    pub fn get_window(&self) -> &winit::window::Window {
        &self.window
    }

    pub fn get_context(&self) -> &Option<PossiblyCurrentContext> {
        &self.context
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
