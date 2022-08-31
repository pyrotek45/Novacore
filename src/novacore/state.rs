use super::core::Token;
use hashbrown::HashMap;

pub struct State {
    pub debug: bool,
    pub exit_loop: bool,
    pub execution_stack: Vec<Token>,
    pub temp: Option<Token>,
    pub call_stack: Vec<HashMap<String, Token>>,
    pub error_log: Vec<(String, usize)>,
    pub continue_loop: bool,
    pub line_number: usize,
    pub current_function: String,
}

impl State {
    pub fn get_from_heap_or_pop(&mut self) -> Option<Token> {
        if let Some(tok) = self.execution_stack.pop() {
            if let Token::Identifier(ident, _) = tok {
                if let Some(scope) = self.call_stack.last_mut() {
                    if let Some(token) = scope.get(&ident) {
                        Some(token.clone())
                    } else {
                        if self.debug {
                            self.error_log.push((
                                format!("Unknown identifier [{}] ", ident),
                                self.line_number,
                            ))
                        }
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

    pub fn get_from_heap(&mut self, ident: &str) -> Option<Token> {
        if let Some(scope) = self.call_stack.last_mut() {
            if let Some(token) = scope.get(ident) {
                Some(token.clone())
            } else {
                if self.debug {
                    self.error_log
                        .push((format!("Unknown identifier [{}] ", ident), self.line_number))
                }
                None
            }
        } else {
            None
        }
    }
}

pub fn new(debug: bool) -> Box<State> {
    Box::new(State {
        execution_stack: Vec::with_capacity(1024),
        call_stack: vec![HashMap::new()],
        temp: None,
        debug,
        error_log: vec![],
        exit_loop: false,
        continue_loop: false,
        line_number: 0,
        current_function: "".to_string(),
    })
}
