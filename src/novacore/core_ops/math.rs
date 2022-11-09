use crate::novacore::{core::Token, evaluator::Evaluator, state};

pub fn sqrt(eval: &mut Evaluator) {
    match eval.state.get_from_heap_or_pop() {
        Some(left) => {
            match &left {
                Token::Integer(left) => {
                    eval.state
                        .execution_stack
                        .push(Token::Float((*left as f64).sqrt()));
                }
                Token::Float(left) => {
                    eval.state.execution_stack.push(Token::Float(left.sqrt()));
                }
                _ => {
                    // Log error
                    if eval.state.debug {
                        eval.state
                            .error_log
                            .push(format!("can not sqrt this type {:?} ", left));
                    }
                }
            }
        }
        None => {
            // Log error
            if eval.state.debug {
                eval.state
                    .error_log
                    .push("Not enough arguments for ! not".to_string());
            }
        }
    }
}
