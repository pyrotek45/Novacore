use std::rc::Rc;

use crate::novacore::{
    core::{Block, Token},
    evaluator::Evaluator,
};

#[inline(always)]
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
            (a, b) => eval.state.show_error(&format!(
                "Incorrect arguments for range , got [{:?},{:?}]",
                a, b
            )),
        }
    } else {
        eval.state.show_error("Not enough arguments for range")
    }
}

#[inline(always)]
pub fn collect(eval: &mut Evaluator) {
    let mut newlist = vec![];
    if let Some(list) = eval.state.get_from_heap_or_pop() {
        match list {
            Token::Block(Block::List(list)) => {
                for item in list.iter() {
                    match item {
                        Token::Id(ident) => {
                            if let Some(value) = eval.state.get_from_heap(ident) {
                                newlist.push(value)
                            } else {
                                newlist.push(item.clone())
                            }
                        }
                        _ => newlist.push(item.clone()),
                    }
                }
                eval.state
                    .execution_stack
                    .push(Token::Block(Block::List(Rc::new(newlist))))
            }
            Token::Block(Block::Literal(list)) => {
                for item in list.iter() {
                    match item {
                        Token::Id(ident) => {
                            if let Some(value) = eval.state.get_from_heap(ident) {
                                newlist.push(value)
                            } else {
                                newlist.push(item.clone())
                            }
                        }
                        _ => newlist.push(item.clone()),
                    }
                }
                eval.state
                    .execution_stack
                    .push(Token::Block(Block::Literal(Rc::new(newlist))))
            }
            // Token::Block(Block::Function(_,list)) => {
            //     for item in list.iter() {
            //         match item {
            //             Token::Id(ident) => {
            //                 if let Some(value) = eval.state.get_from_heap(ident) {
            //                     newlist.push(value)
            //                 } else {
            //                     newlist.push(item.clone())
            //                 }
            //             }
            //             _ => newlist.push(item.clone()),
            //         }
            //     }
            //     eval.state
            //         .execution_stack
            //         .push(Token::Block(Block::Function(Rc::new()),Rc::new(newlist))))
            // }
            _ => eval.state.show_error(&format!(
                "Incorrect arguments for collect , got [{:?}]",
                list
            )),
        }
    } else {
        eval.state.show_error("Not enough arguments for collect")
    }
}

#[inline(always)]
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
            _ => eval
                .state
                .show_error(&format!("Incorrect arguments for iota , got [{:?}]", end)),
        }
    } else {
        eval.state.show_error("Not enough arguments for iota")
    }
}
