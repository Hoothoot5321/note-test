use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::traits::CursorC;

pub struct BaseCusorController {
    pub cursor_x: usize,
    pub cursor_y: usize,
    min_height: usize,
}

impl BaseCusorController {
    pub fn new(min_height: usize) -> BaseCusorController {
        BaseCusorController {
            cursor_x: 0,
            cursor_y: min_height,
            min_height,
        }
    }
    fn move_up(&mut self) {
        if self.cursor_y > self.min_height {
            self.cursor_y -= 1;
        }
    }
    fn move_down(&mut self, max_height: usize) {
        if self.cursor_y < max_height {
            self.cursor_y += 1;
        }
    }
}
impl CursorC for BaseCusorController {
    fn get_x(&self) -> usize {
        self.cursor_x
    }
    fn get_y(&self) -> usize {
        self.cursor_y - self.min_height
    }
    fn change_x(&mut self, changer: usize) {
        self.cursor_x = changer;
    }
    fn change_y(&mut self, changer: usize) {
        self.cursor_y = changer
    }

    fn get_min_height(&self) -> usize {
        self.min_height
    }

    fn reset(&mut self, lines: &Vec<String>) {
        self.min_height = lines.len();
        self.cursor_x = 0;
        self.cursor_y = self.min_height;
    }
    fn take_input(&mut self, event: crossterm::event::KeyEvent, lines: &Vec<String>) {
        match event {
            KeyEvent {
                code: KeyCode::Up,
                modifiers: KeyModifiers::NONE,
                ..
            } => self.move_up(),
            KeyEvent {
                code: KeyCode::Down,
                modifiers: KeyModifiers::NONE,
                ..
            } => self.move_down(lines.len()),

            _ => {}
        }
    }
}
