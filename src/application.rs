use crate::events::Event;
use crate::platform::Window;
use std::thread::sleep;
use std::time::{Duration, Instant};
use gl::types::{GLfloat, GLsizei, GLsizeiptr, GLuint};
use tracing::info;

#[derive(Debug)]
pub struct Application {
    window: Box<Window>,
    running: bool,
}

impl Application {
    pub fn new() -> Self {
        info!("Initializing application...");
        Application {
            window: Window::new("Rust IDE", 800, 600),
            running: true,
        }
    }

    pub fn init(&mut self) {
        let app_ref = self as *mut Self;

        // @TODO: Fix event callback to avoid unsafe code
        self.window.set_event_callback(move |event| unsafe {
            (*app_ref).handle_event(event);
        });

        let vertices: [GLfloat; 12] = [
            -0.2, -0.2, 0.0,  // Bottom-left
            0.2, -0.2, 0.0,  // Bottom-right
            0.2,  0.2, 0.0,  // Top-right
            -0.2,  0.2, 0.0,  // Top-left
        ];

        let indices: [GLuint; 6] = [
            0, 1, 2,  // First triangle
            2, 3, 0,  // Second triangle
        ];

        let (mut vbo, mut vao, mut ebo) = (0, 0, 0);
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * std::mem::size_of::<GLuint>()) as GLsizeiptr,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (3 * std::mem::size_of::<GLfloat>()) as GLsizei, std::ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
    }

    pub fn run(&self) {
        let target_fps = 1.0 / 60.0;
        let frame_time = Duration::from_secs_f64(target_fps);

        while self.running {
            let start = Instant::now();

            unsafe {
                gl::ClearColor(0.8, 0.2, 0.2, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

                gl::BindVertexArray(1); // Use the VAO of the rectangle
                gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
                gl::BindVertexArray(0);
            }

            self.window.update();

            let elapsed = start.elapsed();
            if elapsed < frame_time {
                sleep(frame_time - elapsed);
            }
        }
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::WindowClose => {
                self.running = false;
            }
            Event::WindowResized(width, height) => {
                unsafe {
                    gl::Viewport(0, 0, width as i32, height as i32);
                }
            }
            Event::MouseDown(b) => {
                info!("Mouse DOWN {:?}", b);
            }
            Event::MouseRelease(b) => {
                info!("Mouse Release {:?}", b);
            }
            Event::KeyDown(k) => {
                info!("Key DOWN {:?}", k);
            }
            Event::KeyRelease(k) => {
                info!("Key Release {:?}", k);
            }
            _ => {}
        }
    }
}
