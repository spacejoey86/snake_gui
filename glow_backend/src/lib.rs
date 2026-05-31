mod widgets;

use glow::{Context, HasContext};

pub struct GlowBackendContext {
    gl: Context,
}

const VERTEX_SHADER_SOURCE: &str = "
#version 330 core
layout (location = 0) in vec2 vertexPos;

out vec3 fColor;

void main()
{
    gl_Position = vec4(vertexPos, 0.0, 1.0);
    fColor = vec3(1.0, 0.0, 1.0);
}
";

const FRAGMENT_SHADER_SOURCE: &str = "
# version 330 core
out vec4 FragColor;

in vec3 fColor;

void main()
{
    FragColor = vec4(fColor, 1.0);
}
";

impl GlowBackendContext {
    pub fn new(gl: Context) -> Self {
        unsafe {
            // upload shaders
            let shader_program = gl.create_program().unwrap();
            let shader_sources = [
                (glow::VERTEX_SHADER, VERTEX_SHADER_SOURCE),
                (glow::FRAGMENT_SHADER, FRAGMENT_SHADER_SOURCE),
            ];
            let mut shaders = vec![];
            for (shader_type, shader_source) in shader_sources.iter() {
                let shader = gl.create_shader(*shader_type).unwrap();
                gl.shader_source(shader, *shader_source);
                gl.compile_shader(shader);
                gl.attach_shader(shader_program, shader);
                shaders.push(shader);
            }
            gl.link_program(shader_program);
            for shader in shaders {
                gl.detach_shader(shader_program, shader);
                gl.delete_shader(shader);
            }
            gl.use_program(Some(shader_program));

            // create buffers?
            // vertices of my rectangle
            let vertex_array = gl.create_vertex_array().unwrap(); // this is the state, stores which vertex buffer is used
            gl.bind_vertex_array(Some(vertex_array)); // do I need to bind once here, and/or every render call?
            let vertex_buffer = gl.create_buffer().unwrap(); // this actually stores the vertices
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer));
            // the vertices are an attribute passed to the vertex shader:
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 0, 0);

            let vertices: [f32; _] = [-1.0, 1.0, 0.5, 1.0, -1.0, 0.3, 0.5, 0.3];
            let vertices_u8 = core::slice::from_raw_parts(
                vertices.as_ptr() as *const u8,
                vertices.len() * core::mem::size_of::<f32>(),
            );
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vertices_u8, glow::STATIC_DRAW);

            // some settings
            gl.disable(glow::FRAMEBUFFER_SRGB);
            gl.disable(glow::BLEND);

            // todo: font texture

            Self { gl }
        }
    }

    pub fn render(&self) {
        unsafe {
            self.gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);
        }
    }
}
