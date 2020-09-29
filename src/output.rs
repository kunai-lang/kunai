extern crate termcolor;
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

use std::io::Write;

pub fn error(message: &str) {
  let bufwtr = BufferWriter::stderr(ColorChoice::Always);
  let mut buffer = bufwtr.buffer();
  buffer.set_color(ColorSpec::new().set_fg(Some(Color::Red))).expect("");
  write!(&mut buffer, "err -> ").expect("");
  buffer.set_color(ColorSpec::new().set_reset(true)).expect("");
  writeln!(&mut buffer, "{}", message).expect("");
  bufwtr.print(&buffer).expect("");
}

pub fn error_error(message: &dyn std::error::Error) {
  let bufwtr = BufferWriter::stderr(ColorChoice::Always);
  let mut buffer = bufwtr.buffer();
  buffer.set_color(ColorSpec::new().set_fg(Some(Color::Red))).expect("");
  write!(&mut buffer, "err -> ").expect("");
  buffer.set_color(ColorSpec::new().set_reset(true)).expect("");
  writeln!(&mut buffer, "{}", message.to_string()).expect("");
  bufwtr.print(&buffer).expect("");
}