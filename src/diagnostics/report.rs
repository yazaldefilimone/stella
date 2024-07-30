use crate::utils::{highlight_text_with_cyan, highlight_text_with_red, highlight_text_with_yellow, location::Location};

use code_highlighter::{highlight_error, highlight_warning};

pub fn report_error(message: &str, location: &mut Location, raw: &str, file_name: &str, warning: bool) {
  println!("");
  if !warning {
    let message = format!("ERROR >>> {}", message);
    println!("{}.", highlight_text_with_red(&message));
  } else {
    let warning = highlight_text_with_yellow("WARNING");
    let message = format!("{} >>> {}", warning, message);
    println!("{}", message);
  }

  let file_highlight = highlight_text_with_cyan(&file_name);
  let line_highlight = highlight_text_with_yellow(&format!(" {}:{}", location.start.line, location.end.column));
  println!("{}{}", file_highlight, line_highlight);
  println!("");
  if warning {
    let code_highliter = format!("{}", highlight_warning(location.rage_start, location.rage_end, raw));
    println!("{}", code_highliter);
  } else {
    let code_highliter = format!("{}", highlight_error(location.rage_start, location.rage_end, raw));
    println!("{}", code_highliter);
  }
  println!();
}

pub fn report_and_exit(message: &str, location: &mut Location, raw: &str, file_name: &str) -> ! {
  report_error(message, location, raw, file_name, false);
  std::process::exit(1);
}

fn is_warning(message: &str) -> bool {
  message.contains("WARNING")
}

fn is_parser_or_lexer_error(message: &str) -> bool {
  !message.contains("ERROR") && !message.contains("WARNING")
}
