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
            (Token::String(mut string1), Token::Char(char)) => {
                string1.push(char);
                eval.state.execution_stack.push(Token::String(string1))
            }
            (Token::String(mut string1), Token::String(string2)) => {
                string1 = string1 + &string2;
                eval.state.execution_stack.push(Token::String(string1))
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
    if let Some(list) = (eval.state.get_from_heap_or_pop()) {
        match list {
            Token::Block(Block::List(list)) => {
                let mut newlist = list.to_vec();
                if newlist.is_empty() {
                    eval
                    .state
                    .show_error("Pop failed, List is empty, ") 
                } else {
                    newlist.pop();
                    eval.state
                        .execution_stack
                        .push(Token::Block(Block::List(Rc::new(newlist))))
                }

            }
            Token::String(mut string1) => {
                if string1.is_empty() {
                    eval
                    .state
                    .show_error("Pop failed, String is empty, ") 
                } else {
                    string1.pop();
                    eval.state.execution_stack.push(Token::String(string1))
                }
            }
            list => eval
                .state
                .show_error(&format!("Incorrect arguments for pop, got [{:?}]", list)),
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
                } else {
                    eval
                    .state
                    .show_error("Last failed, List is empty, ") 
                }
            }
            Token::String(mut string1) => {
                if let Some(last) = string1.pop() {
                    eval.state.execution_stack.push(Token::Char(last))
                }else {
                    eval
                    .state
                    .show_error("Last failed, String is empty, ") 
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
            (Token::String(string), Token::Integer(index), Token::Char(item)) => {
                let mut newlist = string;
                if index as usize <= newlist.len() {
                    newlist.insert(index as usize, item);
                    eval.state.execution_stack.push(Token::String(newlist))
                } else {
                    newlist.push(item);
                    eval.state.execution_stack.push(Token::String(newlist))
                }
            }
            (Token::String(string), Token::Integer(index), Token::String(item)) => {
                let mut newlist = string;
                if index as usize <= newlist.len() {
                    newlist.insert_str(index as usize, &item);
                    eval.state.execution_stack.push(Token::String(newlist))
                } else {
                    newlist.push_str(&item);
                    eval.state.execution_stack.push(Token::String(newlist))
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
            (Token::String(list), Token::Integer(index)) => {
                let mut newlist = list;
                if index as usize <= newlist.len() {
                    newlist.remove(index as usize);
                    eval.state.execution_stack.push(Token::String(newlist))
                } else {
                    newlist.pop();
                    eval.state.execution_stack.push(Token::String(newlist))
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
