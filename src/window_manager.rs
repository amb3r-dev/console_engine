#![allow(dead_code)]

use crate::pixel::pxl;
use crate::rect_style::BorderStyle;
use crate::style::Color;
use crate::Screen;

enum ResizeDir {
    Right,
    RightCorner,
    Left,
    LeftCorner,
    Bottom,
}

struct Window {
    title: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    contents: Screen,
    drag_starting_pos: (i32, i32),
    moving: bool,
    orig_window_pos: (i32, i32),
    resize_dir: Option<ResizeDir>,
    orig_window_size: (i32, i32),
    close: bool,
}
impl Window {
    pub fn get_titlebar_area(&self) -> (i32, i32, i32, i32) {
        (self.x, self.y, self.x + self.width, self.y + 2)
    }
    pub fn get_window_area(&self) -> (i32, i32, i32, i32) {
        (
            self.x,
            self.y + 2,
            self.x + self.width,
            self.y + self.height,
        )
    }
    pub fn get_contents_size(&self) -> (i32, i32) {
        (self.width - 1, self.height - 3)
    }
    pub fn get_whole_window_area(&self) -> (i32, i32, i32, i32) {
        (self.x, self.y, self.x + self.width, self.y + self.height)
    }
    pub fn get_x_button_loc(&self) -> (i32, i32) {
        (self.x + self.width - 1, self.y + 1)
    }
}

pub struct WindowManager {
    windows: Vec<Window>,
    focus_order: Vec<usize>,
}
impl WindowManager {
    pub fn new() -> Self {
        Self {
            windows: vec![],
            focus_order: vec![],
        }
    }
    pub fn add_window(&mut self, title: String, x: i32, y: i32, width: i32, height: i32) {
        self.windows.push(Window {
            title,
            x,
            y,
            width,
            height,
            contents: Screen::new_fill(
                width as u32 - 1,
                height as u32 - 3,
                pxl(' ', None, None, None),
            ),
            drag_starting_pos: (0, 0),
            moving: false,
            orig_window_pos: (0, 0),
            resize_dir: None,
            orig_window_size: (0, 0),
            close: false,
        });
        self.focus_order.insert(0, self.windows.len() - 1);
    }
    pub fn render_windows(&mut self, engine: &mut crate::ConsoleEngine) {
        // Delete closed windows in windows and focus_order
        if let Some(i) = self.focus_order.iter().position(|x| self.windows[*x].close) {
            self.windows.remove(self.focus_order[i]);
            for idx in 0..self.focus_order.len() {
                if self.focus_order[idx] > self.focus_order[i] {
                    self.focus_order[idx] -= 1;
                }
            }
            self.focus_order.remove(i);
        }
        for i in self.focus_order.as_slice() {
            let w = &self.windows[*i];
            // If this is the last element, then we're in focus
            let mut title_style = BorderStyle::new_titlebar();
            let mut border_style = BorderStyle::new_double();
            if *i == self.focus_order[self.focus_order.len() - 1] {
                title_style = BorderStyle::new_titlebar().with_colors(Color::White, Color::Reset);
                border_style = BorderStyle::new_double().with_colors(Color::White, Color::Reset);
            }
            // Window border & contents
            let (x, y, x2, y2) = w.get_window_area();
            engine.rect_border(x, y, x2, y2, border_style);
            engine.print_screen(x + 1, y + 1, &w.contents);
            // Titlebar
            let (x, y, x2, y2) = w.get_titlebar_area();
            engine.rect_border(x, y, x2, y2, title_style);
            engine.line(x + 1, y + 1, x2 - 1, y2 - 1, pxl(' ', None, None, None));
            engine.print(x + 1, y + 1, &w.title);
            // X Button
            let (x, y) = w.get_x_button_loc();
            engine
                .screen
                .set_pxl(x, y, pxl('X', Some(Color::White), Some(Color::Red), None));
        }
    }
    pub fn handle_input(&mut self, engine: &mut crate::ConsoleEngine) {
        let mut focus_order = self.focus_order.clone();
        for i in self.focus_order.iter().rev() {
            let w = &mut self.windows[*i];
            if let Some(mouse_pos) = engine.get_mouse_press(crossterm::event::MouseButton::Left) {
                let whole_window_area = w.get_whole_window_area();
                // check if mouse_pos is inside whole_window_area
                if mouse_pos.0 as i32 >= whole_window_area.0
                    && mouse_pos.0 as i32 <= whole_window_area.2
                    && mouse_pos.1 as i32 >= whole_window_area.1
                    && mouse_pos.1 as i32 <= whole_window_area.3
                {
                    // Raise focus
                    focus_order.remove(focus_order.iter().position(|x| x == i).unwrap());
                    focus_order.push(*i);

                    let titlebar_area = w.get_titlebar_area();
                    // check if mouse_pos is inside titlebar_area
                    if mouse_pos.0 as i32 >= titlebar_area.0
                        && mouse_pos.0 as i32 <= titlebar_area.2
                        && mouse_pos.1 as i32 >= titlebar_area.1
                        && mouse_pos.1 as i32 <= titlebar_area.3
                    {
                        if (mouse_pos.0 as i32, mouse_pos.1 as i32) == w.get_x_button_loc() {
                            w.close = true;
                            continue;
                        } else {
                            w.moving = true;
                            w.drag_starting_pos = (mouse_pos.0 as i32, mouse_pos.1 as i32);
                            w.orig_window_pos = (w.x, w.y);
                        }
                    }
                    // Check if mouse_pos is on the bottom-left corner of the window
                    else if mouse_pos.0 as i32 == whole_window_area.0
                        && mouse_pos.1 as i32 == whole_window_area.3
                    {
                        w.resize_dir = Some(ResizeDir::LeftCorner);
                        w.drag_starting_pos = (mouse_pos.0 as i32, mouse_pos.1 as i32);
                        w.orig_window_size = (w.width, w.height);
                        w.orig_window_pos = (w.x, w.y);
                    }
                    // Check if mouse_pos is on the bottom-right corner of the window
                    else if mouse_pos.0 as i32 == whole_window_area.2
                        && mouse_pos.1 as i32 == whole_window_area.3
                    {
                        w.resize_dir = Some(ResizeDir::RightCorner);
                        w.drag_starting_pos = (mouse_pos.0 as i32, mouse_pos.1 as i32);
                        w.orig_window_size = (w.width, w.height);
                    }
                    // Check if mouse_pos is on the bottom border of the window
                    else if mouse_pos.1 as i32 == whole_window_area.3
                        && mouse_pos.0 as i32 >= whole_window_area.0
                        && mouse_pos.0 as i32 <= whole_window_area.2
                    {
                        w.resize_dir = Some(ResizeDir::Bottom);
                        w.drag_starting_pos = (mouse_pos.0 as i32, mouse_pos.1 as i32);
                        w.orig_window_size = (w.width, w.height);
                    }
                    // Check if mouse_pos is on the left border of the window
                    else if mouse_pos.0 as i32 == whole_window_area.0
                        && mouse_pos.1 as i32 >= whole_window_area.1
                        && mouse_pos.1 as i32 <= whole_window_area.3
                    {
                        w.resize_dir = Some(ResizeDir::Left);
                        w.drag_starting_pos = (mouse_pos.0 as i32, mouse_pos.1 as i32);
                        w.orig_window_size = (w.width, w.height);
                        w.orig_window_pos = (w.x, w.y);
                    }
                    // Check if mouse_pos is on the right border of the window
                    else if mouse_pos.0 as i32 == whole_window_area.2
                        && mouse_pos.1 as i32 >= whole_window_area.1
                        && mouse_pos.1 as i32 <= whole_window_area.3
                    {
                        w.resize_dir = Some(ResizeDir::Right);
                        w.drag_starting_pos = (mouse_pos.0 as i32, mouse_pos.1 as i32);
                        w.orig_window_size = (w.width, w.height);
                    }
                    break;
                }
            }
            if let Some(mouse_pos) = engine.get_mouse_held(crossterm::event::MouseButton::Left) {
                if w.moving {
                    w.x = w.orig_window_pos.0 + mouse_pos.0 as i32 - w.drag_starting_pos.0;
                    w.y = w.orig_window_pos.1 + mouse_pos.1 as i32 - w.drag_starting_pos.1;
                }
                match w.resize_dir {
                    Some(ResizeDir::Left) => {
                        w.width = (w.orig_window_size.0 - mouse_pos.0 as i32 + w.drag_starting_pos.0).max(6);
                        w.x = w.orig_window_pos.0 + mouse_pos.0 as i32 - w.drag_starting_pos.0;
                    }
                    Some(ResizeDir::Right) => {
                        w.width = (w.orig_window_size.0 + mouse_pos.0 as i32 - w.drag_starting_pos.0).max(6);
                    }
                    Some(ResizeDir::Bottom) => {
                        w.height =
                            (w.orig_window_size.1 + mouse_pos.1 as i32 - w.drag_starting_pos.1).max(4);
                    }
                    Some(ResizeDir::LeftCorner) => {
                        w.width = (w.orig_window_size.0 - mouse_pos.0 as i32 + w.drag_starting_pos.0).max(6);
                        w.height =
                            (w.orig_window_size.1 + mouse_pos.1 as i32 - w.drag_starting_pos.1).max(4);
                        w.x = w.orig_window_pos.0 + mouse_pos.0 as i32 - w.drag_starting_pos.0;
                    }
                    Some(ResizeDir::RightCorner) => {
                        w.width = (w.orig_window_size.0 + mouse_pos.0 as i32 - w.drag_starting_pos.0).max(6);
                        w.height =
                            (w.orig_window_size.1 + mouse_pos.1 as i32 - w.drag_starting_pos.1).max(4);
                    }
                    _ => {}
                }
                let size = w.get_contents_size();
                if w.resize_dir.is_some() { w.contents.resize(size.0 as u32, size.1 as u32) }
            }
            if engine
                .get_mouse_released(crossterm::event::MouseButton::Left)
                .is_some()
            {
                w.moving = false;
                w.resize_dir = None;
            }
        }
        // Set focus order
        self.focus_order = focus_order;
    }
}
