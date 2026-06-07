use angui::{
    ElementFixedSizeTrait, Position,
    pure_containers::{HorizontalContainer, PaddingContainer},
    widgets::{Button, Label, RectangleElement},
};
use glfw::{Action, Context, Key, MouseButton::Button1, fail_on_errors};
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
    let mut ctx = GlowBackendContext::new(
        glow_context,
        window.get_size().0 as u32,
        window.get_size().1 as u32,
    );

    let mut button_down = false;
    let mut second_button_down = false;

    // Run the app:
    while !window.should_close() {
        glfw.poll_events();
        window.set_mouse_button_polling(true);
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    ctx.set_window_size(width as u32, height as u32);
                }
                _ => {}
            }
        }

        ctx.clear();

        let mouse_pos = window.get_cursor_pos();
        ctx.set_mouse(
            match window.get_mouse_button(Button1) {
                Action::Press => true,
                _ => false,
            },
            Position::new(mouse_pos.0.floor() as usize, mouse_pos.1.floor() as usize),
        );

        Box::new(
            HorizontalContainer::new(10)
                .add_child(RectangleElement::new(20, 50, 7), |_, _| ())
                .add_child(RectangleElement::new(50, 200, 1), |_, _| ())
                .add_child(Button::new(button_down), |_, button_result| {
                    button_down = button_result.held;
                })
                .add_child(Button::new(second_button_down), |_, res| {
                    if res.clicked {
                        println!("clicked second button")
                    }
                    second_button_down = res.held
                })
                .add_child(
                    PaddingContainer::all(Label::new("TEST TEXT 'n'"), 4),
                    |_, _| (),
                ),
        )
        .render(&mut ctx, Position::new(0, 0));
        ctx.display();

        window.swap_buffers();
    }
}
