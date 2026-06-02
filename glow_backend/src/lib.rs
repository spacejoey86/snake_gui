mod widgets;

use glow::{Context, HasContext, NativeBuffer};

// general backend architecture
// constructing the backend does the opengl setup
// each frame, upload instance data to gpu
// everything is rendered as instanced rectangles
// rendered back to front with transparency
// rectangles sample a font texture
// for rectangles that aren't text, the font texture has an opaque section with no pattern

pub struct GlowBackendContext {
    gl: Context,
    // buffers for uploading instance data to the gpu each frame
    instance_offset_buffer: NativeBuffer,
    instance_size_buffer: NativeBuffer,
    instance_colour_buffer: NativeBuffer,
    instance_texture_offset_buffer: NativeBuffer,
    // accumulate rects during UI render calls
    // used in Self::display()
    rects: Vec<Rect>,
    // positions are in screen space, which depends on the window size
    window_width: u32,
    window_height: u32,
    // colours are picked from a palette
    colour_palette: [Colour; 256],
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
    // there is one font texture
    // at offset zero, there is a filled character
    // equivalent to not using any texture
    texture_offset_x: f32,
    texture_offset_y: f32,
}

const VERTEX_SHADER_SOURCE: &str = "
#version 330 core
layout (location = 0) in vec2 vertexPos; // same for all instances
// instance data, different for each instance
layout (location = 1) in vec2 instanceOffset;
layout (location = 2) in vec2 instanceSize;
layout (location = 3) in vec3 instanceColour;
layout (location = 4) in vec2 textureOffset;

out vec3 fColor;
out vec2 TexCoord; // gets interpolated by gpu before the fragment shader

void main()
{
    gl_Position = vec4((vertexPos * instanceSize + instanceOffset) // scale and position the rect
        * vec2(1.0, -1.0), // flip to put origin in top left
        0.0, // not using depth
        1.0); // w component, not used
    TexCoord = vec2(vertexPos.x / 28.0 + textureOffset.x, vertexPos.y + textureOffset.y); // output coordinates for texture sampling
    fColor = instanceColour;
}
";

const FRAGMENT_SHADER_SOURCE: &str = "
# version 330 core
out vec4 FragColor;

in vec3 fColor;
in vec2 TexCoord; // interpolated by the gpu

uniform sampler2D fontTexture;

void main()
{
    float texture_alpha = texture(fontTexture, TexCoord).r; // only the r component is set
    // which we use as an alpha value
    FragColor = vec4(fColor, texture_alpha);
}
";

impl GlowBackendContext {
    pub fn new(gl: Context, window_width: u32, window_height: u32) -> Self {
        // unsafe is mostly just opengl calls
        // and a few unsafe conversions of slice types
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

            // vertices of the rectangle
            // reused for every instance
            let vertex_array = gl.create_vertex_array().unwrap(); // this is the state, stores which vertex buffer is used
            gl.bind_vertex_array(Some(vertex_array));
            let vertex_buffer = gl.create_buffer().unwrap(); // this will actually contain the vertices
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer));
            // the vertices are an attribute passed to the vertex shader:
            gl.enable_vertex_attrib_array(0); // each attribure array has an index, matching the definition in the shader
            gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 0, 0);
            // upload the vertices to the gpu
            let vertices: [f32; _] = [0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0];
            let vertices_u8 = core::slice::from_raw_parts( // actual unsafe call
                vertices.as_ptr() as *const u8,
                vertices.len() * core::mem::size_of::<f32>(),
            );
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vertices_u8, glow::STATIC_DRAW);

            // instance offset buffer
            let instance_offset_buffer = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(instance_offset_buffer));
            gl.enable_vertex_attrib_array(1);
            gl.vertex_attrib_pointer_f32(1, 2, glow::FLOAT, false, 0, 0);
            gl.vertex_attrib_divisor(1, 1); // different value for every instance
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
            // instance texture offset buffer
            let instance_texture_offset_buffer = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(instance_texture_offset_buffer));
            gl.enable_vertex_attrib_array(4);
            gl.vertex_attrib_pointer_f32(4, 2, glow::FLOAT, false, 0, 0);
            gl.vertex_attrib_divisor(4, 1);

            // upload font texture
            let font_texture = gl.create_texture().unwrap();
            gl.active_texture(0);
            gl.bind_texture(glow::TEXTURE_2D, Some(font_texture));
            // not sure if wrap params are necessary?
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::NEAREST as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::NEAREST as i32,
            );
            // load texture
            let (font_data, font_width, font_height) = font_data();
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::RED as i32,
                font_width as i32,
                font_height as i32,
                0,
                glow::RED,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(Some(font_data.as_slice())),
            );

            // enable blending
            gl.enable(glow::BLEND);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);

            // set initial window size
            gl.viewport(0, 0, window_width as i32, window_height as i32);

            // set clear colour
            gl.clear_color(1.0, 1.0, 1.0, 1.0);

            Self {
                gl,
                instance_offset_buffer,
                instance_size_buffer,
                instance_colour_buffer,
                instance_texture_offset_buffer,
                rects: vec![],
                window_height,
                window_width,
                colour_palette: default_colour_palette(),
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
                        let colour = self.colour_palette[rect.colour_index as usize];
                        [colour.r, colour.g, colour.b]
                    })
                    .collect(),
                &self.gl,
            );
            upload_buffer(
                self.instance_texture_offset_buffer,
                self.rects
                    .iter()
                    .flat_map(|rect| [rect.texture_offset_x, rect.texture_offset_y])
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

const FONT_NUM_CHARACTERS: usize = 28; // constant is duplicated in vertex shader. todo: fix
const FONT_CHARS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ"; // ignoring the two special chars at the start

/// data, width, height
fn font_data() -> (Vec<u8>, u32, u32) {
    const FILE: &[u8] = include_bytes!("../font.bmp");
    let mut data_reader = FILE;
    let image = bmp::from_reader(&mut data_reader).unwrap();
    let mut output_data = vec![];
    for y in 0..image.get_height() {
        for x in 0..image.get_width() {
            let pixel = image.get_pixel(x, y);
            output_data.push(u8::MAX - pixel.r)
        }
    }
    (output_data, image.get_width(), image.get_height())
}
