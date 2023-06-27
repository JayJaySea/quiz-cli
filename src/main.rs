use std::process::ExitCode;
use quiz::error::CliError;
use quiz::controller::Controller;

use quiz::init_db::setup_database;
use clap::{command, Command, arg, value_parser, ArgMatches};
use console::style;

fn main() {
    let matches = command!()
        .arg_required_else_help(true)
        .subcommand(Command::new("add")
            .arg(
            arg!(<topic> "Topic for a task to add")
                .required(false)
                .value_parser(value_parser!(String)))
            )
        .subcommand(Command::new("delete"))
        .subcommand(Command::new("start")
            .arg(
            arg!(<topic> "Topic for a task to add")
                .required(false)
                .value_parser(value_parser!(String)))
            )
        .get_matches();

    let result = handle_subcommand(matches);

    exit(result);
}

fn handle_subcommand(matches: ArgMatches) -> Result<&'static str, CliError> {
    let conn = setup_database();
    let controller = Controller::new(conn);

    match matches.subcommand() {
        Some(("add", args)) => controller.add_question(parse_option_string(args, "topic")),
        Some(("delete", _)) => controller.delete_question(),
        Some(("start", args)) => controller.start_quiz(parse_option_string(args, "topic")),
        Some((&_, _)) => Ok(""),
        None => Ok(""),
    }
}

fn exit(result: Result<&'static str, CliError>) -> ExitCode {
    match result {
        Ok(message) => {
            eprintln!("{}", style(message).bold().green());
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{}", style(error).bold().red());
            ExitCode::FAILURE
        }
    }
}

fn parse_option_string(value: &ArgMatches, name: &str) -> Option<String> {
    value
        .get_one::<String>(name)
        .map(String::from)
}
