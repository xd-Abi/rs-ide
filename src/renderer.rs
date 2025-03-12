use crate::logging::info;
use crate::window::Window;
use glow::HasContext;
use glutin::display::{GetGlDisplay, GlDisplay};
use imgui::{Condition, Context};
use imgui_glow_renderer::AutoRenderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::time::Duration;
use winit::event::{Event, WindowEvent};
use winit::raw_window_handle::HasDisplayHandle;
use winit::window::WindowId;

#[derive(Default)]
pub struct Renderer {
    imgui: Option<Context>,
    renderer: Option<AutoRenderer>,
    platform: Option<WinitPlatform>,
}

impl Renderer {
    pub fn new(window: &Window) -> Self {
        info!("Initializing renderer and ImGui...");
        let mut imgui = Context::create();
        let mut platform = WinitPlatform::new(&mut imgui);
        platform.attach_window(imgui.io_mut(), window.get_window(), HiDpiMode::Default);

        imgui.set_ini_filename(None);
        imgui
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);
        let context = window
            .get_context()
            .as_ref()
            .expect("Failed to get window context");

        let gl = unsafe {
            glow::Context::from_loader_function_cstr(|s| {
                context.display().get_proc_address(s).cast()
            })
        };

        info!("OpenGL Version: {:?}", gl.version());

        let renderer = imgui_glow_renderer::AutoRenderer::new(gl, &mut imgui)
            .expect("Failed to create imgui renderer");

        Renderer {
            imgui: Some(imgui),
            renderer: Some(renderer),
            platform: Some(platform),
        }
    }

    pub fn update(
        &mut self,
        delta: Duration,
        window_id: WindowId,
        window: &Window,
        event: WindowEvent,
    ) {
        let imgui = self.imgui.as_mut().expect("ImGui needs to be set");
        let platform = self.platform.as_mut().expect("ImGui needs to be set");
        let renderer = self
            .renderer
            .as_mut()
            .expect("ImGui renderer needs to be set");

        let wrapped_event: Event<()> = Event::WindowEvent {
            window_id,
            event: event.clone(),
        };
        platform.handle_event(imgui.io_mut(), &window.get_window(), &wrapped_event);
        imgui.io_mut().update_delta_time(delta);

        match event {
            WindowEvent::RedrawRequested => {
                unsafe {
                    renderer.gl_context().clear_color(0.1, 0.2, 0.3, 1.0);
                    renderer.gl_context().clear(glow::COLOR_BUFFER_BIT)
                };

                let ui = imgui.frame();
                ui.show_demo_window(&mut true);

                platform.prepare_render(ui, &window.get_window());
                let draw_data = imgui.render();
                renderer.render(draw_data).expect("Failed to render ImGui");
                window.swap_buffers();
            }
            _ => {}
        }
    }

    pub fn about_to_wait(&mut self, window: &Window) {
        let imgui = self.imgui.as_mut().expect("ImGui needs to be set");
        let platform = self.platform.as_mut().expect("ImGui needs to be set");

        platform
            .prepare_frame(imgui.io_mut(), &window.get_window())
            .expect("Failed to prepare frame");
    }
}
