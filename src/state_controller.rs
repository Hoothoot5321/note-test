use std::io::{self, Stdout};

use crossterm::{
    event::{KeyCode, KeyEvent, KeyModifiers},
    queue, terminal,
};

use crate::{
    colour_holder::ColourHolder,
    reader::Reader,
    renderer::Renderer,
    states::States,
    supabase::Supabase,
    traits::{CursorC, LineC},
};

pub struct StateController {
    pub state: States,
    pub renderer: Renderer,
    pub colour_holder: ColourHolder,
    pub supabase: Supabase,
    pub table: String,
}

impl StateController {
    pub fn new(
        state: States,
        colour_holder: ColourHolder,
        supabase: Supabase,
        table: String,
    ) -> StateController {
        StateController {
            state,
            renderer: Renderer::new(),
            colour_holder,
            supabase,
            table,
        }
    }
    pub async fn run_one<C: CursorC, L: LineC, M: LineC, D: CursorC>(
        &mut self,
        stdout: &mut Stdout,
        lines_controller: &mut L,
        cursor_controller: &mut C,
        reader: &mut Reader,
        alt_line: &mut M,
        alt_cursor: &mut D,
    ) -> Result<bool, exitfailure::ExitFailure> {
        self.renderer.render_all(
            stdout,
            lines_controller,
            &self.colour_holder,
            cursor_controller,
        )?;

        // if self.pass > 1 {
        //     saki = reader.read(lines_controller, cursor_controller, self, alt_line)?;
        // } else {
        //     self.pass += 1;
        // }
        // Ok(saki)

        reader
            .read(
                lines_controller,
                cursor_controller,
                self,
                alt_line,
                alt_cursor,
            )
            .await
    }
    pub async fn run<C: CursorC, L: LineC, D: CursorC, M: LineC>(
        &mut self,
        stdout: &mut Stdout,
        lines_controller: &mut L,
        cursor_controller: &mut C,
        base_line: &mut M,
        base_cursor: &mut D,
        reader: &mut Reader,
    ) -> Result<bool, exitfailure::ExitFailure> {
        match self.state {
            States::Setup => {
                self.run_one(
                    stdout,
                    base_line,
                    base_cursor,
                    reader,
                    lines_controller,
                    cursor_controller,
                )
                .await
            }
            States::Editor => {
                self.run_one(
                    stdout,
                    lines_controller,
                    cursor_controller,
                    reader,
                    base_line,
                    base_cursor,
                )
                .await
            }
        }
    }

    pub async fn take_input<L: LineC, C: CursorC, M: LineC, D: CursorC>(
        &mut self,
        event: KeyEvent,
        lines_controller: &mut L,
        alt_line: &mut M,
        cursor_controller: &mut C,
        alt_cursor: &mut D,
    ) -> Result<bool, exitfailure::ExitFailure> {
        match event {
            KeyEvent {
                code: KeyCode::Char('b'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                match self.state {
                    States::Setup => {
                        self.state = States::Editor;
                    }
                    States::Editor => {
                        self.state = States::Setup;

                        let header_list = self.supabase.get_all_headers(&self.table).await?;

                        let headers: Vec<String> = header_list
                            .iter()
                            .map(|header| (&header.header).to_string())
                            .collect();
                        alt_line.change_lines(headers);
                    }
                }
                queue!(io::stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
                return Ok(true);
            }
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                ..
            } => {
                match self.state {
                    States::Setup => {
                        self.state = States::Editor;

                        alt_line.change_title(
                            (&lines_controller.get_lines()[cursor_controller.get_y()]).to_owned(),
                        );

                        let header_list = self
                            .supabase
                            .get_from_header(
                                &self.table,
                                &lines_controller.get_lines()[cursor_controller.get_y()],
                            )
                            .await?;
                        let content = (&header_list[0].content).to_string();

                        let final_content: Vec<String> = content
                            .split("\n")
                            .into_iter()
                            .map(|line| line.to_string())
                            .collect();
                        alt_line.change_lines(final_content);
                        alt_line.change_status("Not Saved".to_string());
                        alt_cursor.reset(alt_line.get_header());
                    }
                    States::Editor => return Ok(false),
                }
                queue!(io::stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
                return Ok(true);
            }
            KeyEvent {
                code: KeyCode::Char('s'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => {
                match self.state {
                    States::Setup => return Ok(false),
                    States::Editor => {
                        let conent = lines_controller.get_lines().iter().fold(
                            String::new(),
                            |mut content, line| {
                                content.push_str(line);
                                content.push_str("\n");
                                content
                            },
                        );

                        self.supabase
                            .patch_from_header(
                                &self.table,
                                lines_controller.get_header()[0].to_string(),
                                conent,
                            )
                            .await?;
                        lines_controller.change_status("Saved".to_string());
                    }
                }

                queue!(io::stdout(), terminal::Clear(terminal::ClearType::All)).unwrap();
                return Ok(true);
            }

            _ => Ok(false),
        }
    }
}
