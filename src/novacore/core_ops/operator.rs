use std::rc::Rc;

use modulo::Mod;

use crate::novacore::{core::Token, state};

pub fn add(mut state: Box<state::State>) -> Box<state::State> {
    if let (Some(right), Some(left)) = (state.get_from_heap_or_pop(), state.get_from_heap_or_pop())
    {
        match (&left, &right) {
            (Token::Integer(left), Token::Integer(right)) => {
                state.execution_stack.push(Token::Integer(left + right));
            }
            (Token::Integer(ref left), Token::Float(right)) => {
                let left = *left as f64;
                state.execution_stack.push(Token::Float(left + right));
            }
            (Token::Float(left), Token::Float(right)) => {
                state.execution_stack.push(Token::Float(left + right));
            }
            (Token::Float(left), Token::Integer(ref right)) => {
                let right = *right as f64;
                state.execution_stack.push(Token::Float(left + right));
            }
            (Token::String(left), Token::String(right)) => {
                state
                    .execution_stack
                    .push(Token::String(left.to_string() + right));
            }
            (Token::Char(left), Token::Char(right)) => {
                state
                    .execution_stack
                    .push(Token::String(left.to_string() + &right.to_string()));
            }
            (Token::Char(left), Token::String(right)) => {
                state
                    .execution_stack
                    .push(Token::String(left.to_string() + right));
            }
            (Token::String(left), Token::Char(right)) => {
                state
                    .execution_stack
                    .push(Token::String(left.to_string() + &right.to_string()));
            }
            (Token::String(left), Token::Integer(right)) => {
                state
                    .execution_stack
                    .push(Token::String(left.to_string() + &right.to_string()));
            }
            (Token::Integer(left), Token::String(right)) => {
                state
                    .execution_stack
                    .push(Token::String(left.to_string() + right));
            }
            (Token::Char(left), Token::Integer(right)) => {
                state
                    .execution_stack
                    .push(Token::String(left.to_string() + &right.to_string()));
            }
            (Token::Integer(left), Token::Char(right)) => {
                state
                    .execution_stack
                    .push(Token::String(left.to_string() + &right.to_string()));
            }
            (Token::List(left), Token::List(right)) => {
                let mut newlist = vec![];
                newlist.clone_from(&*left);
                let mut secondlist = vec![];
                secondlist.clone_from(&*right);

                newlist.append(&mut secondlist);
                state.execution_stack.push(Token::List(Rc::new(newlist)));
            }
            _ => {
                // Log error
                if state.debug {
                    state.error_log.push(format!(
                        "can not add these two types {:?} :: {:?}",
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
                .push("Not enough arguments for +".to_string());
        }
    }

    state
}

pub fn div(mut state: Box<state::State>) -> Box<state::State> {
    if let (Some(right), Some(left)) = (state.get_from_heap_or_pop(), state.get_from_heap_or_pop())
    {
        match (&left, &right) {
            (Token::Integer(left), Token::Integer(right)) => {
                state
                    .execution_stack
                    .push(Token::Float(*left as f64 / *right as f64));
            }
            (Token::Integer(ref left), Token::Float(right)) => {
                state
                    .execution_stack
                    .push(Token::Float(*left as f64 / *right as f64));
            }
            (Token::Float(left), Token::Float(right)) => {
                state
                    .execution_stack
                    .push(Token::Float(*left as f64 / *right as f64));
            }
            (Token::Float(left), Token::Integer(ref right)) => {
                state
                    .execution_stack
                    .push(Token::Float(*left as f64 / *right as f64));
            }
            _ => {
                // Log error
                if state.debug {
                    state.error_log.push(format!(
                        "can not div these two types {:?} :: {:?}",
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
                .push("Not enough arguments for /".to_string());
        }
    }

    state
}

pub fn neg(mut state: Box<state::State>) -> Box<state::State> {
    if let Some(left) = state.get_from_heap_or_pop() {
        match &left {
            Token::Integer(left) => {
                state.execution_stack.push(Token::Integer(-left));
            }
            Token::Float(left) => {
                state.execution_stack.push(Token::Float(-left));
            }
            _ => {
                // Log error
                if state.debug {
                    state
                        .error_log
                        .push(format!("can not make this a negitive{:?}", left));
                }
            }
        }
    } else {
        // Log error
        if state.debug {
            state
                .error_log
                .push("Not enough arguments for - unary minus".to_string());
        }
    }

    state
}

pub fn sub(mut state: Box<state::State>) -> Box<state::State> {
    if let (Some(right), Some(left)) = (state.get_from_heap_or_pop(), state.get_from_heap_or_pop())
    {
        match (&left, &right) {
            (Token::Integer(left), Token::Integer(right)) => {
                state.execution_stack.push(Token::Integer(left - right));
            }
            (Token::Integer(ref left), Token::Float(right)) => {
                let left = *left as f64;
                state.execution_stack.push(Token::Float(left - right));
            }
            (Token::Float(left), Token::Float(right)) => {
                state.execution_stack.push(Token::Float(left - right));
            }
            (Token::Float(left), Token::Integer(ref right)) => {
                let right = *right as f64;
                state.execution_stack.push(Token::Float(left - right));
            }
            _ => {
                // Log error
                if state.debug {
                    state.error_log.push(format!(
                        "can not sub these two types {:?} :: {:?}",
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
                .push("Not enough arguments for -".to_string());
        }
    }

    state
}

pub fn modulo(mut state: Box<state::State>) -> Box<state::State> {
    if let (Some(right), Some(left)) = (state.get_from_heap_or_pop(), state.get_from_heap_or_pop())
    {
        match (&left, &right) {
            (Token::Integer(left), Token::Integer(right)) => {
                state
                    .execution_stack
                    .push(Token::Integer(left.modulo(right)));
            }
            _ => {
                if state.debug {
                    state.error_log.push(format!(
                        "can not sub these two types {:?} :: {:?}",
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
                .push("Not enough arguments for % modulo".to_string());
        }
    }

    state
}

pub fn mul(mut state: Box<state::State>) -> Box<state::State> {
    if let (Some(right), Some(left)) = (state.get_from_heap_or_pop(), state.get_from_heap_or_pop())
    {
        match (&left, &right) {
            (Token::Integer(left), Token::Integer(right)) => {
                state.execution_stack.push(Token::Integer(left * right));
            }
            (Token::Integer(ref left), Token::Float(right)) => {
                let left = *left as f64;
                state.execution_stack.push(Token::Float(left * right));
            }
            (Token::Float(left), Token::Float(right)) => {
                state.execution_stack.push(Token::Float(left * right));
            }
            (Token::Float(left), Token::Integer(ref right)) => {
                let right = *right as f64;
                state.execution_stack.push(Token::Float(left * right));
            }
            _ => {
                // Log error
                if state.debug {
                    state.error_log.push(format!(
                        "can not mul these two types {:?} :: {:?}",
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
                .push("Not enough arguments for *".to_string());
        }
    }

    state
}

pub fn variable_assign(mut state: Box<state::State>) -> Box<state::State> {
    if let (Some(token), Some(ident)) = (state.execution_stack.pop(), state.execution_stack.pop()) {
        match (&token, &ident) {
            (Token::Identifier(moved), Token::Identifier(identifier)) => {
                if let Some(scope) = state.call_stack.last_mut() {
                    if identifier != "_" {
                        if let Some(item) = scope.remove(moved) {
                            scope.insert(identifier.to_string(), item);
                        }
                    }
                }
            }
            (_, Token::Identifier(identifier)) => {
                if let Some(scope) = state.call_stack.last_mut() {
                    if identifier != "_" {
                        scope.insert(identifier.to_string(), token);
                    }
                }
            }
            _ => {
                // Log error
                if state.debug {
                    state.error_log.push(format!(
                        "can not assign these two types {:?} :: {:?}",
                        token, ident
                    ));
                }
            }
        }
    } else {
        // Log error
        if state.debug {
            state
                .error_log
                .push("Not enough arguments for =".to_string());
        }
    }

    state
}

pub fn function_variable_assign(mut state: Box<state::State>) -> Box<state::State> {
    let mut variable_stack: Vec<String> = Vec::with_capacity(10);
    if let Some(Token::List(identifiers)) = state.get_from_heap_or_pop() {
        for toks in identifiers.iter().rev() {
            if let Token::Identifier(ident) = &toks {
                variable_stack.push(ident.clone())
            }
        }
    }

    // Tie each Token into the call_stack using the tokens poped

    if let Some(mut newscope) = state.call_stack.pop() {
        for tokens in variable_stack {
            if let Some(tok) = state.get_from_heap_or_pop() {
                newscope.insert(tokens, tok.clone());
            }
        }
        state.call_stack.push(newscope);
    } else {
        // Log error
        if state.debug {
            state
                .error_log
                .push("Not enough arguments for ~ , Callstack error".to_string());
        }
    }

    state
}
