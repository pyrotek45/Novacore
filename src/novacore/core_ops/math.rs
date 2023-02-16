use crate::novacore::{core::Token, evaluator::Evaluator};

pub fn sqrt(eval: &mut Evaluator) {
    match eval.state.get_from_heap_or_pop() {
        Some(left) => match &left {
            Token::Integer(left) => {
                eval.state
                    .execution_stack
                    .push(Token::Float((*left as f64).sqrt()));
            }
            Token::Float(left) => {
                eval.state.execution_stack.push(Token::Float(left.sqrt()));
            }
            _ => eval
                .state
                .show_error(&format!("Incorrect argument for sqrt, got [{:?}]", left)),
        },
        None => {
            eval.state.show_error("Not enough arguments for sqrt");
        }
    }
}
