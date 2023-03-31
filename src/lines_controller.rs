use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::traits::{CursorC, LineC};

pub struct LinesController {
    pub header: Vec<String>,
    pub lines: Vec<String>,
}

impl LinesController {
    pub fn new(lines: Vec<String>, header: Vec<String>) -> LinesController {
        LinesController { header, lines }
    }
    fn add_line<C: CursorC>(&mut self, cursor_controller: &mut C) {
        let part_2 = (&self.lines[cursor_controller.get_y()]
            [cursor_controller.get_x()..self.lines[cursor_controller.get_y()].len()])
            .to_string();
        self.lines.insert(cursor_controller.get_y() + 1, part_2);
        let part_1 =
            (&self.lines[cursor_controller.get_y()][0..cursor_controller.get_x()]).to_string();
        self.lines[cursor_controller.get_y()] = part_1;

        cursor_controller
            .change_y(cursor_controller.get_y() + cursor_controller.get_min_height() + 1);
        cursor_controller.change_x(0);
    }

    fn typing<C: CursorC>(&mut self, char: char, cursor_controller: &mut C) {
        self.lines[cursor_controller.get_y()].insert(cursor_controller.get_x(), char);
        cursor_controller.change_x(cursor_controller.get_x() + 1);
    }
    fn backspace<C: CursorC>(&mut self, cursor_controller: &mut C) {
        if cursor_controller.get_x() != 0 {
            self.lines[cursor_controller.get_y()].remove(cursor_controller.get_x() - 1);
            cursor_controller.change_x(cursor_controller.get_x() - 1);
        } else if cursor_controller.get_y() + cursor_controller.get_min_height()
            != cursor_controller.get_min_height()
        {
            let part_1 = &self.lines[cursor_controller.get_y() - 1];
            let part_2 = &self.lines[cursor_controller.get_y()];
            cursor_controller.change_x(part_1.len());
            let full_part = part_1.to_string() + part_2;
            self.lines[cursor_controller.get_y() - 1] = full_part;
            self.lines.remove(cursor_controller.get_y());

            cursor_controller
                .change_y(cursor_controller.get_y() + cursor_controller.get_min_height() - 1);
        }
    }
    fn delete<C: CursorC>(&mut self, cursor_controller: &mut C) {
        if cursor_controller.get_x() != self.lines[cursor_controller.get_y()].len() {
            self.lines[cursor_controller.get_y()].remove(cursor_controller.get_x());
        } else if cursor_controller.get_y() != self.lines.len() - 1 {
            let part_1 = &self.lines[cursor_controller.get_y()];

            let part_2 = &self.lines[cursor_controller.get_y() + 1];
            self.lines[cursor_controller.get_y()] = part_1.to_string() + part_2;
            self.lines.remove(cursor_controller.get_y() + 1);
        }
    }
}
impl LineC for LinesController {
    fn get_lines(&self) -> &Vec<String> {
        &self.lines
    }

    fn get_header(&self) -> &Vec<String> {
        &self.header
    }
    fn change_lines(&mut self, lines: Vec<String>) {
        self.lines = lines;
    }
    fn change_title(&mut self, title: String) {
        self.header[0] = title;
    }
    fn change_status(&mut self, status: String) {
        self.header[1] = status;
    }
    fn take_input<C: CursorC>(&mut self, event: KeyEvent, cursor_controller: &mut C) {
        self.change_status("Not Saved".to_string());
        match event {
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                ..
            } => self.add_line(cursor_controller),

            KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
                ..
            } => self.backspace(cursor_controller),

            KeyEvent {
                code: KeyCode::Delete,
                modifiers: KeyModifiers::NONE,
                ..
            } => self.delete(cursor_controller),
            KeyEvent {
                code: _a @ (KeyCode::Char('å') | KeyCode::Char('æ') | KeyCode::Char('ø')),
                ..
            } => {}
            KeyEvent {
                code: KeyCode::Char(char),
                ..
            } => self.typing(char, cursor_controller),
            _ => {}
        }
    }
}
