use clap::{Arg, Command};

pub fn command_line() -> clap::ArgMatches {
  let matches = Command::new("Stella")
    .about("Speedy, lightweight lua type checker.")
    .subcommand_required(true)
    .arg_required_else_help(true)
    .author("Yazalde Filimone <yazaldefilimon@gmail.com>")
    .subcommand(
      Command::new("check")
        .about("check a lua file.")
        .arg(Arg::new("file").help("the lua file to check.").required(true)),
    )
    .subcommand(
      Command::new("compile")
        .about("transform a lua file to a native executable.")
        .arg(Arg::new("file").help("the lua file to compile.").required(true)),
    )
    .get_matches();

  return matches;
}
