#[macro_use]
extern crate clap;

mod output;
mod subcommands;

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
  let matches = clap_app!(app =>
    (name: "kunai")
    (version: "0.1.0")
    (@subcommand build =>
      (about: "Builds an executable from the specified module")
      (@arg INPUT: +required "The module to build")
    )
  )
  .get_matches();

  if let Some(matches) = matches.subcommand_matches("build") {
    let input = matches.value_of("INPUT").unwrap();
    let cwd = env::current_dir();

    if cwd.as_ref().is_err() {
      output::error_error(cwd.as_ref().err().unwrap());
    }

    let mut file_path = PathBuf::new();
    file_path.push(cwd.unwrap());
    file_path.push(input);

    let final_path = fs::canonicalize(file_path);

    if final_path.as_ref().is_err() {
      output::error_error(final_path.as_ref().err().unwrap());
    }

    let contents = fs::read_to_string(final_path.as_ref().ok().unwrap());

    if contents.as_ref().is_err() {
      output::error_error(contents.as_ref().err().unwrap());
    }

    let text = contents.ok().unwrap();
    let tokens = subcommands::build::tokenizer::tokenize(
      text,
      final_path.as_ref().ok().unwrap().to_str().unwrap(),
    );
  }
}
