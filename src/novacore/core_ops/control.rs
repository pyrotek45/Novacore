use hashbrown::HashMap;

use crate::novacore::{
    core::{Block, Token},
    evaluator::Evaluator,
};

pub fn block_call(eval: &mut Evaluator) {
    if let Some(token) = eval.state.get_from_heap_or_pop() {
        if let Token::Block(block) = token {
            match block {
                Block::Literal(block) => {
                    // Call with new scope
                    eval.state.call_stack.push(HashMap::new());

                    eval.evaluate(block.to_vec());

                    if let Some(token) = eval.state.get_from_heap_or_pop() {
                        eval.state.execution_stack.push(token)
                    }
                    eval.state.call_stack.pop();
                }
                Block::Procedure(block) => {
                    // call in same scope
                    eval.evaluate(block.to_vec())
                }
                Block::Lambda(_) => todo!(),
            }
        } else {
            println!("Cant call this type");
        }
    }
}

pub fn user_block_call(eval: &mut Evaluator, function_name: &str) {
    if let Some(token) = eval.state.get_from_heap(function_name) {
        if let Token::Block(block) = token {
            match block {
                Block::Literal(block) => {
                    // Call with new scope
                    eval.state.call_stack.push(HashMap::new());

                    eval.evaluate(block.to_vec());

                    if let Some(token) = eval.state.get_from_heap_or_pop() {
                        eval.state.execution_stack.push(token)
                    }
                    eval.state.call_stack.pop();
                }
                Block::Procedure(block) => {
                    // call in same scope
                    eval.evaluate(block.to_vec())
                }
                Block::Lambda(_) => todo!(),
            }
        } else {
            println!("Cant call this type");
        }
    }
}

pub fn if_statement(eval: &mut Evaluator) {
    if let (Some(block), Some(boolmaybe)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        //if true single if statement
        if let Token::Bool(bool) = boolmaybe {
            if bool {
                if let Token::Block(Block::Literal(block)) = block {
                    eval.evaluate(block.to_vec())
                }
            }
        } else if let Some(Token::Bool(bool)) = eval.state.get_from_heap_or_pop() {
            if bool {
                if let Token::Block(Block::Literal(block)) = boolmaybe {
                    eval.evaluate(block.to_vec())
                }
            } else if let Token::Block(Block::Literal(block)) = block {
                eval.evaluate(block.to_vec())
            }
        }
    }
}

pub fn break_loop(eval: &mut Evaluator) {
    eval.state.exit_loop = true;
}

pub fn for_loop(eval: &mut Evaluator) {
    if let (Some(block), Some(list), Some(variable)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
        eval.state.execution_stack.pop(),
    ) {
        match (block, list, variable) {
            (
                Token::Block(Block::Literal(block)),
                Token::List(ref list),
                Token::Identifier(variable_name),
            ) => {
                'outer1: for variable in list.iter() {
                    if variable_name != "_" {
                        if let Some(scope) = eval.state.call_stack.last_mut() {
                            scope.insert(variable_name.to_string(), variable.clone());
                        }
                    }
                    for t in block.iter() {
                        eval.eval(t.clone());

                        if eval.state.exit_loop {
                            break 'outer1;
                        }
                        if eval.state.continue_loop {
                            eval.state.continue_loop = false;
                            continue 'outer1;
                        }
                    }
                }
                if let Some(scope) = eval.state.call_stack.last_mut() {
                    scope.remove(&variable_name);
                }
                eval.state.exit_loop = false;
            }
            (
                Token::Block(Block::Literal(block)),
                Token::Block(Block::Literal(ref list)),
                Token::Identifier(variable_name),
            ) => {
                'outer1: for variable in list.iter() {
                    if variable_name != "_" {
                        if let Some(scope) = eval.state.call_stack.last_mut() {
                            scope.insert(variable_name.to_string(), variable.clone());
                        }
                    }
                    for t in block.iter() {
                        eval.eval(t.clone());

                        if eval.state.exit_loop {
                            break 'outer1;
                        }
                        if eval.state.continue_loop {
                            eval.state.continue_loop = false;
                            continue 'outer1;
                        }
                    }
                }
                if let Some(scope) = eval.state.call_stack.last_mut() {
                    scope.remove(&variable_name);
                }
                eval.state.exit_loop = false;
            }
            _ => {
                println!("cant make a iterate from these types");
            }
        }
    }
}

pub fn user_chain_call(eval: &mut Evaluator) {
    if let Some(token) = &eval.state.temp {
        if let Token::Block(block) = token {
            match block {
                Block::Literal(block) => {
                    // Call with new scope
                    eval.state.call_stack.push(HashMap::new());

                    eval.evaluate(block.to_vec());

                    if let Some(token) = eval.state.get_from_heap_or_pop() {
                        eval.state.execution_stack.push(token)
                    }
                    eval.state.call_stack.pop();
                }
                Block::Procedure(block) => {
                    // call in same scope
                    eval.evaluate(block.to_vec())
                }
                Block::Lambda(_) => todo!(),
            }
        } else {
            println!("Cant call this type");
        }
    }
}

pub fn get_access(eval: &mut Evaluator) {
    match (
        eval.state.execution_stack.pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        (Some(Token::Identifier(ident)), Some(Token::Block(Block::Literal(block)))) => {
            match ident.as_str() {
                "len" => eval
                    .state
                    .execution_stack
                    .push(Token::Integer(block.len() as i128)),
                _ => {
                    eval.state.call_stack.push(HashMap::new());

                    for t in block.iter() {
                        eval.eval(t.clone())
                    }

                    if let Some(tok) = eval.state.get_from_heap(&ident) {
                        eval.state.execution_stack.push(tok)
                    }

                    eval.state.call_stack.pop();
                }
            }
        }
        _ => {
            println!("Cant access this");
        }
    }
}

pub fn store_temp(eval: &mut Evaluator) {
    if let Some(token) = eval.state.get_from_heap_or_pop() {
        eval.state.temp = Some(token);
    }
}
