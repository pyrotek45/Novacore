use crate::novacore::{core::Token, state};

pub fn equals(mut state: Box<state::State>) -> Box<state::State> {
    if let (Some(right), Some(left)) = (state.get_from_heap_or_pop(), state.get_from_heap_or_pop())
    {
        state.execution_stack.push(Token::Bool(left == right));
    } else {
        // Log error
        if state.debug {
            state
                .error_log
                .push("Not enough arguments for ==".to_string());
        }
    }

    state
}

pub fn lss_comparison(mut state: Box<state::State>) -> Box<state::State> {
    if let (Some(right), Some(left)) = (state.get_from_heap_or_pop(), state.get_from_heap_or_pop())
    {
        match (&left, &right) {
            (Token::Integer(left), Token::Integer(right)) => {
                state.execution_stack.push(Token::Bool(left < right));
            }
            (Token::Integer(ref left), Token::Float(right)) => {
                let left = *left as f64;
                state.execution_stack.push(Token::Bool(left < *right));
            }
            (Token::Float(left), Token::Float(right)) => {
                state.execution_stack.push(Token::Bool(left < right));
            }
            (Token::Float(left), Token::Integer(ref right)) => {
                let right = *right as f64;
                state.execution_stack.push(Token::Bool(left < &right));
            }
            _ => {
                println!("cant lss these two types");
            }
        }
    } else {
        // Log error
        if state.debug {
            state
                .error_log
                .push("Not enough arguments for <".to_string());
        }
    }

    state
}

pub fn gtr_comparison(mut state: Box<state::State>) -> Box<state::State> {
    if let (Some(right), Some(left)) = (state.get_from_heap_or_pop(), state.get_from_heap_or_pop())
    {
        match (&left, &right) {
            (Token::Integer(left), Token::Integer(right)) => {
                state.execution_stack.push(Token::Bool(left > right));
            }
            (Token::Integer(ref left), Token::Float(right)) => {
                let left = *left as f64;
                state.execution_stack.push(Token::Bool(left > *right));
            }
            (Token::Float(left), Token::Float(right)) => {
                state.execution_stack.push(Token::Bool(left > right));
            }
            (Token::Float(left), Token::Integer(ref right)) => {
                let right = *right as f64;
                state.execution_stack.push(Token::Bool(left > &right));
            }
            _ => {
                println!("cant lss these two types");
            }
        }
    } else {
        // Log error
        if state.debug {
            state
                .error_log
                .push("Not enough arguments for >".to_string());
        }
    }

    state
}
