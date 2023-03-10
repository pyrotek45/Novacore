mod novacore;
use std::{io::stdout, time::Instant, rc::Rc};

use clap::{App, Arg};
use colored::Colorize;
use crossterm::{
    cursor::MoveTo,
    execute,
    style::Stylize,
    terminal::{self, ClearType},
};
use rustyline::{error::ReadlineError, validate::MatchingBracketValidator, Editor};

use novacore::lexer;
use rustyline::{Cmd, EventHandler, KeyCode, KeyEvent, Modifiers};
use rustyline_derive::{Completer, Helper, Highlighter, Hinter, Validator};

use crate::novacore::core::{Token, Instructions};

#[derive(Completer, Helper, Highlighter, Hinter, Validator)]
struct InputValidator {
    #[rustyline(Validator)]
    brackets: MatchingBracketValidator,
}

fn main() {
    // println!("{}", std::mem::size_of::<Token>());
    // std::process::exit(0);
    // Clap setup
    let matches = App::new("Novacore")
        .version("0.1")
        .author("Pyrotek45 pyrotek45_gaming@yahoo.com")
        .about("Novacore VM")
        .arg(
            Arg::with_name("FILE")
                .value_name("FILE")
                .multiple_values(true)
                .help("Sets the input file to be used")
                .index(1),
        )
        .arg(
            Arg::with_name("DEBUGOUTPUT")
                .value_name("DEBUGOUTPUT")
                .long("debugoutput")
                .takes_value(false)
                .short('o')
                .help("disassembles file"),
        )
        .arg(
            Arg::with_name("TIME")
                .value_name("TIME")
                .long("time")
                .takes_value(false)
                .short('t')
                .help("displays how long novacore takes to run"),
        )
        .arg(
            Arg::with_name("DEBUG")
                .value_name("DEBUG")
                .long("debug")
                .takes_value(false)
                .short('d')
                .help("runs file with debug mode"),
        )
        .get_matches();

    // Repl or File
    if let Some(filename) = matches.value_of("FILE") {
        let start = Instant::now();
        let mut core = novacore::new_from_file(filename);

        if matches.is_present("DEBUGOUTPUT") {
            core.debug_file(filename);
        } else if matches.is_present("DEBUG") {
            println!("RUNNING DEBUG...");
            let mut args: Vec<String> = std::env::args().collect();
            args.remove(0);
            args.remove(0);
            args.remove(0);
            let args = args.join(" ");
            let mut lex = lexer::new();
            lex.insert_string(&args);
            core.evaluator.debug = true;
            let parsed = match lex.parse() {
                Ok(parsed) => parsed,
                Err(_) => todo!(),
            };
            core.evaluator.state.execution_stack = parsed;
            core.run();
        } else {
            let mut args: Vec<String> = std::env::args().collect();
            args.remove(0);
            args.remove(0);
            let args = args.join(" ");
            let mut lex = lexer::new();
            lex.insert_string(&args);
            let parsed = match lex.parse() {
                Ok(parsed) => parsed,
                Err(_) => todo!(),
            };
            core.evaluator.state.execution_stack = parsed;
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
            KeyEvent(KeyCode::Char('n'), Modifiers::CTRL),
            EventHandler::Simple(Cmd::Newline),
        );
        if rl.load_history("history.txt").is_err() {
            println!("No previous history.");
        }

        let _repl = String::new();
        let mut repl_debug: bool = false;
        let mut core = novacore::new();
        core.evaluator.state.repl_mode = true;
        // core.lexer = Lexer::new();
        // core.init();

        loop {
            // Repl prompt
            let readline = rl.readline("Nova $ ");
            match readline {
                Ok(line) => {
                    // Rustlyline History support
                    rl.add_history_entry(line.as_str()).unwrap();
                    rl.save_history("history.txt").unwrap();

                    // Basic repl commands to check
                    if line.to_lowercase() == "exit" {
                        break;
                    };

                    if line.to_lowercase() == "dis" {
                        repl_debug = !repl_debug;
                        continue;
                    };

                    if line.to_lowercase() == "debug" {
                        core.evaluator.debug = true;
                        continue;
                    };

                    if line.to_lowercase() == "reset" {
                        core = novacore::new();
                        core.evaluator.state.repl_mode = true;
                        continue;
                    };

                    if line.to_lowercase() == "clear" {
                        execute!(stdout(), terminal::Clear(ClearType::All)).unwrap();
                        execute!(stdout(), MoveTo(0, 0)).unwrap();
                        continue;
                    }
                    // Enable vm debug
                    if repl_debug {
                        core.debug_string(&line)
                    } else {
                        core.run_string(&line);
                        if let Some(last) = core.get_stack_output() {
                            println!(" ---> {}", last.white())
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
