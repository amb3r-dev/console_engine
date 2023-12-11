use console_engine::rect_style::BorderStyle;
use console_engine::screen;
use console_engine::pixel::pxl;
use crossterm::style::Color;

fn main() {
    let mut scr = screen::Screen::new(32, 10);

    scr.rect_border(0, 0, 3, 2, BorderStyle::new_simple());
    scr.rect_border(0, 3, 3, 5, BorderStyle::new_light());
    scr.rect_border(4, 0, 7, 2, BorderStyle::new_heavy());
    scr.rect_border(4, 3, 7, 5, BorderStyle::new_double());
    scr.rect_border(8, 0, 11, 2, BorderStyle::new_solid());
    scr.rect_border(12, 2, 24, 7, BorderStyle::new_light());
    scr.rect_border(12, 0, 24, 2, BorderStyle::new_titlebar());
    scr.set_pxl(23,1,pxl('X',Some(Color::White),Some(Color::Red),None));
    scr.print(13,1,"Hello!");
    scr.print(13,3,"This rect");
    scr.print(13,4,"is styled");
    scr.print(13,5,"as a");
    scr.print(13,6,"window!");

    // print the screen to the terminal
    scr.draw();
}
