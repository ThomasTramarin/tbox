use std::{
    fs::OpenOptions,
    io::{stdin, Write},
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

    // Confirm write
    if !force {
        let action = if append {
            "append content to"
        } else {
            "overwrite the content of"
        };

        println!(
            "{}",
            format!(
                "Are you sure you want to {} the file {}? (y/n)",
                action, pathname
            )
            .yellow()
        );

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        if input.trim().to_lowercase() != "y" {
            println!("{}", "Write canceled.".red());
            return;
        }
    }

    // Write to the file (overwrite)
    if !append {
        match std::fs::write(pathname, content) {
            Ok(_) => println!("{}", "File written successfully.".green()),
            Err(e) => println!("{}", format!("Error: {}", e).red()),
        }
        return;
    }

    // Write to the file (append)
    let mut file = match OpenOptions::new().append(true).open(pathname) {
        Ok(file) => file,
        Err(e) => {
            println!("{}", format!("Error opening file: {}", e).red());
            return;
        }
    };

    match file.write_all(content.as_bytes()) {
        Ok(_) => println!("{}", "File written successfully.".green()),
        Err(e) => println!("{}", format!("Error: {}", e).red()),
    }
}
