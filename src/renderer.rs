use crate::{
    colour_holder::ColourHolder,
    states::States,
    traits::{CursorC, LineC},
};
use crossterm::{
    cursor, queue,
    style::{self, Color},
    terminal,
};
use std::io::{Stdout, Write};
pub struct Renderer {}

impl Renderer {
    pub fn new() -> Renderer {
        Renderer {}
    }

    fn render_line(
        &self,
        stdout: &mut Stdout,
        line: &str,
        foreground_colour: Color,
        background_colour: Color,
    ) -> Result<bool, exitfailure::ExitFailure> {
        queue!(
            stdout,
            style::SetForegroundColor(foreground_colour),
            style::SetBackgroundColor(background_colour),
            style::Print(line),
            terminal::Clear(terminal::ClearType::UntilNewLine),
            cursor::MoveToNextLine(1)
        )?;
        Ok(true)
    }
    fn render_lines<C: CursorC>(
        &self,
        stdout: &mut Stdout,
        lines: &Vec<String>,
        colour_holder: &ColourHolder,
        cursor_controller: &C,
    ) -> Result<bool, exitfailure::ExitFailure> {
        let mut return_bool = true;

        lines.iter().enumerate().for_each(|(pos, line)| {
            let colour = if pos == cursor_controller.get_y() {
                colour_holder.highlight_colour
            } else {
                colour_holder.background_colour
            };
            let res = self.render_line(stdout, line, colour_holder.foreground_colour, colour);
            match res {
                Ok(_) => {}
                Err(_) => return_bool = false,
            }
        });

        Ok(return_bool)
    }

    fn render_header(
        &self,
        stdout: &mut Stdout,
        lines: &Vec<String>,
        colour_holder: &ColourHolder,
    ) -> Result<bool, exitfailure::ExitFailure> {
        if lines.len() > 0 {
            let mut return_bool = true;
            let header = (&lines[0]).to_string();
            let mut header_line = (&header).to_string();
            let mut status_line;
            let linus;

            for _ in 0..(terminal::size()?.0 / 2 - (header_line.len() / 2) as u16) {
                header_line.insert(0, ' ');
            }
            if lines.len() > 1 {
                status_line = (&lines[1]).to_string();

                for _ in 0..(terminal::size()?.0 / 2 - (status_line.len() / 2) as u16) {
                    status_line.insert(0, ' ');
                }

                linus = vec![header_line, status_line];
            } else {
                linus = vec![header]
            }

            linus.iter().for_each(|line| {
                let res = self.render_line(
                    stdout,
                    line,
                    colour_holder.foreground_colour,
                    colour_holder.background_colour,
                );
                match res {
                    Ok(_) => {}
                    Err(_) => return_bool = false,
                }
            });

            Ok(return_bool)
        } else {
            Ok(true)
        }
    }
    pub fn render_all<L: LineC, C: CursorC>(
        &self,
        stdout: &mut Stdout,
        lines_controller: &L,
        colour_holder: &ColourHolder,
        cursor_controller: &C,
        state: &States,
    ) -> Result<bool, exitfailure::ExitFailure> {
        queue!(stdout, cursor::MoveTo(0, 0))?;
        let y;

        self.render_header(stdout, &lines_controller.get_header(), colour_holder)?;

        self.render_lines(
            stdout,
            &lines_controller.get_lines(),
            colour_holder,
            cursor_controller,
        )?;

        match state {
            States::Setup => {
                y = 0;
            }
            States::Editor => {
                y = cursor_controller.get_y() + cursor_controller.get_min_height();
            }
        }

        queue!(
            stdout,
            style::ResetColor,
            terminal::Clear(terminal::ClearType::UntilNewLine),
            cursor::MoveTo((cursor_controller.get_x()) as u16, y as u16)
        )?;

        stdout.flush()?;
        Ok(true)
    }
}
