use std::rc::Rc;

use hashbrown::HashMap;
use modulo::Mod;

use crate::novacore::{
    core::{Block, Token},
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

            (a, b) => eval.state.show_error(&format!(
                "Incorrect arguments for addition. got [{:?},{:?}]",
                a, b
            )),
        }
    } else {
        eval.state.show_error("Not enough arguments for addition")
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
            (a, b) => eval.state.show_error(&format!(
                "Incorrect arguments for division. got [{:?},{:?}]",
                a, b
            )),
        }
    } else {
        eval.state.show_error("Not enough arguments for division")
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
            a => eval
                .state
                .show_error(&format!("Incorrect arguments for inversion. got [{:?}]", a)),
        }
    } else {
        eval.state.show_error("Not enough arguments for inversion")
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
            (a, b) => eval.state.show_error(&format!(
                "Incorrect arguments for subtraction. got [{:?},{:?}]",
                a, b
            )),
        }
    } else {
        eval.state
            .show_error("Not enough arguments for subtraction")
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
            (a, b) => eval.state.show_error(&format!(
                "Incorrect arguments for modulo %. got [{:?},{:?}]",
                a, b
            )),
        }
    } else {
        eval.state.show_error("Not enough arguments for modulo %")
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
            (a, b) => eval.state.show_error(&format!(
                "Incorrect arguments for multiplication. got [{:?},{:?}]",
                a, b
            )),
        }
    } else {
        eval.state
            .show_error("Not enough arguments for multiplication")
    }
}

pub fn variable_assign(eval: &mut Evaluator) {
    if let (Some(token), Some(ident)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.execution_stack.pop(),
    ) {
        match (&token, &ident) {
            (Token::Id(moved), Token::Id(identifier)) => {
                eval.state.move_varaible(moved, identifier)
            }
            (_, Token::Id(identifier)) => eval.state.add_varaible(identifier, token),
            _ => {
                eval.state.show_error(&format!(
                    "Can not assign these two types [{:?},{:?}]",
                    token, ident
                ));
            }
        }
    } else {
        eval.state
            .show_error("Not enough arguments for variable assignment");
    }
}

pub fn variable_assign_set(eval: &mut Evaluator) {
    if let (Some(ident), Some(token)) = (
        eval.state.execution_stack.pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        match (&token, &ident) {
            (Token::Id(moved), Token::Id(identifier)) => {
                eval.state.move_varaible(moved, identifier)
            }
            (_, Token::Id(identifier)) => eval.state.add_varaible(identifier, token),
            _ => {
                eval.state.show_error(&format!(
                    "Can not assign these two types [{:?},{:?}]",
                    token, ident
                ));
            }
        }
    } else {
        eval.state
            .show_error("Not enough arguments for variable assignment");
    }
}

pub fn bind_variables(eval: &mut Evaluator) {
    let mut variable_stack: Vec<String> = Vec::with_capacity(10);
    if let Some(Token::Block(Block::List(identifiers))) = eval.state.get_from_heap_or_pop() {
        for toks in identifiers.iter().rev() {
            if let Token::Id(ident) = &toks {
                variable_stack.push(ident.clone())
            }
        }
    } else {
        eval.state.show_error("Not enough arguments for [->]");
    }

    // Tie each Token into the call_stack using the tokens poped
    let mut newscope = HashMap::new();
    for tokens in variable_stack {
        if let Some(tok) = eval.state.get_from_heap_or_pop() {
            newscope.insert(tokens, tok.clone());
        } else {
            eval.state.show_error("Not enough arguments for -> ")
        }
    }
    eval.state.bindings.push(newscope);
}

pub fn pop_bindings(eval: &mut Evaluator) {
    eval.state.bindings.pop();
}

pub fn get_new(eval: &mut Evaluator) {
    if let Some(scope) = eval.state.call_stack.last_mut() {
        let mut core_self = HashMap::new();

        for (ident, token) in scope {
            core_self.insert(ident.clone(), token.clone());
        }

        eval.state
            .execution_stack
            .push(Token::Block(Block::Struct(Rc::new(core_self))))
    }
}

pub fn free(eval: &mut Evaluator) {
    if let Some(token) = eval.state.execution_stack.pop() {
        if let Token::Id(ident) = token {
            eval.state.remove_varaible(&ident)
        }
    } else {
        eval.state.show_error("Not enough arguments for free");
    }
}

pub fn resolve(eval: &mut Evaluator) {
    if let Some(top) = eval.state.get_from_heap_or_pop() {
        eval.state.execution_stack.push(top)
    } else {
        eval.state.show_error("Not enough arguments for return");
    }
}

pub fn resolve_binding(eval: &mut Evaluator) {
    if let Some(top) = eval.state.get_from_binding() {
        eval.state.execution_stack.push(top)
    } else {
        eval.state
            .show_error("Not enough arguments for binding resolve");
    }
}
