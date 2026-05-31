use glfw::{Action, Context, Key, fail_on_errors};
use glow_backend::GlowBackendContext;

fn main() {
    let mut glfw = glfw::init(fail_on_errors!()).unwrap();
    let (mut window, events) = glfw
        .create_window(300, 300, "angui", glfw::WindowMode::Windowed)
        .expect("Failed to create window");
    window.make_current();
    window.set_key_polling(true);

    // set up the renderer
    let glow_context = unsafe {
        glow::Context::from_loader_function(|string| {
            window.get_proc_address(string).unwrap() as *const _
        })
    };
    let backend_context = GlowBackendContext::new(glow_context);
    backend_context.set_window_size(window.get_size().0, window.get_size().1);

    // Run the app:
    while !window.should_close() {
        backend_context.display();

        window.swap_buffers();

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }
    }
}
