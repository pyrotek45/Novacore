use super::core::Token;
use hashbrown::HashMap;

pub struct State {
    pub debug: bool,
    pub exit_loop: bool,
    pub execution_stack: Vec<Token>,
    pub temp: Option<Token>,
    pub call_stack: Vec<HashMap<String, Token>>,
    pub error_log: Vec<String>,
    pub continue_loop: bool,
    pub current_function_index: usize,
    pub current_object_name: String,
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
        temp: None,
        debug: false,
        error_log: vec![],
        exit_loop: false,
        continue_loop: false,
        current_function_index: 0,
        current_object_name: "".to_string(),
    })
}
