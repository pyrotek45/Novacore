use std::rc::Rc;

use crate::novacore::{
    core::{Block, Token},
    evaluator::Evaluator,
    utilities::print_error,
};

pub fn create_range(eval: &mut Evaluator) {
    if let (Some(ref end), Some(ref start)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        match (start, end) {
            (Token::Integer(start), Token::Integer(end)) => {
                let mut new_list: Vec<Token> = Vec::new();
                for x in *start..=*end {
                    new_list.push(Token::Integer(x));
                }
                eval.state
                    .execution_stack
                    .push(Token::Block(Block::List(Rc::new(new_list.to_vec()))));
            }
            (a, b) => print_error(&format!(
                "Incorrect arguments for range , got [{:?},{:?}]",
                a, b
            )),
        }
    } else {
        print_error("Not enough arguments for range")
    }
}

pub fn collect(eval: &mut Evaluator) {
    let mut newlist = vec![];
    if let Some(list) = eval.state.get_from_heap_or_pop() {
        match list {
            Token::Block(Block::List(list)) => {
                for item in list.iter() {
                    match item {
                        Token::Identifier(ident) => {
                            if let Some(value) = eval.state.get_from_heap(ident) {
                                newlist.push(value)
                            } else {
                                newlist.push(item.clone())
                            }
                        }
                        _ => newlist.push(item.clone()),
                    }
                }
            }
            _ => print_error(&format!(
                "Incorrect arguments for collect , got [{:?}]",
                list
            )),
        }
        eval.state
            .execution_stack
            .push(Token::Block(Block::List(Rc::new(newlist))))
    } else {
        print_error("Not enough arguments for collect")
    }
}

pub fn iota(eval: &mut Evaluator) {
    if let Some(ref end) = eval.state.get_from_heap_or_pop() {
        match end {
            Token::Integer(end) => {
                let mut new_list: Vec<Token> = Vec::new();
                for x in 0..*end {
                    new_list.push(Token::Integer(x));
                }
                eval.state
                    .execution_stack
                    .push(Token::Block(Block::List(Rc::new(new_list.to_vec()))));
            }
            _ => print_error(&format!("Incorrect arguments for iota , got [{:?}]", end)),
        }
    } else {
        print_error("Not enough arguments for iota")
    }
}
