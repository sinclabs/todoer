pub mod todo;
pub mod app;

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{CrosstermBackend},
    Terminal,
};
use todo::Todo;
use app::App;

fn main() -> Result<(), Box<dyn Error>> {
    let todos = [
        Todo::parse(&String::from("x A 2022-05-01 Some content at @place and +project")),
        Todo::parse(&String::from("A 2022-05-01 Some other content at @otherPlace and +otherProject"))
    ];

    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = App::default();
    let res = app.run();

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

