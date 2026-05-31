mod widgets;

use glow::{Context, HasContext, NativeBuffer};

pub struct GlowBackendContext {
    gl: Context,
    instance_offset_buffer: NativeBuffer,
    instance_size_buffer: NativeBuffer,
    instance_colour_buffer: NativeBuffer,
    rects: Vec<Rect>,
    window_width: u32,
    window_height: u32,
    colour_pallete: [Colour; 256],
}

#[derive(Copy, Clone)]
pub struct Colour {
    r: f32,
    g: f32,
    b: f32,
}

struct Rect {
    offset_x: f32,
    offset_y: f32,
    width: f32,
    height: f32,
    colour_index: u8,
}

const VERTEX_SHADER_SOURCE: &str = "
#version 330 core
layout (location = 0) in vec2 vertexPos;
layout (location = 1) in vec2 instanceOffset;
layout (location = 2) in vec2 instanceSize;
layout (location = 3) in vec3 instanceColour;

out vec3 fColor;

void main()
{
    gl_Position = vec4((vertexPos * instanceSize + instanceOffset) * vec2(1.0, -1.0), 0.0, 1.0);
    fColor = instanceColour;
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
            // instance colour buffer
            let instance_colour_buffer = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(instance_colour_buffer));
            gl.enable_vertex_attrib_array(3);
            gl.vertex_attrib_pointer_f32(3, 3, glow::FLOAT, false, 0, 0);
            gl.vertex_attrib_divisor(3, 1);

            // todo: font texture

            // set initial window size
            gl.viewport(0, 0, window_width as i32, window_height as i32);

            // set clear colour
            gl.clear_color(1.0, 1.0, 1.0, 1.0);

            Self {
                gl,
                instance_offset_buffer,
                instance_size_buffer,
                instance_colour_buffer,
                rects: vec![],
                window_height,
                window_width,
                colour_pallete: default_colour_palette(),
            }
        }
    }

    pub fn display(&self) {
        unsafe {
            // upload instance attributes
            upload_buffer(
                self.instance_offset_buffer,
                self.rects
                    .iter()
                    .flat_map(|rect| [rect.offset_x, rect.offset_y])
                    .collect(),
                &self.gl,
            );
            upload_buffer(
                self.instance_size_buffer,
                self.rects
                    .iter()
                    .flat_map(|rect| [rect.width, rect.height])
                    .collect(),
                &self.gl,
            );
            upload_buffer(
                self.instance_colour_buffer,
                self.rects
                    .iter()
                    .flat_map(|rect| {
                        let colour = self.colour_pallete[rect.colour_index as usize];
                        [colour.r, colour.g, colour.b]
                    })
                    .collect(),
                &self.gl,
            );
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
        unsafe {
            self.gl.clear(glow::COLOR_BUFFER_BIT);
        }
    }
}

fn default_colour_palette() -> [Colour; 256] {
    let mut palette = [Colour {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    }; 256];
    // greys
    palette[0] = Colour {
        r: 0.0,
        g: 0.0,
        b: 0.0,
    };
    palette[1] = Colour {
        r: 0.37,
        g: 0.37,
        b: 0.42,
    }; //todo
    palette[2] = Colour {
        r: 0.37,
        g: 0.37,
        b: 0.42,
    }; //todo
    palette[3] = Colour {
        r: 0.37,
        g: 0.37,
        b: 0.42,
    }; //todo
    palette[4] = Colour {
        r: 0.37,
        g: 0.37,
        b: 0.42,
    };
    palette[5] = Colour {
        r: 0.57,
        g: 0.59,
        b: 0.64,
    };
    palette[6] = Colour {
        r: 0.77,
        g: 0.79,
        b: 0.85,
    };
    palette[7] = Colour {
        r: 0.91,
        g: 0.92,
        b: 0.96,
    };

    palette
}

unsafe fn upload_buffer(buffer: NativeBuffer, values: Vec<f32>, gl: &Context) {
    unsafe {
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(buffer));
        let values_u8 = core::slice::from_raw_parts(
            values.as_ptr() as *const u8,
            values.len() * core::mem::size_of::<f32>(),
        );
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, values_u8, glow::STATIC_DRAW);
    }
}
