use crate::novacore::{core::Token, evaluator::Evaluator, state};

pub fn equals(eval: &mut Evaluator) {
    if let (Some(right), Some(left)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        eval.state.execution_stack.push(Token::Bool(left == right));
    } else {
        // Log error
        if eval.state.debug {
            eval.state
                .error_log
                .push("Not enough arguments for ==".to_string());
        }
    }
}

pub fn lss_comparison(eval: &mut Evaluator) {
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
            _ => {
                println!("cant lss these two types");
            }
        }
    } else {
        // Log error
        if eval.state.debug {
            eval.state
                .error_log
                .push("Not enough arguments for <".to_string());
        }
    }
}

pub fn gtr_comparison(eval: &mut Evaluator) {
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
            _ => {
                println!("cant lss these two types");
            }
        }
    } else {
        // Log error
        if eval.state.debug {
            eval.state
                .error_log
                .push("Not enough arguments for >".to_string());
        }
    }
}
