use crate::novacore::{core::Token, evaluator::Evaluator, state};

pub fn sqrt(mut state: Box<state::State>, eval: &mut Evaluator) -> Box<state::State> {
    match state.get_from_heap_or_pop() {
        Some(left) => {
            match &left {
                Token::Integer(left) => {
                    state
                        .execution_stack
                        .push(Token::Float((*left as f64).sqrt()));
                }
                Token::Float(left) => {
                    state.execution_stack.push(Token::Float(left.sqrt()));
                }
                _ => {
                    // Log error
                    if state.debug {
                        state
                            .error_log
                            .push(format!("can not sqrt this type {:?} ", left));
                    }
                }
            }
        }
        None => {
            // Log error
            if state.debug {
                state
                    .error_log
                    .push("Not enough arguments for ! not".to_string());
            }
        }
    }

    state
}
