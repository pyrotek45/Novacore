use crate::novacore::{core::Token, evaluator::Evaluator, utilities::print_error};

pub fn equality_comparison(eval: &mut Evaluator) {
    if let (Some(right), Some(left)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        eval.state.execution_stack.push(Token::Bool(left == right));
    } else {
        eval.state
            .error_log
            .push("Not enough arguments for equality_comparison".to_string());
    }
}

pub fn less_than_comparison(eval: &mut Evaluator) {
    if let (Some(right), Some(left)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        match (&left, &right) {
            (Token::Integer(left), Token::Integer(right)) => {
                eval.state.execution_stack.push(Token::Bool(left < right));
            }
            (Token::Integer(ref left), Token::Float(right)) => {
                let left = *left as f64;
                eval.state.execution_stack.push(Token::Bool(left < *right));
            }
            (Token::Float(left), Token::Float(right)) => {
                eval.state.execution_stack.push(Token::Bool(left < right));
            }
            (Token::Float(left), Token::Integer(ref right)) => {
                let right = *right as f64;
                eval.state.execution_stack.push(Token::Bool(left < &right));
            }
            _ => print_error(&format!(
                "Cannot check if {:?} is less than {:?}",
                left, right
            )),
        }
    } else {
        eval.state
            .error_log
            .push("Not enough arguments for less_than_comparison".to_string());
    }
}

pub fn greater_than_comparison(eval: &mut Evaluator) {
    if let (Some(right), Some(left)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        match (&left, &right) {
            (Token::Integer(left), Token::Integer(right)) => {
                eval.state.execution_stack.push(Token::Bool(left > right));
            }
            (Token::Integer(ref left), Token::Float(right)) => {
                let left = *left as f64;
                eval.state.execution_stack.push(Token::Bool(left > *right));
            }
            (Token::Float(left), Token::Float(right)) => {
                eval.state.execution_stack.push(Token::Bool(left > right));
            }
            (Token::Float(left), Token::Integer(ref right)) => {
                let right = *right as f64;
                eval.state.execution_stack.push(Token::Bool(left > &right));
            }
            _ => print_error(&format!(
                "Cannot check if {:?} is greater than {:?}",
                left, right
            )),
        }
    } else {
        eval.state
            .error_log
            .push("Not enough arguments for greater_than_comparison".to_string());
    }
}
