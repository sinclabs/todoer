use std::fmt::Result;

enum InputMode {
  Normal,
  Editing,
}

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