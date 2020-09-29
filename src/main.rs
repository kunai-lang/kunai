#[macro_use]
extern crate clap;

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
  }
}
