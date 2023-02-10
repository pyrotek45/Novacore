use std::rc::Rc;

use crate::novacore::{
    core::{Block, Token},
    evaluator::Evaluator,
};

pub fn list_push(eval: &mut Evaluator) {
    if let (Some(token), Some(list)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        match (list, token) {
            (Token::Block(Block::List(list)), token) => {
                let mut newlist = list.to_vec();
                newlist.push(token);
                eval.state
                    .execution_stack
                    .push(Token::Block(Block::List(Rc::new(newlist))))
            }
            (list, token) => eval.state.show_error(&format!(
                "Incorrect arguments for push, got [{:?},{:?}]",
                list, token
            )),
        }
    } else {
        eval.state.show_error("Not enough arguments for push");
    }
}

pub fn list_pop(eval: &mut Evaluator) {
    if let (Some(token), Some(list)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.execution_stack.pop(),
    ) {
        match (list, token) {
            (Token::Block(Block::List(list)), Token::Id(ident)) => {
                let mut newlist = list.to_vec();
                if let Some(value) = newlist.pop() {
                    eval.state.add_varaible(&ident, value)
                }
                eval.state
                    .execution_stack
                    .push(Token::Block(Block::List(Rc::new(newlist))))
            }
            (list, token) => eval.state.show_error(&format!(
                "Incorrect arguments for pop, got [{:?},{:?}]",
                list, token
            )),
        }
    } else {
        eval.state.show_error("Not enough arguments for pop");
    }
}

pub fn list_last(eval: &mut Evaluator) {
    if let Some(list) = eval.state.get_from_heap_or_pop() {
        match list {
            Token::Block(Block::List(list)) => {
                if let Some(token) = list.last() {
                    eval.state.execution_stack.push(token.clone())
                }
            }
            list => eval
                .state
                .show_error(&format!("Incorrect arguments for last, got [{:?}]", list)),
        }
    } else {
        eval.state.show_error("Not enough arguments for last");
    }
}

pub fn list_insert(eval: &mut Evaluator) {
    if let (Some(item), Some(index), Some(list)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        match (list, index, item) {
            (Token::Block(Block::List(list)), Token::Integer(index), item) => {
                let mut newlist = list.to_vec();
                if index as usize <= list.len() {
                    newlist.insert(index as usize, item);
                    eval.state
                        .execution_stack
                        .push(Token::Block(Block::List(Rc::new(newlist))))
                } else {
                    newlist.push(item);
                    eval.state
                        .execution_stack
                        .push(Token::Block(Block::List(Rc::new(newlist))))
                }
            }
            (list, index, item) => eval.state.show_error(&format!(
                "Incorrect arguments for insert, got [{:?},{:?},{:?}]",
                list, index, item
            )),
        }
    } else {
        eval.state.show_error("Not enough arguments for insert");
    }
}

pub fn list_remove(eval: &mut Evaluator) {
    if let (Some(index), Some(list)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        match (list, index) {
            (Token::Block(Block::List(list)), Token::Integer(index)) => {
                let mut newlist = list.to_vec();
                if index as usize <= list.len() {
                    newlist.remove(index as usize);
                    eval.state
                        .execution_stack
                        .push(Token::Block(Block::List(Rc::new(newlist))))
                } else {
                    newlist.pop();
                    eval.state
                        .execution_stack
                        .push(Token::Block(Block::List(Rc::new(newlist))))
                }
            }
            (list, index) => eval.state.show_error(&format!(
                "Incorrect arguments for remove, got [{:?},{:?}]",
                list, index
            )),
        }
    } else {
        eval.state.show_error("Not enough arguments for remove");
    }
}

// pub fn list_remove(eval: &mut Evaluator) {
//     todo!()
// }

// pub fn list_map(eval: &mut Evaluator) {
//     todo!()
// }

// pub fn list_filter(eval: &mut Evaluator) {
//     todo!()
// }

// pub fn list_fold(eval: &mut Evaluator) {
//     todo!()
// }

// pub fn list_car(eval: &mut Evaluator) {
//     todo!()
// }

// pub fn cdr(eval: &mut Evaluator) {
//     todo!()
// }

// pub fn list_chop(eval: &mut Evaluator) {
//     todo!()
// }
