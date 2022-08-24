use std::thread::ScopedJoinHandle;

use hashbrown::HashMap;

use crate::novacore::{
    core::{Block, Token},
    evaluator::Evaluator,
    state::{self, State},
};

pub fn user_block_call(
    mut state: Box<state::State>,
    eval: &mut Evaluator,
    function_name: &str,
) -> Box<state::State> {
    if let Some(token) = state.get_from_heap(function_name) {
        if let Token::Block(block) = token {
            match block {
                Block::Literal(block) => {
                    // Call with new scope
                    state.call_stack.push(HashMap::new());

                    state = eval.evaluate(block.to_vec(), state);

                    if let Some(token) = state.get_from_heap_or_pop() {
                        state.execution_stack.push(token)
                    }
                    state.call_stack.pop();
                }
                Block::Procedure(block) => {
                    // call in same scope
                    state = eval.evaluate(block.to_vec(), state)
                }
                Block::Lambda(_) => todo!(),
            }
        } else {
            println!("Cant call this type");
        }
    }

    state
}

pub fn if_statement(mut state: Box<state::State>, eval: &mut Evaluator) -> Box<state::State> {
    if let (Some(block), Some(boolmaybe)) =
        (state.get_from_heap_or_pop(), state.get_from_heap_or_pop())
    {
        //if true single if statement
        if let Token::Bool(bool) = boolmaybe {
            if bool {
                if let Token::Block(Block::Literal(block)) = block {
                    state = eval.evaluate(block.to_vec(), state)
                }
            }
        } else if let Some(Token::Bool(bool)) = state.get_from_heap_or_pop() {
            if bool {
                if let Token::Block(Block::Literal(block)) = boolmaybe {
                    state = eval.evaluate(block.to_vec(), state)
                }
            } else if let Token::Block(Block::Literal(block)) = block {
                state = eval.evaluate(block.to_vec(), state)
            }
        }
    }

    state
}

pub fn break_loop(mut state: Box<state::State>) -> Box<state::State> {
    state.exit_loop = true;

    state
}

pub fn for_loop(state: Box<state::State>, eval: &mut Evaluator) -> Box<state::State> {
    let mut state = state;
    if let (Some(block), Some(list), Some(variable)) = (
        state.get_from_heap_or_pop(),
        state.get_from_heap_or_pop(),
        state.execution_stack.pop(),
    ) {
        match (block, list, variable) {
            (
                Token::Block(Block::Literal(block)),
                Token::List(ref list),
                Token::Identifier(variable_name),
            ) => {
                'outer1: for variable in list.iter() {
                    if variable_name != "_" {
                        if let Some(scope) = state.call_stack.last_mut() {
                            scope.insert(variable_name.to_string(), variable.clone());
                        }
                    }
                    for t in block.iter() {
                        state = eval.eval(state, t.clone());

                        if state.exit_loop {
                            break 'outer1;
                        }
                        if state.continue_loop {
                            state.continue_loop = false;
                            continue 'outer1;
                        }
                    }
                }
                if let Some(scope) = state.call_stack.last_mut() {
                    scope.remove(&variable_name);
                }
                state.exit_loop = false;
            }
            _ => {
                println!("cant make a iterate from these types");
            }
        }
    }

    state
}

pub fn user_chain_call(mut state: Box<state::State>, eval: &mut Evaluator) -> Box<state::State> {
    if let Some(token) = &state.temp {
        if let Token::Block(block) = token {
            match block {
                Block::Literal(block) => {
                    // Call with new scope
                    state.call_stack.push(HashMap::new());

                    state = eval.evaluate(block.to_vec(), state);

                    if let Some(token) = state.get_from_heap_or_pop() {
                        state.execution_stack.push(token)
                    }
                    state.call_stack.pop();
                }
                Block::Procedure(block) => {
                    // call in same scope
                    state = eval.evaluate(block.to_vec(), state)
                }
                Block::Lambda(_) => todo!(),
            }
        } else {
            println!("Cant call this type");
        }
    }

    state
}

pub fn get_access(mut state: Box<state::State>, eval: &mut Evaluator) -> Box<state::State> {
    match (state.execution_stack.pop(), state.get_from_heap_or_pop()) {
        (Some(Token::Identifier(ident)), Some(Token::Block(Block::Literal(block)))) => {
            match ident.as_str() {
                "len" => state
                    .execution_stack
                    .push(Token::Integer(block.len() as i128)),
                _ => {
                    state.call_stack.push(HashMap::new());

                    for t in block.iter() {
                        state = eval.eval(state, t.clone())
                    }

                    if let Some(tok) = state.get_from_heap(&ident) {
                        state.execution_stack.push(tok)
                    }

                    state.call_stack.pop();
                }
            }
        }
        _ => {
            println!("Cant access this");
        }
    }

    state
}

pub fn store_temp(mut state: Box<state::State>) -> Box<state::State> {
    match state.get_from_heap_or_pop() {
        Some(token) => {
            state.temp = Some(token);
            state
        }
        None => state,
    }
}
