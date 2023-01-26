use super::core::Token;
use colored::Colorize;
use hashbrown::HashMap;

pub struct State {
    pub debug: bool,
    pub execution_stack: Vec<Token>,
    pub auxiliary: Vec<Token>,
    pub call_stack: Vec<HashMap<String, Token>>,
    pub error_log: Vec<String>,
    pub current_function_index: Vec<usize>,
    pub traceback: Vec<String>,
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
        // Function call traceback/ show each function line

        // line and position of error

        // type of error: output

        for function_call in &self.traceback {
            println!("Prev Call: {}", &function_call.bright_yellow());
        }

        println!("{}: {}", "Error".red(), &err.bright_yellow());
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
    
        if let Token::Identifier(ident) = tok {
            for scopes in self.call_stack.iter().rev() {
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

    pub fn get_from_heap(&mut self, ident: &str) -> Option<Token> {
        for scopes in self.call_stack.iter().rev() {
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
    })
}
