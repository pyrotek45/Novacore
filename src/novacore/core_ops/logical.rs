use crate::novacore::{core::Token, state};

pub fn logical_and(mut state: Box<state::State>) -> Box<state::State> {
    if let (Some(right), Some(left)) = (state.get_from_heap_or_pop(), state.get_from_heap_or_pop())
    {
        match (&right, &left) {
            (Token::Bool(right), Token::Bool(left)) => {
                state.execution_stack.push(Token::Bool(*left && *right));
            }
            _ => {
                // Log error
                if state.debug {
                    state.error_log.push(format!(
                        "can not and these two types {:?} :: {:?}",
                        left, right
                    ));
                }
            }
        }
    } else {
        // Log error
        if state.debug {
            state
                .error_log
                .push("Not enough arguments for and".to_string());
        }
    }

    state
}

pub fn logical_or(mut state: Box<state::State>) -> Box<state::State> {
    if let (Some(right), Some(left)) = (state.get_from_heap_or_pop(), state.get_from_heap_or_pop())
    {
        match (&right, &left) {
            (Token::Bool(right), Token::Bool(left)) => {
                state.execution_stack.push(Token::Bool(*left || *right));
            }
            _ => {
                // Log error
                if state.debug {
                    state.error_log.push(format!(
                        "can not or these two types {:?} :: {:?}",
                        left, right
                    ));
                }
            }
        }
    } else {
        // Log error
        if state.debug {
            state
                .error_log
                .push("Not enough arguments for or".to_string());
        }
    }

    state
}

pub fn logical_not(mut state: Box<state::State>) -> Box<state::State> {
    if let Some(token) = state.get_from_heap_or_pop() {
        if let Token::Bool(bool) = token {
            state.execution_stack.push(Token::Bool(!bool));
        } else {
            // Log error
            if state.debug {
                state
                    .error_log
                    .push(format!("can not ! NOT this type {:?} ", token));
            }
        }
    } else {
        // Log error
        if state.debug {
            state
                .error_log
                .push("Not enough arguments for ! not".to_string());
        }
    }

    state
}
