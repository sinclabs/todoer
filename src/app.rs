use std::fmt::Result;

use crossterm::{
  event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
  backend::{Backend, CrosstermBackend},
  layout::{Constraint, Direction, Layout},
  style::{Color, Modifier, Style},
  text::{Span, Spans, Text},
  widgets::{Block, Borders, List, ListItem, Paragraph},
  Frame, Terminal,
};
use unicode_width::UnicodeWidthStr;

enum InputMode {
  Normal,
  Editing,
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

pub struct App {
  /// Current value of the input box
  input: String,
  /// Current input mode
  input_mode: InputMode,
  /// History of recorded messages
  messages: Vec<String>,
}

impl Default for App {
  fn default() -> App {
      App {
          input: String::new(),
          input_mode: InputMode::Normal,
          messages: Vec::new(),
      }
  }
}

impl App {
  pub fn run(&self) -> Result {

    Ok(())
  }
}