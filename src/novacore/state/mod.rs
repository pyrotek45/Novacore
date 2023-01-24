use crate::novacore::utilities::print_error;

use super::core::Token;
use hashbrown::HashMap;

pub struct State {
    pub debug: bool,
    pub break_loop: Vec<bool>,
    pub execution_stack: Vec<Token>,
    pub temp: Vec<Token>,
    pub call_stack: Vec<HashMap<String, Token>>,
    pub error_log: Vec<String>,
    pub continue_loop: Vec<bool>,
    pub current_function_index: Vec<usize>,
}

impl State {
    pub fn get_from_heap_or_pop(&mut self) -> Option<Token> {
        match self.execution_stack.pop() {
            Some(Token::Identifier(ident)) => match self.call_stack.last_mut() {
                Some(scope) => match scope.get(&ident) {
                    Some(token) => Some(token.clone()),
                    None => {
                        print_error(&format!("Unknown identifier {}", ident));
                        None
                    }
                },
                None => None,
            },
            Some(tok) => Some(tok),
            None => None,
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
        temp: vec![],
        debug: false,
        error_log: vec![],
        break_loop: vec![],
        continue_loop: vec![],
        current_function_index: vec![],
    })
}
