use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::traits::CursorC;

pub struct CursorController {
    pub cursor_x: usize,
    pub cursor_y: usize,
    min_height: usize,
    min_width: usize,
}

impl CursorController {
    pub fn new(min_height: usize, min_width: usize) -> CursorController {
        CursorController {
            cursor_x: 0,
            cursor_y: min_height,
            min_height,
            min_width,
        }
    }
    fn move_up(&mut self) {
        if self.cursor_y > self.min_height {
            self.cursor_y -= 1;
        }
    }
    fn move_down(&mut self, max_height: usize) {
        if self.cursor_y - self.min_height < max_height {
            self.cursor_y += 1;
        }
    }
    fn check_up(&mut self, lines: &Vec<String>) {
        if self.cursor_y != self.min_height {
            if self.cursor_x > lines[self.cursor_y - self.min_height - 1].len() {
                self.cursor_x = lines[self.cursor_y - self.min_height - 1].len();
            }
        }
    }

    fn check_down(&mut self, lines: &Vec<String>) {
        if self.cursor_y - self.min_height != lines.len() - 1 {
            if self.cursor_x > lines[self.cursor_y - self.min_height + 1].len() {
                self.cursor_x = lines[self.cursor_y - self.min_height + 1].len();
            }
        }
    }
    fn move_right(&mut self, max_width: usize) {
        if self.cursor_x < max_width {
            self.cursor_x += 1;
        }
    }
    fn move_left(&mut self) {
        if self.cursor_x > self.min_width {
            self.cursor_x -= 1;
        }
    }
}
impl CursorC for CursorController {
    fn get_x(&self) -> usize {
        self.cursor_x - self.min_width
    }

    fn get_y(&self) -> usize {
        self.cursor_y - self.min_height
    }
    fn change_x(&mut self, changer: usize) {
        self.cursor_x = changer;
    }
    fn change_y(&mut self, changer: usize) {
        self.cursor_y = changer;
    }

    fn get_min_height(&self) -> usize {
        self.min_height
    }

    fn reset(&mut self, lines: &Vec<String>) {
        self.min_height = lines.len();
        self.cursor_x = 0;
        self.cursor_y = self.min_height;
    }

    fn take_input(&mut self, event: KeyEvent, lines: &Vec<String>) {
        match event {
            KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                self.check_up(lines);
                self.move_up()
            }
            KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                self.check_down(lines);
                self.move_down(lines.len() - 1)
            }
            KeyEvent {
                code: KeyCode::Left,
                modifiers: KeyModifiers::NONE,
                ..
            } => self.move_left(),
            KeyEvent {
                code: KeyCode::Right,
                modifiers: KeyModifiers::NONE,
                ..
            } => self.move_right(lines[self.cursor_y - self.min_height].len()),
            _ => {}
        }
    }
}
