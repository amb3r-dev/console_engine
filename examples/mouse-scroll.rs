use console_engine::pixel;
use console_engine::KeyCode;

fn main() {
    // initializes a screen filling the terminal with a target of 30 frames per second
    let mut engine = console_engine::ConsoleEngine::init_fill_require(30, 20, 60).unwrap();

    let mut rect_x = 8;
    let mut rect_y = 3;
    let mut rect_w = 17;
    let mut rect_h = 4;

    // main loop, be aware that you'll have to break it because ctrl+C is captured
    loop {
        engine.wait_frame(); // wait for next frame + capture inputs
        engine.check_resize(); // resize the terminal if its size has changed
        if engine.is_key_pressed(KeyCode::Char('q')) {
            // if the user presses 'q' :
            break; // exits app
        }
        engine.clear_screen();

        // check if scrolling
        if let Some((directions, modifiers, _position)) = engine.is_mouse_scrolled() {
            if modifiers.contains(crossterm::event::KeyModifiers::CONTROL){
                for direction in directions {
                    match direction {
                        console_engine::ScrollDirection::UP => rect_h -= 1,
                        console_engine::ScrollDirection::DOWN => rect_h += 1,
                        console_engine::ScrollDirection::LEFT => rect_w -= 1,
                        console_engine::ScrollDirection::RIGHT => rect_w += 1,
                    }
                }
            } else if modifiers.contains(crossterm::event::KeyModifiers::SHIFT){
                for direction in directions {
                    match direction {
                        console_engine::ScrollDirection::UP => rect_y -= 3,
                        console_engine::ScrollDirection::DOWN => rect_y += 3,
                        console_engine::ScrollDirection::LEFT => rect_x -= 3,
                        console_engine::ScrollDirection::RIGHT => rect_x += 3,
                    }
                }
            } else {
                for direction in directions {
                    match direction {
                        console_engine::ScrollDirection::UP => rect_y -= 1,
                        console_engine::ScrollDirection::DOWN => rect_y += 1,
                        console_engine::ScrollDirection::LEFT => rect_x -= 1,
                        console_engine::ScrollDirection::RIGHT => rect_x += 1,
                    }
                }
            }
        }

        // print the recrangle
        engine.rect(
            rect_x,
            rect_y,
            rect_x + rect_w,
            rect_y + rect_h,
            pixel::pxl_plain('#'),
        );
        engine.print(rect_x + 4, rect_y + 2, "Scroll me!");

        engine.draw(); // draw the screen
    }
}
