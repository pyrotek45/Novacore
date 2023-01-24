use crate::novacore::{core::Token, evaluator::Evaluator, utilities::print_error};

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
            _ => print_error(&format!("Incorrect argument for sqrt, got [{:?}]", left)),
        },
        None => {
            print_error("Not enough arguments for sqrt");
        }
    }
}
