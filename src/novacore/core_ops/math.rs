use crate::novacore::{core::Token, evaluator::Evaluator};

#[inline(always)]
pub fn sqrt(eval: &mut Evaluator) {
    match eval.state.get_from_heap_or_pop() {
        Some(left) => match &left {
            Token::Integer(left) => {
                eval.state
                    .execution_stack
                    .push(Token::Float((*left as f64).sqrt()));
            }
            Token::Float(left) => {
                eval.state.execution_stack.push(Token::Float(left.sqrt()));
            }
            _ => eval
                .state
                .show_error(&format!("Incorrect argument for sqrt, got [{:?}]", left)),
        },
        None => {
            eval.state.show_error("Not enough arguments for sqrt");
        }
    }
}

#[inline(always)]
pub fn pow(eval: &mut Evaluator) {
    match (eval.state.get_from_heap_or_pop(),eval.state.get_from_heap_or_pop()) {
        (Some(left),Some(right)) => match (&right ,&left) {
            (Token::Integer(left),Token::Integer(right)) => {
                eval.state
                    .execution_stack
                    .push(Token::Integer(i128::pow(*left, *right as u32)));
            }
            (Token::Integer(left),Token::Float(right)) => {
                eval.state
                    .execution_stack
                    .push(Token::Float(i128::pow(*left, *right as u32) as f64));
            }
            (Token::Float(left),Token::Integer(right)) => {
                eval.state
                    .execution_stack
                    .push(Token::Float(f64::powf(*left, *right as f64)));
            }
            (Token::Float(left),Token::Float(right)) => {
                eval.state
                    .execution_stack
                    .push(Token::Float(f64::powf(*left, *right)));
            }
            _ => eval
                .state
                .show_error(&format!("Incorrect argument for power, got [{:?}]", left)),
        },
        (_,_) => {
            eval.state.show_error("Not enough arguments for power");
        }
    }
}

pub fn round(eval: &mut Evaluator) {
    match eval.state.get_from_heap_or_pop() {
        Some(left) => match &left {
            Token::Integer(left) => {
                eval.state
                    .execution_stack
                    .push(Token::Float((*left as f64).round()));
            }
            Token::Float(left) => {
                eval.state.execution_stack.push(Token::Float(left.round()));
            }
            _ => eval
                .state
                .show_error(&format!("Incorrect argument for round, got [{:?}]", left)),
        },
        None => {
            eval.state.show_error("Not enough arguments for round");
        }
    }
}