pub mod basic_cursor_controller;
pub mod basic_line_controller;
pub mod colour_holder;
pub mod cursor_controller;
pub mod lines_controller;
pub mod reader;
pub mod renderer;
pub mod state_controller;
pub mod states;
pub mod supabase;
pub mod traits;

use std::io::{self, Stdout};

use basic_cursor_controller::BaseCusorController;
use basic_line_controller::BaseLineController;
use clap::Parser;
use colour_holder::ColourHolder;
use crossterm::{cursor, queue, style::Color, terminal};
use cursor_controller::CursorController;
use dotenv;
use lines_controller::LinesController;
use reader::Reader;
use state_controller::StateController;
use states::States;
use supabase::Supabase;
use traits::LineC;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(short, long)]
    title: Option<Option<String>>,
}

#[tokio::main]

async fn main() -> Result<(), exitfailure::ExitFailure> {
    dotenv::dotenv().ok();
    let args = Cli::parse();
    let colour_holder = ColourHolder {
        foreground_colour: Color::White,
        background_colour: Color::Black,
        highlight_colour: Color::Blue,
    };
    let mut content: Vec<String> = vec!["".to_string()];

    let url = std::env::var("BASE_URL").expect("Sheis");
    let api_key = std::env::var("API_KEY").expect("C");
    let supabase = Supabase::new(url, api_key)?;

    let res = supabase.get_all_headers("notes").await?;
    let mut tit = "A";
    let header_list: Vec<String> = res
        .iter()
        .map(|header| (&header.header).to_string())
        .collect();
    let state;
    match &args.title {
        Some(title) => match title {
            Some(title) => {
                state = States::Editor;
                if header_list.contains(title) {
                    let resp = supabase.get_from_header("notes", title).await?;

                    let fisk: Vec<&str> = resp[0].content.split("\n").collect();
                    content = fisk.iter().map(|s| s.to_string()).collect();
                    content.pop();
                    tit = title;
                } else {
                    supabase
                        .post_text("notes", title.to_owned(), "".to_string())
                        .await?;
                    content = vec!["".to_string()];
                    tit = title;
                }
            }
            None => panic!("Please enter a title"),
        },
        None => {
            state = States::Setup;
        }
    }

    let mut lines_controller = LinesController::new(
        content,
        vec![tit.to_string(), "It is working suiii".to_string()],
    );
    let mut cursor_controller = CursorController::new(lines_controller.get_header().len(), 0);
    let mut base_line = BaseLineController::new(header_list, vec!["".to_string()]);
    let mut base_cursor = BaseCusorController::new(base_line.get_header().len());

    let mut status = StateController::new(state, colour_holder, supabase, "notes".to_string());

    let mut stdout = io::stdout();

    let mut reader = Reader::new();

    setup(&mut stdout)?;
    while status
        .run(
            &mut stdout,
            &mut lines_controller,
            &mut cursor_controller,
            &mut base_line,
            &mut base_cursor,
            &mut reader,
        )
        .await?
    {}
    quit(&mut stdout)?;
    Ok(())
}

fn setup(stdout: &mut Stdout) -> Result<(), exitfailure::ExitFailure> {
    terminal::enable_raw_mode()?;
    queue!(stdout, terminal::Clear(terminal::ClearType::All))?;
    Ok(())
}

fn quit(stdout: &mut Stdout) -> Result<(), exitfailure::ExitFailure> {
    terminal::disable_raw_mode()?;
    queue!(
        stdout,
        cursor::MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::All)
    )?;
    Ok(())
}
