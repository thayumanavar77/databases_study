use clap::{Arg, Command};
use std::process::exit;

fn main() {
    let matches = Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg_required_else_help(true)
        .subcommand(
            Command::new("set")
                .about("Set the value of a string key to a string.")
                .arg(Arg::new("KEY").help("A string key").required(true))
                .arg(
                    Arg::new("VALUE")
                        .help("Value of a string key")
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("get")
                .about("Get the string value of a given string key.")
                .arg(Arg::new("KEY").help("A string key").required(true)),
        )
        .subcommand(
            Command::new("remove")
                .about("Remove a given string key")
                .arg(Arg::new("KEY").help("A string key").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("set", _matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        Some(("get", _matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        Some(("remove", _matches)) => {
            eprintln!("unimplemented");
            exit(1);
        }
        _ => unreachable!(),
    }
}
