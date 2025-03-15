use glutin::config::ConfigTemplateBuilder;
use glutin::context::{ContextAttributesBuilder, NotCurrentGlContext, PossiblyCurrentContext};
use glutin::display::{Display, DisplayApiPreference, GlDisplay};
use glutin::surface::{GlSurface, Surface, SurfaceAttributesBuilder, WindowSurface};
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};
use std::ffi::{CStr, CString};
use std::num::NonZeroU32;
use tracing::{error, info};

#[derive(Debug)]
pub struct OpenGLGraphicsContext {
    display: Display,
    context: PossiblyCurrentContext,
    surface: Surface<WindowSurface>,
}

impl OpenGLGraphicsContext {
    pub fn new(
        window_handle: RawWindowHandle,
        display_handle: RawDisplayHandle,
        width: u32,
        height: u32,
    ) -> Self {
        info!("Initializing OpenGL context...");

        let template = ConfigTemplateBuilder::new()
            .with_alpha_size(8)
            .with_transparency(false)
            .build();

        unsafe {
            let display = Display::new(
                display_handle,
                DisplayApiPreference::Wgl(Some(window_handle)),
            )
            .expect("Failed to create OpenGL display");

            let config = display
                .find_configs(template)
                .expect("Could not find suitable OpenGL config")
                .next()
                .expect("No OpenGL config found");

            let context_attributes = ContextAttributesBuilder::new().build(Some(window_handle));

            let not_current_context = display
                .create_context(&config, &context_attributes)
                .expect("Failed to create OpenGL context");

            let surface_attributes = SurfaceAttributesBuilder::<WindowSurface>::new().build(
                window_handle,
                NonZeroU32::new(width).unwrap(),
                NonZeroU32::new(height).unwrap(),
            );

            let surface = display
                .create_window_surface(&config, &surface_attributes)
                .expect("Failed to create OpenGL surface");

            let context = not_current_context
                .make_current(&surface)
                .expect("Failed to make OpenGL context current");

            gl::load_with(|s| {
                let c_str = CString::new(s).expect("Failed to convert function name to CString");
                display.get_proc_address(&c_str) as *const _
            });

            let version_ptr = gl::GetString(gl::VERSION);
            let version_cstr = CStr::from_ptr(version_ptr as *const i8);
            match version_cstr.to_str() {
                Ok(version) => info!("OpenGL Version: {}", version),
                Err(e) => error!("Failed to convert OpenGL version string: {:?}", e),
            }

            OpenGLGraphicsContext {
                display,
                context,
                surface,
            }
        }
    }

    pub fn swap_buffers(&self) {
        self.surface
            .swap_buffers(&self.context)
            .expect("Failed to swap buffers");
    }
}
