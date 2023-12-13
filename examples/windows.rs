use crossterm::event::KeyCode;
use console_engine::window_manager::WindowManager;
use console_engine::ConsoleEngine;

fn main() {
    let mut wm = WindowManager::new();
    let mut engine = ConsoleEngine::init(30, 20, 60).unwrap();

    loop {
        engine.wait_frame();
        engine.check_resize();
        if engine.is_key_pressed(KeyCode::Char('q')) {
            break;
        }
        if let Some(mouse_pos) = engine.get_mouse_press(console_engine::MouseButton::Right) {
            wm.add_window("Test".into(), mouse_pos.0 as i32, mouse_pos.1 as i32, 30, 9);
        }
        engine.clear_screen();
        engine.print(2, 2, "Right click to create windows!");
        wm.handle_input(&mut engine);
        wm.render_windows(&mut engine);
        engine.draw();
    }
}
