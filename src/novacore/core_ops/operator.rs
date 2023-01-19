use std::rc::Rc;

use modulo::Mod;

use crate::novacore::{
    core::{Block, Operator, Token},
    evaluator::Evaluator,
};

pub fn add(eval: &mut Evaluator) {
    if let (Some(right), Some(left)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        match (&left, &right) {
            (Token::Integer(left), Token::Integer(right)) => {
                eval.state
                    .execution_stack
                    .push(Token::Integer(left + right));
            }
            (Token::Integer(ref left), Token::Float(right)) => {
                let left = *left as f64;
                eval.state.execution_stack.push(Token::Float(left + right));
            }
            (Token::Float(left), Token::Float(right)) => {
                eval.state.execution_stack.push(Token::Float(left + right));
            }
            (Token::Float(left), Token::Integer(ref right)) => {
                let right = *right as f64;
                eval.state.execution_stack.push(Token::Float(left + right));
            }
            (Token::String(left), Token::String(right)) => {
                eval.state
                    .execution_stack
                    .push(Token::String(left.to_string() + right));
            }
            (Token::Char(left), Token::Char(right)) => {
                eval.state
                    .execution_stack
                    .push(Token::String(left.to_string() + &right.to_string()));
            }
            (Token::Char(left), Token::String(right)) => {
                eval.state
                    .execution_stack
                    .push(Token::String(left.to_string() + right));
            }
            (Token::String(left), Token::Char(right)) => {
                eval.state
                    .execution_stack
                    .push(Token::String(left.to_string() + &right.to_string()));
            }
            (Token::String(left), Token::Float(right)) => {
                eval.state
                    .execution_stack
                    .push(Token::String(left.to_string() + &right.to_string()));
            }
            (Token::Float(left), Token::String(right)) => {
                eval.state
                    .execution_stack
                    .push(Token::String(left.to_string() + &right.to_string()));
            }
            (Token::String(left), Token::Bool(right)) => {
                eval.state
                    .execution_stack
                    .push(Token::String(left.to_string() + &right.to_string()));
            }
            (Token::Bool(left), Token::String(right)) => {
                eval.state
                    .execution_stack
                    .push(Token::String(left.to_string() + &right.to_string()));
            }
            (Token::String(left), Token::Integer(right)) => {
                eval.state
                    .execution_stack
                    .push(Token::String(left.to_string() + &right.to_string()));
            }
            (Token::Integer(left), Token::String(right)) => {
                eval.state
                    .execution_stack
                    .push(Token::String(left.to_string() + right));
            }
            (Token::Char(left), Token::Integer(right)) => {
                eval.state
                    .execution_stack
                    .push(Token::String(left.to_string() + &right.to_string()));
            }
            (Token::Integer(left), Token::Char(right)) => {
                eval.state
                    .execution_stack
                    .push(Token::String(left.to_string() + &right.to_string()));
            }

            _ => {
                // Log error
                if eval.state.debug {
                    eval.state.error_log.push(format!(
                        "can not add these two types {:?} :: {:?}",
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
                .push("Not enough arguments for +".to_string());
        }
    }
}

pub fn div(eval: &mut Evaluator) {
    if let (Some(right), Some(left)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        match (&left, &right) {
            (Token::Integer(left), Token::Integer(right)) => {
                eval.state
                    .execution_stack
                    .push(Token::Float(*left as f64 / *right as f64));
            }
            (Token::Integer(ref left), Token::Float(right)) => {
                eval.state
                    .execution_stack
                    .push(Token::Float(*left as f64 / *right as f64));
            }
            (Token::Float(left), Token::Float(right)) => {
                eval.state
                    .execution_stack
                    .push(Token::Float(*left as f64 / *right as f64));
            }
            (Token::Float(left), Token::Integer(ref right)) => {
                eval.state
                    .execution_stack
                    .push(Token::Float(*left as f64 / *right as f64));
            }
            _ => {
                // Log error
                if eval.state.debug {
                    eval.state.error_log.push(format!(
                        "can not div these two types {:?} :: {:?}",
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
                .push("Not enough arguments for /".to_string());
        }
    }
}

pub fn neg(eval: &mut Evaluator) {
    if let Some(left) = eval.state.get_from_heap_or_pop() {
        match &left {
            Token::Integer(left) => {
                eval.state.execution_stack.push(Token::Integer(-left));
            }
            Token::Float(left) => {
                eval.state.execution_stack.push(Token::Float(-left));
            }
            _ => {
                // Log error
                if eval.state.debug {
                    eval.state
                        .error_log
                        .push(format!("can not make this a negitive{:?}", left));
                }
            }
        }
    } else {
        // Log error
        if eval.state.debug {
            eval.state
                .error_log
                .push("Not enough arguments for - unary minus".to_string());
        }
    }
}

pub fn sub(eval: &mut Evaluator) {
    if let (Some(right), Some(left)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        match (&left, &right) {
            (Token::Integer(left), Token::Integer(right)) => {
                eval.state
                    .execution_stack
                    .push(Token::Integer(left - right));
            }
            (Token::Integer(ref left), Token::Float(right)) => {
                let left = *left as f64;
                eval.state.execution_stack.push(Token::Float(left - right));
            }
            (Token::Float(left), Token::Float(right)) => {
                eval.state.execution_stack.push(Token::Float(left - right));
            }
            (Token::Float(left), Token::Integer(ref right)) => {
                let right = *right as f64;
                eval.state.execution_stack.push(Token::Float(left - right));
            }
            _ => {
                // Log error
                if eval.state.debug {
                    eval.state.error_log.push(format!(
                        "can not sub these two types {:?} :: {:?}",
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
                .push("Not enough arguments for -".to_string());
        }
    }
}

pub fn modulo(eval: &mut Evaluator) {
    if let (Some(right), Some(left)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        match (&left, &right) {
            (Token::Integer(left), Token::Integer(right)) => {
                eval.state
                    .execution_stack
                    .push(Token::Integer(left.modulo(right)));
            }
            _ => {
                if eval.state.debug {
                    eval.state.error_log.push(format!(
                        "can not sub these two types {:?} :: {:?}",
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
                .push("Not enough arguments for % modulo".to_string());
        }
    }
}

pub fn mul(eval: &mut Evaluator) {
    if let (Some(right), Some(left)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        match (&left, &right) {
            (Token::Integer(left), Token::Integer(right)) => {
                eval.state
                    .execution_stack
                    .push(Token::Integer(left * right));
            }
            (Token::Integer(ref left), Token::Float(right)) => {
                let left = *left as f64;
                eval.state.execution_stack.push(Token::Float(left * right));
            }
            (Token::Float(left), Token::Float(right)) => {
                eval.state.execution_stack.push(Token::Float(left * right));
            }
            (Token::Float(left), Token::Integer(ref right)) => {
                let right = *right as f64;
                eval.state.execution_stack.push(Token::Float(left * right));
            }
            _ => {
                // Log error
                if eval.state.debug {
                    eval.state.error_log.push(format!(
                        "can not mul these two types {:?} :: {:?}",
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
                .push("Not enough arguments for *".to_string());
        }
    }
}

pub fn variable_assign(eval: &mut Evaluator) {
    if let (Some(token), Some(ident)) = (
        eval.state.execution_stack.pop(),
        eval.state.execution_stack.pop(),
    ) {
        match (&token, &ident) {
            (Token::Identifier(moved), Token::Identifier(identifier)) => {
                if let Some(scope) = eval.state.call_stack.last_mut() {
                    if identifier != "_" {
                        if let Some(item) = scope.remove(moved) {
                            scope.insert(identifier.to_string(), item);
                        }
                    }
                }
            }
            (_, Token::Identifier(identifier)) => {
                if let Some(scope) = eval.state.call_stack.last_mut() {
                    if identifier != "_" {
                        scope.insert(identifier.to_string(), token);
                    }
                }
            }
            _ => {
                // Log error
                if eval.state.debug {
                    eval.state.error_log.push(format!(
                        "can not assign these two types {:?} :: {:?}",
                        token, ident
                    ));
                }
            }
        }
    } else {
        // Log error
        if eval.state.debug {
            eval.state
                .error_log
                .push("Not enough arguments for =".to_string());
        }
    }
}

pub fn function_variable_assign(eval: &mut Evaluator) {
    let mut variable_stack: Vec<String> = Vec::with_capacity(10);
    if let Some(Token::Block(Block::Raw(identifiers))) = eval.state.get_from_heap_or_pop() {
        for toks in identifiers.iter().rev() {
            if let Token::Identifier(ident) = &toks {
                variable_stack.push(ident.clone())
            }
        }
    }

    // Tie each Token into the call_stack using the tokens poped

    if let Some(mut newscope) = eval.state.call_stack.pop() {
        for tokens in variable_stack {
            if let Some(tok) = eval.state.get_from_heap_or_pop() {
                newscope.insert(tokens, tok.clone());
            }
        }
        eval.state.call_stack.push(newscope);
    } else {
        // Log error
        if eval.state.debug {
            eval.state
                .error_log
                .push("Not enough arguments for ~ , Callstack error".to_string());
        }
    }
}

pub fn get_self(eval: &mut Evaluator) {
    if let Some(scope) = eval.state.call_stack.last_mut() {
        let mut core_self = vec![];

        for (ident, token) in scope {
            core_self.push(Token::Identifier(ident.clone()));
            core_self.push(token.clone());
            core_self.push(Token::Op(Operator::VariableAssign))
        }

        eval.state
            .execution_stack
            .push(Token::Block(Block::Parsed(Rc::new(core_self))))
    }
}
