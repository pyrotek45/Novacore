use crate::novacore::{core::Token, evaluator::Evaluator, utilities::print_error};

pub fn logical_and(eval: &mut Evaluator) {
    if let (Some(right), Some(left)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        match (&right, &left) {
            (Token::Bool(right), Token::Bool(left)) => {
                eval.state
                    .execution_stack
                    .push(Token::Bool(*left && *right));
            }
            _ => print_error(&format!(
                "Cannot compare these {:?} {:?} using logical AND",
                left, right
            )),
        }
    } else {
        eval.state
            .error_log
            .push("Not enough arguments for logical_and".to_string());
    }
}

pub fn logical_or(eval: &mut Evaluator) {
    if let (Some(right), Some(left)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        match (&right, &left) {
            (Token::Bool(right), Token::Bool(left)) => {
                eval.state
                    .execution_stack
                    .push(Token::Bool(*left || *right));
            }
            _ => print_error(&format!(
                "Cannot compare these {:?} {:?} using logical OR",
                left, right
            )),
        }
    } else {
        eval.state
            .error_log
            .push("Not enough arguments for logical_or".to_string());
    }
}

pub fn logical_not(eval: &mut Evaluator) {
    if let Some(token) = eval.state.get_from_heap_or_pop() {
        if let Token::Bool(bool) = token {
            eval.state.execution_stack.push(Token::Bool(!bool));
        } else {
            print_error(&format!("Cannot apply logical NOT to {:?}", token))
        }
    } else {
        eval.state
            .error_log
            .push("Not enough arguments for logical_not".to_string());
    }
}
