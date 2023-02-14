use crate::novacore::utilities::print_line;

use super::core::Token;
use colored::Colorize;
use hashbrown::HashMap;

pub fn read_lines<P>(
    filename: P,
) -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>>
where
    P: AsRef<std::path::Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(std::io::BufRead::lines(std::io::BufReader::new(file)))
}

pub struct State {
    pub debug: bool,
    pub execution_stack: Vec<Token>,
    pub auxiliary: Vec<Token>,
    pub call_stack: Vec<HashMap<String, Token>>,
    pub bindings: Vec<HashMap<String, Token>>,
    pub error_log: Vec<String>,
    pub current_function_index: Vec<usize>,
    pub traceback: Vec<(String, usize)>,
    pub current_file: String,
    pub function_list: HashMap<String, usize>,
    pub break_loop: Vec<bool>,
    pub continue_loop: Vec<bool>,
    pub exit: bool,
    pub repl_mode: bool,
}

impl State {
    pub fn add_varaible(&mut self, ident: &str, item: Token) {
        if ident != "_" {
            if let Some(scope) = self.call_stack.last_mut() {
                scope.insert(ident.to_string(), item);
            }
        }
    }

    pub fn show_error(&mut self, err: &str) {
        println!();
        if let Some(function_call) = self.traceback.first() {
            print_line(function_call.1, &self.current_file);
            println!("Last call: {}", &function_call.0.bright_yellow());
        }
        println!("{}: {}", "Error".red(), &err.bright_yellow());

        if self.repl_mode {
            //self.exit = true;
        } else {
            std::process::exit(1);
        }
    }

    pub fn remove_varaible(&mut self, ident: &str) {
        if let Some(scope) = self.call_stack.last_mut() {
            scope.remove(ident);
        }
    }
    pub fn move_varaible(&mut self, ident: &str, newident: &str) {
        if let Some(scope) = self.call_stack.last_mut() {
            if let Some(moved) = scope.remove(ident) {
                scope.insert(newident.to_string(), moved);
            }
        }
    }

    pub fn get_from_heap_or_pop(&mut self) -> Option<Token> {
        let tok = self.execution_stack.pop()?;

        if let Token::Id(ident) = tok {
            for scopes in self.call_stack.iter().rev() {
                if let Some(func) = self.function_list.get(&ident) {
                    return Some(Token::Function(*func, 0));
                }
                if let Some(token) = scopes.get(&ident) {
                    return Some(token.clone());
                }
            }
            self.show_error(&format!("Unknown identifier {}", ident));
            None
        } else {
            Some(tok)
        }
    }

    pub fn get_from_binding(&mut self) -> Option<Token> {
        let tok = self.execution_stack.pop()?;

        if let Token::Id(ident) = tok {
            if let Some(scopes) = self.bindings.last_mut() {
                if let Some(token) = scopes.get(&ident) {
                    return Some(token.clone());
                }
            }
            self.show_error(&format!("Unknown Binding {}", ident));
            None
        } else {
            Some(tok)
        }
    }

    pub fn get_from_heap(&mut self, ident: &str) -> Option<Token> {
        for scopes in self.call_stack.iter().rev() {
            if let Some(func) = self.function_list.get(ident) {
                return Some(Token::Function(*func, 0));
            }
            if let Some(token) = scopes.get(ident) {
                return Some(token.clone());
            }
        }
        self.show_error(&format!("Unknown identifier {}", ident));
        None
    }
}

pub fn new() -> Box<State> {
    Box::new(State {
        execution_stack: Vec::with_capacity(1024),
        call_stack: vec![HashMap::new()],
        auxiliary: vec![],
        debug: false,
        error_log: vec![],
        current_function_index: vec![],
        traceback: vec![],
        current_file: "".to_string(),
        function_list: HashMap::new(),
        break_loop: vec![],
        continue_loop: vec![],
        exit: false,
        bindings: vec![HashMap::new()],
        repl_mode: false,
    })
}
