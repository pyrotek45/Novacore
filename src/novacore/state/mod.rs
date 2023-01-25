use crate::novacore::utilities::print_error;

use super::core::Token;
use hashbrown::HashMap;

pub struct State {
    pub debug: bool,
    pub execution_stack: Vec<Token>,
    pub auxiliary: Vec<Token>,
    pub call_stack: Vec<HashMap<String, Token>>,
    pub error_log: Vec<String>,
    pub current_function_index: Vec<usize>,
}

impl State {
    pub fn add_varaible(&mut self, ident: &str, item: Token) {
        if ident != "_" {
            if let Some(scope) = self.call_stack.last_mut() {
                scope.insert(ident.to_string(), item);
            }
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
        if let Some(tok) = self.execution_stack.pop() {
            if let Token::Identifier(ident) = tok {
                if let Some(scope) = self.call_stack.last_mut() {
                    if let Some(token) = scope.get(&ident) {
                        Some(token.clone())
                    } else {
                        print_error(&format!("Unknown identifier {}", ident));
                        None
                    }
                } else {
                    None
                }
            } else {
                Some(tok)
            }
        } else {
            None
        }
    }

    pub fn get_from_heap(&self, ident: &str) -> Option<Token> {
        match self.call_stack.last() {
            Some(scope) => match scope.get(ident) {
                Some(token) => Some(token.clone()),
                None => {
                    print_error(&format!("unknown identifier {}", ident));
                    None
                }
            },
            None => None,
        }
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
    })
}
