use std::num::NonZeroU32;

use glutin::{
    config::{ConfigTemplateBuilder, GlConfig},
    context::{ContextApi, ContextAttributesBuilder, Version},
    display::GetGlDisplay,
    prelude::{GlDisplay, NotCurrentGlContext},
    surface::{GlSurface, SwapInterval},
};
use glutin_winit::{DisplayBuilder, GlWindow};
use winit::{application::ApplicationHandler, raw_window_handle::HasWindowHandle};

fn main() {
    let (gl, gl_surface, gl_context, shader_version, _window, event_loop) = {
        let event_loop = winit::event_loop::EventLoopBuilder::default()
            .build()
            .unwrap();
        let window_builder = winit::window::Window::default_attributes().with_title("angui");
        let template = ConfigTemplateBuilder::new();
        let display_builder = DisplayBuilder::new().with_window_attributes(Some(window_builder));

        let (window, gl_config) = display_builder
            .build(&event_loop, template, |configs| {
                configs
                    .reduce(|accum, config| {
                        if config.num_samples() > accum.num_samples() {
                            config
                        } else {
                            accum
                        }
                    })
                    .unwrap()
            })
            .unwrap();

        let raw_window_handle = window
            .as_ref()
            .and_then(|window| window.window_handle().map(Into::into).ok());

        let gl_display = gl_config.display();
        let context_attributes = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::OpenGl(Some(Version { major: 4, minor: 1 })))
            .build(raw_window_handle);

        unsafe {
            let not_current_gl_context = gl_display
                .create_context(&gl_config, &context_attributes)
                .unwrap();
            let window = window.unwrap();
            let attrs = window.build_surface_attributes(Default::default()).unwrap();
            let gl_surface = gl_display
                .create_window_surface(&gl_config, &attrs)
                .unwrap();
            let gl_context = not_current_gl_context.make_current(&gl_surface).unwrap();
            let gl = glow::Context::from_loader_function_cstr(|s| gl_display.get_proc_address(s));
            gl_surface
                .set_swap_interval(&gl_context, SwapInterval::Wait(NonZeroU32::new(1).unwrap()))
                .unwrap();
            (
                gl,
                gl_surface,
                gl_context,
                "#version 410",
                window,
                event_loop,
            )
        }
    };

    let mut app = App {};
    let _ = event_loop.run_app(&mut app);
}

struct App {

}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        todo!()
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        todo!()
    }
}