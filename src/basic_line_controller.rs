use std::io;

use crossterm::{
    event::{KeyCode, KeyEvent, KeyModifiers},
    queue, terminal,
};

use crate::traits::{CursorC, LineC};

pub struct BaseLineController {
    pub header: Vec<String>,
    pub lines: Vec<String>,
    line_cop: Vec<String>,
}
impl BaseLineController {
    pub fn new(lines: Vec<String>, header: Vec<String>) -> BaseLineController {
        BaseLineController {
            header,
            lines: (&lines).to_owned(),
            line_cop: lines,
        }
    }

    fn update_line<C: CursorC>(&mut self, cursor_controller: &mut C) {
        if self.header[0].len() != 0 {
            self.line_cop = self
                .lines
                .iter()
                .filter(|line| line.contains(&self.header[0]))
                .map(|line| line.to_string())
                .collect();
            if (cursor_controller.get_y() + cursor_controller.get_min_height())
                > self.line_cop.len()
                && self.line_cop.len() > 0
            {
                cursor_controller.change_y(self.line_cop.len());
            }
        } else {
            self.line_cop = (&self.lines).to_owned();
        }

        queue!(io::stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
    }

    fn typing<C: CursorC>(&mut self, char: char, cursor_controller: &mut C) {
        self.header[0].insert(cursor_controller.get_x(), char);
        cursor_controller.change_x(cursor_controller.get_x() + 1);
    }
    fn backspace<C: CursorC>(&mut self, cursor_controller: &mut C) {
        if cursor_controller.get_x() != 0 {
            self.header[0].remove(cursor_controller.get_x() - 1);
            cursor_controller.change_x(cursor_controller.get_x() - 1);
        }
    }
}

impl LineC for BaseLineController {
    fn get_lines(&self) -> &Vec<String> {
        &self.line_cop
    }
    fn get_spec(&self) -> &Vec<String> {
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
        match event {
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                ..
            } => self
                .lines
                .push((&self.lines[cursor_controller.get_y()]).to_string()),

            KeyEvent {
                code: KeyCode::Backspace,
                modifiers: KeyModifiers::NONE,
                ..
            } => self.backspace(cursor_controller),
            KeyEvent {
                code: KeyCode::Char(s),
                ..
            } => self.typing(s, cursor_controller),
            _ => {}
        }
        self.update_line(cursor_controller)
    }
}
