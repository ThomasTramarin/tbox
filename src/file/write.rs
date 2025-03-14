use std::{
    fs::{self, OpenOptions},
    io::{self, stdin, Write},
    path::Path,
};

use clap::{Arg, ArgAction, ArgMatches, Command};
use colored::Colorize;

pub fn write_file_command() -> Command {
    Command::new("write")
        .about("Write content to a file with the path provided")
        .arg(
            Arg::new("pathname")
                .required(true)
                .index(1)
                .help("The path of the file to write to"),
        )
        .arg(
            Arg::new("content")
                .required(true)
                .index(2)
                .help("The content to write to the file"),
        )
        .arg(
            Arg::new("append")
                .short('a')
                .long("append")
                .action(ArgAction::SetTrue)
                .help("Append content to the file"),
        )
        .arg(
            Arg::new("force")
                .short('f')
                .long("force")
                .action(ArgAction::SetTrue)
                .help("Force write without confirmation"),
        )
}

pub fn write_file(matches: &ArgMatches) {
    // Get arguments
    let pathname = matches.get_one::<String>("pathname").unwrap();
    let content = matches.get_one::<String>("content").unwrap();
    let append = matches.get_flag("append");
    let force = matches.get_flag("force");

    let content = content.replace(r"\n", "\n");

    // Check if the file exists
    let path = Path::new(pathname);
    if !path.exists() {
        println!("{}", "File does not exist.".red());
        return;
    }

    // Confirm acqion before writing
    if !force && !confirm_overwrite(append, pathname) {
        println!("{}", "Write canceled.".red());
        return;
    }

    // Write
    match if append {
        append_to_file(pathname, &content)
    } else {
        overwrite_file(pathname, &content)
    } {
        Ok(_) => println!("{}", "File written successfully.".green()),
        Err(e) => eprintln!("{}", format!("Error: {}", e).red()),
    }
}

// Function to confirm overwrite action
fn confirm_overwrite(append: bool, pathname: &str) -> bool {
    let action = if append { "append to" } else { "overwrite" };
    println!(
        "{}",
        format!(
            "Are you sure you want to {} the file {}? (y/n)",
            action, pathname
        )
        .yellow()
    );

    let mut input = String::new();
    if stdin().read_line(&mut input).is_err() || input.trim().to_lowercase() != "y" {
        false
    } else {
        true
    }
}

// Function to overwrite a file
fn overwrite_file(pathname: &str, content: &str) -> io::Result<()> {
    fs::write(pathname, content)
}

// Function to append content to a file
fn append_to_file(pathname: &str, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new().append(true).open(pathname)?;
    file.write_all(content.as_bytes())
}
