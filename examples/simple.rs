#![feature(path, core)]
extern crate glfw;
extern crate gl;
extern crate imgui;

use glfw::{Action, Context, Key};

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
    glfw.window_hint(glfw::WindowHint::OpenglForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenglProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) = glfw.create_window(800, 600, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.make_current();


    gl::load_with(|symbol| window.get_proc_address(symbol));

    unsafe {
        gl::ClearColor(0.3, 0.3, 1.0, 1.0);
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::Disable(gl::DEPTH_TEST);
    }

    let ui = imgui::init(&Path::new("./examples/droid_sans.ttf")).unwrap();

    let mut scroll_area_value = 0i32;
    let mut button_counter = 0u32;
    let items = vec!["Foo.txt", "Bar.md", "Baz.tar.gz"];
    let mut slider_a = 5.0f32;
    let mut slider_b = 0.5f32;
    let mut check_a = false;
    let mut check_b = true;
    let mut collapse_a = true;
    let mut collapse_b = false;
    let mut selected_file = 0;

    while !window.should_close() {

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }

        // retrieve some necessary information from glfw
        let (width, height) = window.get_size();
        let mouse_pos = {
            let (x, y) = window.get_cursor_pos();
            (x as i32, height - (y as i32))
        };
        let mouse_btn_state =
            if window.get_mouse_button(glfw::MouseButtonLeft) == Action::Press {
                imgui::MOUSE_LEFT_PRESSED
            } else {
                imgui::MOUSE_UNPRESSED
            };
        let mouse_scroll = { // emulate mouse scroll with up and down key
            if window.get_key(Key::Up) == Action::Press { -1 }
            else if window.get_key(Key::Down) == Action::Press { 1 }
            else { 0 }
        };

        // clear the screen and set the viewport to the window size
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Viewport(0, 0, width, height);
        }

        // begin ui draw call queuing and provide input state
        ui.begin(mouse_pos, mouse_btn_state, mouse_scroll, |ui| {
            ui.scroll_area("This is the label of a scroll area", (10, 10), (width/3, height-20), &mut scroll_area_value, |ui| {
                ui.separator_line();
                // Labels and values
                ui.label("Just some label");
                ui.value("some value");
                ui.separator();
                ui.separator_line();
                // Buttons illustration
                if ui.button("Click me!", true) {
                    println!("You clicked the button!");
                    button_counter += 1;
                }
                ui.value(format!("You clicked the button {} times.", button_counter).as_slice());
                ui.button("Disabled", false);
                ui.separator_line();
                ui.separator();
                // Some fun with items
                ui.label("These are items!");
                for item in items.iter() {
                    if ui.item(*item, true) {
                        println!("You selected: {}", *item);
                    }
                }
                ui.item("Another.sh (disabled)", false);
                ui.separator_line();
                ui.separator();
                // Slider madness
                ui.slider("Juice", &mut slider_a, 2.0, 7.0, 0.1, true);
                ui.slider("Water", &mut slider_b, 0.0, 1.0, 0.01, true);
                ui.separator_line();
                ui.separator();
                // Checkboxes can be used as check boxes as
                // well as radio buttons
                ui.label("Checkboxes or radiobuttons...");
                ui.label("dependent on logic behind them");
                if ui.check("Hmmmmm", check_a, true) {
                    check_a = !check_a;
                }
                if ui.check("Ahhhhh", check_b, true) {
                    check_b = !check_b;
                }
                ui.separator_line();
                ui.separator();
                // Collapse elements
                if ui.collapse("Some example section", collapse_a, true) {
                    collapse_a = !collapse_a;
                }
                if collapse_a {
                    ui.indent();
                    ui.label("Some indented label.");
                    ui.label("Lorem Ipsum and so on...");
                    ui.unindent();
                }
                if ui.collapse(format!("Selected file: {}", items[selected_file]).as_slice(), collapse_b, true) {
                    collapse_b = !collapse_b;
                }
                if collapse_b {
                    ui.indent();
                    for i in range(0, 3) {
                        if ui.item(items[i], true) {
                            selected_file = i;
                            collapse_b = !collapse_b;
                        }
                    }
                    ui.unindent();
                }
                ui.separator_line();
                ui.separator();
                ui.label("Teeeeeeeeext");
                ui.label("Teeeeeeeext");
                ui.label("Teeeeeeext");
                ui.label("Teeeeeext");
                ui.label("Teeeeext");
                ui.label("Teeeext");
            });
        });

        ui.draw_rect((width-120, 0), (50, 50), imgui::rgba(255, 0, 0, 255));
        ui.draw_text((width-300, 500), imgui::ALIGN_LEFT, "There are also basic drawing methods.", imgui::rgba(0, 255, 0, 255));
        ui.draw_line((width-220, 100), (width-200, height-200), 3.0, imgui::rgba(0, 0, 255, 255));
        ui.draw_rounded_rect((width-300, 200), (100, 100), 10.0, imgui::rgba(0, 255, 255, 255));

        ui.render(width, height);

        window.swap_buffers();
    }

}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        }
        _ => {}
    }
}
