use clap::{Arg, Command};

pub fn command_line() -> clap::ArgMatches {
  let matches = Command::new("Stella")
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .version(env!("CARGO_PKG_VERSION"))
    .author(env!("CARGO_PKG_AUTHORS"))
    .subcommand_required(true)
    .arg_required_else_help(true)
    .subcommand(
      Command::new("check")
        .about("check a stella(lua) file.")
        .arg(Arg::new("file").help("the stella(lua) file to check.").required(true)),
    )
    .subcommand(
      Command::new("compile")
        .about("compile a stella(lua) file to a native executable.")
        .arg(Arg::new("file").help("the stella(lua) file to compile.").required(true)),
    )
    .subcommand(
      Command::new("run")
        .about("run stella(lua) code.")
        .arg(Arg::new("file").help("the stella(lua) file to run.").required(true)),
    )
    .get_matches();

  return matches;
}
