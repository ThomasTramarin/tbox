use colored::Colorize;

pub fn unknown_command() {
    println!(
        "{}\n{}",
        "Oops, this command doesn't exist.".red(),
        "Try using the --help flag.".blue()
    )
}
