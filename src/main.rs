#[macro_use]
extern crate clap;

mod output;

use std::env;

fn main() {
  let matches = clap_app!(app =>
    (name: "kunai")
    (version: "0.1.0")
    (@subcommand build =>
      (about: "Builds an executable from the specified module")
      (@arg INPUT: +required "The module to build")
    )
  ).get_matches();

  if let Some(matches) = matches.subcommand_matches("build") {
    let input = matches.value_of("INPUT").unwrap();
    let cwd = env::current_dir();

    if cwd.as_ref().is_err() {
      output::error_error(cwd.as_ref().err().unwrap());
    }
  }
}
