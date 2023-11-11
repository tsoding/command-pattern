extern crate command;
use std::env;
use std::process::ExitCode;
use command::{command, command_list};

fn usage(program: &str) {
    eprintln!("Usage: {program} <command>");
    eprintln!("Commands:");
    for command in COMMANDS.iter() {
        eprintln!("    {name} - {description}", name = command.name, description = command.description);
    }
}

struct Command {
    name: &'static str,
    description: &'static str,
    run: fn(&str, env::Args) -> ExitCode,
}

#[command("hello", "Prints \"Hello, World\"")]
fn hello_command(_program: &str, _args: env::Args) -> ExitCode {
    println!("Hello, World");
    ExitCode::SUCCESS
}

#[command("reverse", "Reverses characters of the arguments")]
fn reverse_command(_program: &str, args: env::Args) -> ExitCode {
    for arg in args {
        println!("{}", arg.chars().rev().collect::<String>())
    }
    ExitCode::SUCCESS
}

#[command("uppercase", "Brings all the arguments to uppercase")]
fn uppercase_command(_program: &str, args: env::Args) -> ExitCode {
    for arg in args {
        println!("{}", arg.to_uppercase());
    }
    ExitCode::SUCCESS
}

#[command("test", "This is a test command")]
fn test_command(_program: &str, _args: env::Args) -> ExitCode {
    println!("Test");
    ExitCode::SUCCESS
}

#[command("help", "Print this help messsage")]
fn help_command(program: &str, mut args: env::Args) -> ExitCode {
    if let Some(command_name) = args.next() {
        if let Some(command) = COMMANDS.iter().find(|command| command.name == command_name) {
            println!("{name} - {description}", name = command.name, description = command.description);
        } else {
            eprintln!("ERROR: command {command_name} is not found");
            return ExitCode::FAILURE;
        }
    } else {
        usage(&program);
    }
    ExitCode::SUCCESS
}

const COMMANDS: &[Command] = command_list!();

fn main() -> ExitCode {
    let mut args = env::args();
    let program = args.next().expect("program");
    if let Some(command_name) = args.next() {
        if let Some(command) = COMMANDS.iter().find(|command| command.name == command_name) {
            (command.run)(&program, args);
        } else {
            usage(&program);
            eprintln!("ERROR: command {command_name} is unknown");
            return ExitCode::FAILURE;
        }
    } else {
        usage(&program);
        eprintln!("ERROR: no command is provided");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
