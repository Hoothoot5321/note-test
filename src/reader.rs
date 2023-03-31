use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

use crate::{
    state_controller::StateController,
    traits::{CursorC, LineC},
};

pub struct Reader {}

impl Reader {
    pub fn new() -> Reader {
        Reader {}
    }
    fn read_quit(&self, event: KeyEvent) -> Result<bool, exitfailure::ExitFailure> {
        return match event {
            KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::ALT,
                ..
            } => Ok(false),
            _ => Ok(true),
        };
    }

    pub async fn read<L: LineC, C: CursorC, M: LineC, D: CursorC>(
        &self,
        lines_controller: &mut L,
        cursor_controller: &mut C,
        status_controller: &mut StateController,
        alt_line: &mut M,
        alt_cursor: &mut D,
    ) -> Result<bool, exitfailure::ExitFailure> {
        loop {
            if event::poll(Duration::from_millis(0))? {
                if let Ok(Event::Key(event)) = event::read() {
                    let return_bool = self.read_quit(event)?;
                    cursor_controller.take_input(event, lines_controller.get_lines());
                    if !status_controller
                        .take_input(
                            event,
                            lines_controller,
                            alt_line,
                            cursor_controller,
                            alt_cursor,
                        )
                        .await?
                    {
                        lines_controller.take_input(event, cursor_controller);
                    }

                    return Ok(return_bool);
                }
            }
        }
    }
}
