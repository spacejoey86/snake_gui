mod widgets;

use glow::{Context, HasContext, NativeBuffer};

pub struct GlowBackendContext {
    gl: Context,
    instance_offset_buffer: NativeBuffer,
    instance_size_buffer: NativeBuffer,
    rects: Vec<Rect>,
    window_width: u32,
    window_height: u32,
}

struct Rect {
    offset_x: f32,
    offset_y: f32,
    width: f32,
    height: f32,
}

const VERTEX_SHADER_SOURCE: &str = "
#version 330 core
layout (location = 0) in vec2 vertexPos;
layout (location = 1) in vec2 instanceOffset;
layout (location = 2) in vec2 instanceSize;

out vec3 fColor;

void main()
{
    gl_Position = vec4((vertexPos * instanceSize + instanceOffset) * vec2(1.0, -1.0), 0.0, 1.0);
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
    pub fn new(gl: Context, window_width: u32, window_height: u32) -> Self {
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

            // create buffers

            // vertices of my rectangle
            let vertex_array = gl.create_vertex_array().unwrap(); // this is the state, stores which vertex buffer is used
            gl.bind_vertex_array(Some(vertex_array)); // do I need to bind once here, and/or every render call?
            let vertex_buffer = gl.create_buffer().unwrap(); // this actually stores the vertices
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer));
            // the vertices are an attribute passed to the vertex shader:
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 0, 0);
            // upload the vertices to the gpu
            let vertices: [f32; _] = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0];
            let vertices_u8 = core::slice::from_raw_parts(
                vertices.as_ptr() as *const u8,
                vertices.len() * core::mem::size_of::<f32>(),
            );
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vertices_u8, glow::STATIC_DRAW);

            // instance offset buffer
            let instance_offset_buffer = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(instance_offset_buffer));
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(1, 2, glow::FLOAT, false, 0, 0); // should the stride be set?
            gl.vertex_attrib_divisor(1, 1);
            // instance size buffer
            let instance_size_buffer = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(instance_size_buffer));
            gl.enable_vertex_attrib_array(2);
            gl.vertex_attrib_pointer_f32(2, 2, glow::FLOAT, false, 0, 0);
            gl.vertex_attrib_divisor(2, 1);

            // todo: font texture

            // set initial window size
            gl.viewport(0, 0, window_width as i32, window_height as i32);

            Self {
                gl,
                instance_offset_buffer,
                instance_size_buffer,
                rects: vec![
                    // Rect {
                    //     offset_x: 0.0,
                    //     offset_y: 0.0,
                    //     width: 1.0,
                    //     height: 1.0,
                    // },
                    // Rect {
                    //     offset_x: -0.5,
                    //     offset_y: -0.5,
                    //     width: 0.1,
                    //     height: 1.0,
                    // },
                    // Rect {
                    //     offset_x: -1.2,
                    //     offset_y: 0.0,
                    //     width: 1.0,
                    //     height: 1.0,
                    // },
                ],
                window_height,
                window_width,
            }
        }
    }

    pub fn display(&self) {
        unsafe {
            // upload instance attributes
            // offsets
            self.gl
                .bind_buffer(glow::ARRAY_BUFFER, Some(self.instance_offset_buffer));
            let instance_offsets: Vec<f32> = self
                .rects
                .iter()
                .flat_map(|rect| [rect.offset_x, rect.offset_y])
                .collect();
            let instance_offsets_u8 = core::slice::from_raw_parts(
                instance_offsets.as_ptr() as *const u8,
                instance_offsets.len() * core::mem::size_of::<f32>(),
            );
            self.gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                instance_offsets_u8,
                glow::STATIC_DRAW,
            );
            // sizes
            self.gl
                .bind_buffer(glow::ARRAY_BUFFER, Some(self.instance_size_buffer));
            let instance_sizes: Vec<f32> = self
                .rects
                .iter()
                .flat_map(|rect| [rect.width, rect.height])
                .collect();
            let instance_sizes_u8 = core::slice::from_raw_parts(
                instance_sizes.as_ptr() as *const u8,
                instance_sizes.len() * core::mem::size_of::<f32>(),
            );
            self.gl
                .buffer_data_u8_slice(glow::ARRAY_BUFFER, instance_sizes_u8, glow::STATIC_DRAW);

            // draw
            self.gl
                .draw_arrays_instanced(glow::TRIANGLE_STRIP, 0, 4, self.rects.len() as i32);
        }
    }

    pub fn set_window_size(&mut self, width: u32, height: u32) {
        self.window_width = width;
        self.window_height = height;
        unsafe {
            self.gl.viewport(0, 0, width as i32, height as i32);
        }
    }

    pub fn clear(&mut self) {
        self.rects.clear();
    }
}
