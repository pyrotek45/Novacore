use crate::novacore::{core::Token, evaluator::Evaluator};

pub fn as_int(eval: &mut Evaluator) {
    match eval.state.get_from_heap_or_pop() {
        Some(Token::Integer(value)) => {
            eval.state.execution_stack.push(Token::Integer(value));
        }
        Some(Token::Float(value)) => {
            eval.state
                .execution_stack
                .push(Token::Integer(value as i128));
        }

        Some(Token::String(value)) => {
            if let Ok(value) = value.parse::<i128>() {
                eval.state.execution_stack.push(Token::Integer(value));
            } else {
                eval.state
                    .show_error(&format!("Could not parse, but got [{:?}]", value))
            }
        }
        a => eval
            .state
            .show_error(&format!("Incorrect argument for int , got [{:?}]", a)),
    }
}

pub fn is_int(eval: &mut Evaluator) {
    match eval.state.get_from_heap_or_pop() {
        Some(Token::Integer(_)) => {
            eval.state.execution_stack.push(Token::Bool(true));
        }

        Some(Token::String(value)) => {
            if value.parse::<i128>().is_ok() {
                eval.state.execution_stack.push(Token::Bool(true));
            } else {
                eval.state.execution_stack.push(Token::Bool(false));
            }
        }
        _ => eval.state.execution_stack.push(Token::Bool(false)),
    }
}

pub fn as_char(eval: &mut Evaluator) {
    match eval.state.get_from_heap_or_pop() {
        Some(Token::String(value)) => {
            if let Some(value) = value.chars().next() {
                eval.state.execution_stack.push(Token::Char(value));
            }
        }
        a => eval
            .state
            .show_error(&format!("Incorrect argument for char , got [{:?}]", a)),
    }
}

pub fn is_char(eval: &mut Evaluator) {
    match eval.state.get_from_heap_or_pop() {
        Some(Token::String(value)) => {
            if value.parse::<char>().is_ok() {
                eval.state.execution_stack.push(Token::Bool(true));
            } else {
                eval.state.execution_stack.push(Token::Bool(false));
            }
        }
        a => eval
            .state
            .show_error(&format!("Incorrect argument for ischar , got [{:?}]", a)),
    }
}

pub fn as_string(eval: &mut Evaluator) {
    match eval.state.get_from_heap_or_pop() {
        Some(Token::Integer(value)) => {
            eval.state
                .execution_stack
                .push(Token::String(value.to_string()));
        }
        Some(Token::Float(value)) => {
            eval.state
                .execution_stack
                .push(Token::String(value.to_string()));
        }
        Some(Token::String(value)) => {
            eval.state.execution_stack.push(Token::String(value));
        }
        Some(Token::Char(value)) => {
            eval.state
                .execution_stack
                .push(Token::String(value.to_string()));
        }
        a => eval
            .state
            .show_error(&format!("Incorrect argument for str , got [{:?}]", a)),
    }
}
