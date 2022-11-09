use crate::novacore::{core::Token, evaluator::Evaluator};

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
            _ => {
                // Log error
                if eval.state.debug {
                    eval.state.error_log.push(format!(
                        "can not and these two types {:?} :: {:?}",
                        left, right
                    ));
                }
            }
        }
    } else {
        // Log error
        if eval.state.debug {
            eval.state
                .error_log
                .push("Not enough arguments for and".to_string());
        }
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
            _ => {
                // Log error
                if eval.state.debug {
                    eval.state.error_log.push(format!(
                        "can not or these two types {:?} :: {:?}",
                        left, right
                    ));
                }
            }
        }
    } else {
        // Log error
        if eval.state.debug {
            eval.state
                .error_log
                .push("Not enough arguments for or".to_string());
        }
    }
}

pub fn logical_not(eval: &mut Evaluator) {
    if let Some(token) = eval.state.get_from_heap_or_pop() {
        if let Token::Bool(bool) = token {
            eval.state.execution_stack.push(Token::Bool(!bool));
        } else {
            // Log error
            if eval.state.debug {
                eval.state
                    .error_log
                    .push(format!("can not ! NOT this type {:?} ", token));
            }
        }
    } else {
        // Log error
        if eval.state.debug {
            eval.state
                .error_log
                .push("Not enough arguments for ! not".to_string());
        }
    }
}
