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
        if let Some(tok) = self.execution_stack.pop() {
            if let Token::Identifier(ident) = tok {
                if let Some(scope) = self.call_stack.last_mut() {
                    if let Some(token) = scope.get(&ident) {
                        Some(token.clone())
                    } else {
                        println!("unknown identifier {}", ident);
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
        if let Some(scope) = self.call_stack.last() {
            if let Some(token) = scope.get(ident) {
                Some(token.clone())
            } else {
                println!("unknown identifier {}", ident);
                None
            }
        } else {
            None
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
