pub mod todo;
pub mod app;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;
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

// fn print_todos(window: &Window, todos: &[Todo]) {
//     todos.iter().for_each(|todo | {
//         window.
//         window.printw(format!(
//             "[{}] {}\n",
//             if todo.is_complete.unwrap_or(false) { '🔥' } else { ' ' },
//             todo.content.as_ref().unwrap(),
//         ));
//     });
// }

// fn main() {
//     let todos = [
//         Todo::parse(&String::from("x A 2022-05-01 Some content at @place and +project")),
//         Todo::parse(&String::from("A 2022-05-01 Some other content at @otherPlace and +otherProject"))
//     ];

//     let window = initscr();
//     window.printw("[a] Add Todo [↑/↓] Navigate Todos [Esc] Quit\n",);
//     window.hline('-', 100);
//     window.insertln();
//     print_todos(&window, &todos);
//     window.refresh();
//     window.keypad(true);
//     noecho();
//     loop {
//         match window.getch() {
//             Some(Input::Character('a')) => {  },
//             Some(Input::Character('\u{001b}')) => break,
//             Some(input) => { window.addstr(&format!("{:?}", input)); },
//             None => ()
//         }
//     }
//     endwin();
// }
