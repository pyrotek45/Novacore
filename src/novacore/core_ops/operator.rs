use std::rc::Rc;

use modulo::Mod;

use crate::novacore::{
    core::{Block, Operator, Token, Types, LT},
    state,
};

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
            (Token::List(LT::Raw(left)), Token::List(LT::Raw(right))) => {
                let mut newlist = vec![];
                newlist.clone_from(&*left);
                let mut secondlist = vec![];
                secondlist.clone_from(&*right);

                newlist.append(&mut secondlist);
                state
                    .execution_stack
                    .push(Token::List(LT::Raw(Rc::new(newlist))));
            }
            _ => {
                if state.debug {
                    // Log error
                    if left == Token::Break || right == Token::Break {
                        state
                            .error_log
                            .push(("Not enough arguments for +".to_string(), state.line_number));
                    } else {
                        state.error_log.push((
                            format!(
                                "+ cannot use types [{} :: {}]: Expected type [Integer]",
                                left.to_str(),
                                right.to_str()
                            ),
                            state.line_number,
                        ));
                    }
                }
            }
        }
    } else if state.debug {
        state
            .error_log
            .push(("Not enough arguments for +".to_string(), state.line_number));
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
                // if state.debug {
                //     state.error_log.push(format!(
                //         "can not div these two types {:?} :: {:?}",
                //         left, right
                //     ));
                // }
            }
        }
    } else {
        // Log error
        // if state.debug {
        //     state
        //         .error_log
        //         .push("Not enough arguments for /".to_string());
        // }
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
                // if state.debug {
                //     state
                //         .error_log
                //         .push(format!("can not make this a negitive{:?}", left));
                // }
            }
        }
    } else {
        // Log error
        // if state.debug {
        //     state
        //         .error_log
        //         .push("Not enough arguments for - unary minus".to_string());
        // }
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
                // if state.debug {
                //     state.error_log.push(format!(
                //         "can not sub these two types {:?} :: {:?}",
                //         left, right
                //     ));
                // }
            }
        }
    } else {
        // Log error
        // if state.debug {
        //     state
        //         .error_log
        //         .push("Not enough arguments for -".to_string());
        // }
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
                // if state.debug {
                //     state.error_log.push(format!(
                //         "can not sub these two types {:?} :: {:?}",
                //         left, right
                //     ));
                // }
            }
        }
    } else {
        // Log error
        // if state.debug {
        //     state
        //         .error_log
        //         .push("Not enough arguments for % modulo".to_string());
        // }
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
                // if state.debug {
                //     state.error_log.push(format!(
                //         "can not mul these two types {:?} :: {:?}",
                //         left, right
                //     ));
                // }
            }
        }
    } else {
        // Log error
        // if state.debug {
        //     state
        //         .error_log
        //         .push("Not enough arguments for *".to_string());
        // }
    }

    state
}

pub fn variable_assign(mut state: Box<state::State>) -> Box<state::State> {
    if let (Some(token), Some(ident)) = (state.execution_stack.pop(), state.execution_stack.pop()) {
        match (&token, &ident) {
            (Token::Identifier(moved, _), Token::Identifier(identifier, _)) => {
                if let Some(scope) = state.call_stack.last_mut() {
                    if identifier != "_" {
                        if let Some(item) = scope.remove(moved) {
                            scope.insert(identifier.to_string(), item);
                        }
                    }
                }
            }
            (value, Token::Identifier(identifier, oftype)) => {
                if match value {
                    Token::Integer(_) => matches!(*oftype, Types::Any | Types::Int),
                    Token::String(_) => matches!(*oftype, Types::Any | Types::Str),
                    Token::Float(_) => matches!(*oftype, Types::Any | Types::Float),
                    Token::Block(_) => matches!(*oftype, Types::Any | Types::Block),
                    Token::Char(_) => matches!(*oftype, Types::Any | Types::Char),
                    _ => matches!(*oftype, Types::Any),
                } {
                    if let Some(scope) = state.call_stack.last_mut() {
                        if identifier != "_" {
                            scope.insert(identifier.to_string(), token);
                        }
                    }
                } else if state.debug {
                    state.error_log.push((
                        format!(
                            "{:?} expected for identifier {}, got token [{}]",
                            oftype,
                            identifier,
                            value.to_str()
                        ),
                        state.line_number,
                    ))
                }
            }
            _ => {
                // Log error
                // if state.debug {
                //     state.error_log.push(format!(
                //         "can not assign these two types {:?} :: {:?}",
                //         token, ident
                //     ));
                // }
            }
        }
    } else {
        // Log error
        // if state.debug {
        //     state
        //         .error_log
        //         .push("Not enough arguments for =".to_string());
        // }
    }

    state
}

pub fn function_variable_assign(mut state: Box<state::State>) -> Box<state::State> {
    let mut variable_stack: Vec<(String, Types)> = Vec::with_capacity(10);
    if let Some(Token::List(LT::Raw(identifiers))) = state.get_from_heap_or_pop() {
        for toks in identifiers.iter().rev() {
            if let Token::Identifier(ident, oftype) = &toks {
                variable_stack.push((ident.clone(), oftype.clone()))
            }
        }
    }

    // Tie each Token into the call_stack using the tokens poped

    if let Some(mut newscope) = state.call_stack.pop() {
        for tokens in variable_stack {
            if let Some(tok) = state.get_from_heap_or_pop() {
                if tok != Token::Break {
                    if match tok {
                        Token::Integer(_) => matches!(tokens.1, Types::Any | Types::Int),
                        Token::String(_) => matches!(tokens.1, Types::Any | Types::Str),
                        Token::Float(_) => matches!(tokens.1, Types::Any | Types::Float),
                        Token::Block(_) => matches!(tokens.1, Types::Any | Types::Block),
                        Token::Char(_) => matches!(tokens.1, Types::Any | Types::Char),
                        _ => matches!(tokens.1, Types::Any),
                    } {
                        newscope.insert(tokens.0, tok.clone());
                    } else if state.debug {
                        state.error_log.push((
                            format!(
                                "{:?} expected for identifier {}, got token [{}]",
                                tokens.1,
                                tokens.0,
                                tok.to_str(),
                            ),
                            state.line_number,
                        ))
                    }
                } else if state.debug {
                    state.error_log.push((
                        format!("Not enough arguments for {}", state.current_function),
                        state.line_number,
                    ));
                }
            }
        }
        state.call_stack.push(newscope);
    } else {
        // Log error
        // if state.debug {
        //     state
        //         .error_log
        //         .push("Not enough arguments for ~ , Callstack error".to_string());
        // }
    }

    state
}

pub fn get_self(mut state: Box<state::State>) -> Box<state::State> {
    if let Some(scope) = state.call_stack.last_mut() {
        let mut core_self = vec![];

        for (ident, token) in scope {
            core_self.push(Token::Identifier(ident.clone(), Types::Any));
            core_self.push(token.clone());
            core_self.push(Token::Op(Operator::VariableAssign))
        }

        state
            .execution_stack
            .push(Token::Block(Block::Literal(Rc::new(core_self))))
    }
    state
}

pub fn return_top(mut state: Box<state::State>) -> Box<state::State> {
    if let Some(top) = state.get_from_heap_or_pop() {
        state.execution_stack.push(top)
    }
    state
}
