mod novacore;
use std::time::Instant;

use clap::{App, Arg};
use colored::Colorize;
use rustyline::{error::ReadlineError, validate::MatchingBracketValidator, Editor};

use rustyline::{Cmd, EventHandler, KeyCode, KeyEvent, Modifiers};
use rustyline_derive::{Completer, Helper, Highlighter, Hinter, Validator};

#[derive(Completer, Helper, Highlighter, Hinter, Validator)]
struct InputValidator {
    #[rustyline(Validator)]
    brackets: MatchingBracketValidator,
}

fn main() {
    // Clap setup
    let matches = App::new("Novacore Parser")
        .version("0.1")
        .author("Pyrotek45 pyrotek45_gaming@yahoo.com")
        .about("Novacore VM")
        .arg(
            Arg::with_name("FILE")
                .value_name("FILE")
                .help("Sets the input file to be used")
                .index(1),
        )
        .arg(
            Arg::with_name("DEBUG")
                .value_name("DEBUG")
                .long("debug")
                .takes_value(false)
                .short('d')
                .help("displays debug information"),
        )
        .arg(
            Arg::with_name("TIME")
                .value_name("TIME")
                .long("time")
                .takes_value(false)
                .short('t')
                .help("displays how long novacore takes to run"),
        )
        .get_matches();

    // Repl or File
    if let Some(filename) = matches.value_of("FILE") {
        let start = Instant::now();
        let mut core = novacore::new_from_file(filename);

        if matches.is_present("DEBUG") {
            core.debug_file(filename);
        } else {
            core.run();
        }

        if matches.is_present("TIME") {
            let duration = start.elapsed();
            println!("{} {:?}", ">> Execution:".bright_green(), duration);
        }

        std::process::exit(0)
    } else {
        // Using Repl
        let h = InputValidator {
            brackets: MatchingBracketValidator::new(),
        };
        let mut rl = Editor::new().unwrap();
        rl.set_helper(Some(h));
        rl.bind_sequence(
            KeyEvent(KeyCode::Char('s'), Modifiers::CTRL),
            EventHandler::Simple(Cmd::Newline),
        );
        if rl.load_history("history.txt").is_err() {
            println!("No previous history.");
        }

        let mut repl = String::new();
        let mut repl_debug: bool = false;
        let mut core = novacore::new();

        loop {
            // Repl prompt
            let readline = rl.readline("Nova :: $ ");
            match readline {
                Ok(line) => {
                    // Rustlyline History support
                    rl.add_history_entry(line.as_str());
                    rl.save_history("history.txt").unwrap();

                    // Basic repl commands to check
                    if line.to_lowercase() == "exit" {
                        break;
                    };

                    if line.to_lowercase() == "clear" {
                        repl.clear();
                        continue;
                    };

                    if line.to_lowercase() == "debug" {
                        repl_debug = !repl_debug;
                        continue;
                    };

                    // Enable vm debug
                    if repl_debug {
                        core.debug_string(&line)
                    } else {
                        core.run_string(&line);
                        if let Some(last) = core.get_stack_output() {
                            println!(" {}", last)
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    break;
                }
                Err(ReadlineError::Eof) => {
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }
    }
}
