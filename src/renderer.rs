use gl::types::*;
use std::mem;
use std::ptr;
use std::str;

static VERTEX_SHADER_SOURCE: &str = r#"
    #version 330 core
    layout (location = 0) in vec2 aPos;
    layout (location = 1) in vec3 aColor;

    out vec3 vertexColor;

    void main() {
        gl_Position = vec4(aPos, 0.0, 1.0);
        vertexColor = aColor;
    }
"#;

static FRAGMENT_SHADER_SOURCE: &str = r#"
    #version 330 core
    out vec4 FragColor;
    in vec3 vertexColor;

    void main() {
        FragColor = vec4(vertexColor, 1.0);
    }
"#;

#[derive(Debug, Default)]
pub struct Renderer {
    vao: u32,
    vbo: u32,
    shader_program: u32,
}

impl Renderer {
    pub fn new() -> Self {
        let (vao, vbo) = unsafe {
            let mut vao = 0;
            let mut vbo = 0;

            // Define the quad's vertices
            let vertices: [f32; 20] = [
                // Position   // Color
                -0.5, -0.5, 1.0, 0.0, 0.0, // Bottom-left
                0.5, -0.5, 0.0, 1.0, 0.0, // Bottom-right
                0.5, 0.5, 0.0, 0.0, 1.0, // Top-right
                -0.5, 0.5, 1.0, 1.0, 0.0, // Top-left
            ];

            let indices: [u32; 6] = [0, 1, 2, 2, 3, 0];

            let mut ebo = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * mem::size_of::<GLuint>()) as GLsizeiptr,
                indices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // Position attribute
            gl::VertexAttribPointer(
                0,
                2,
                gl::FLOAT,
                gl::FALSE,
                5 * mem::size_of::<GLfloat>() as GLsizei,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            // Color attribute
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                5 * mem::size_of::<GLfloat>() as GLsizei,
                (2 * mem::size_of::<GLfloat>()) as *const _,
            );
            gl::EnableVertexAttribArray(1);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

            (vao, vbo)
        };

        let shader_program = unsafe { Self::create_shader_program() };

        Renderer {
            vao,
            vbo,
            shader_program,
        }
    }

    unsafe fn create_shader_program() -> u32 {
        let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
        gl::ShaderSource(
            vertex_shader,
            1,
            &(VERTEX_SHADER_SOURCE.as_ptr() as *const i8),
            ptr::null(),
        );
        gl::CompileShader(vertex_shader);
        Self::check_shader_errors(vertex_shader);

        let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
        gl::ShaderSource(
            fragment_shader,
            1,
            &(FRAGMENT_SHADER_SOURCE.as_ptr() as *const i8),
            ptr::null(),
        );
        gl::CompileShader(fragment_shader);
        Self::check_shader_errors(fragment_shader);

        let shader_program = gl::CreateProgram();
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);
        gl::LinkProgram(shader_program);
        Self::check_program_errors(shader_program);

        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);

        shader_program
    }

    unsafe fn check_shader_errors(shader: u32) {
        let mut success = gl::FALSE as GLint;
        let mut info_log = [0; 512];
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(
                shader,
                512,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            println!(
                "Shader Compilation Error: {:?}",
                String::from_utf8_lossy(&info_log)
            );
        }
    }

    unsafe fn check_program_errors(program: u32) {
        let mut success = gl::FALSE as GLint;
        let mut info_log = [0; 512];
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetProgramInfoLog(
                program,
                512,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            println!(
                "Program Linking Error: {:?}",
                String::from_utf8_lossy(&info_log)
            );
        }
    }

    pub fn draw(&self) {
        unsafe {
            unsafe {
                gl::ClearColor(0.1, 0.2, 0.3, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            gl::UseProgram(self.shader_program);
            gl::BindVertexArray(self.vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
        }
    }
}
