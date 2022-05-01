use regex::Regex;
use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct SpecialContentToken {
  value: String,
  index: usize,
}

#[derive(Debug)]
pub struct Todo {
  pub is_complete: Option<bool>,
  pub priority: Option<char>,
  pub creation_date: Option<NaiveDate>,
  pub completion_date: Option<NaiveDate>,
  pub projects: Vec<SpecialContentToken>,
  pub contexts: Vec<SpecialContentToken>,
  pub content: Option<String>,
}

impl Todo {
  /* 
   * World's worst ToDo.txt parser 🤣
   */
  pub fn parse(todo_string: &String) -> Todo {
    let mut todo = Todo { is_complete: None, priority: None, creation_date: None, completion_date: None, projects: [].to_vec(), contexts: [].to_vec(), content: None };
    
    let completed_re = Regex::new(r"^x$").unwrap();
    let priority_re = Regex::new(r"^[A-Z]$").unwrap();
    let plus_re = Regex::new(r"^\+").unwrap();
    let at_re = Regex::new(r"^@").unwrap();

    let mut tokens: Vec<&str> = todo_string.split_whitespace().collect();
    for i in 0..4 {
      match i {
        0 => { 
          if completed_re.is_match(tokens[0]) {
            todo.is_complete = Some(true);
            tokens.remove(0);
          }
        }
        1 => { 
          if priority_re.is_match(tokens[0]) {
            todo.priority = Some(tokens[0].chars().next().unwrap()) }
            tokens.remove(0);
          }
        2 => {
          match NaiveDate::parse_from_str(&tokens[0], "%F") {
            Ok(first_date) => {
              tokens.remove(0);

              match NaiveDate::parse_from_str(&tokens[0], "%F") {
                Ok(second_date) => {
                  todo.completion_date = Some(first_date);
                  todo.creation_date = Some(second_date);
                  tokens.remove(0);
                },
                Err(_) => { 
                  todo.creation_date = Some(first_date);
                },
              }
            },
            Err(_) => { /* Do nothing */ },
          }
        }
        3 => {
          for (index, token) in tokens.iter().enumerate() {
            if plus_re.is_match(token) { 
              todo.projects.push(SpecialContentToken { value: token.to_string(), index });
            } else if at_re.is_match(token) {
              todo.contexts.push(SpecialContentToken { value: token.to_string(), index });
            }
          }
          todo.content = Some(tokens.join(" "));
        }
        _ => {}
      } 
    }
  
    return todo;
  }
}
