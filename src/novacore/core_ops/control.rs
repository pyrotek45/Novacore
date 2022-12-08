use std::{process::id, rc::Rc};

use hashbrown::HashMap;

use crate::novacore::{
    core::{Block, Operator, Token},
    evaluator::Evaluator,
};

// pub fn block_call(eval: &mut Evaluator) {
//     if let Some(token) = eval.state.get_from_heap_or_pop() {
//         if let Token::Block(block) = token {
//             match block {
//                 Block::Literal(block) => {
//                     // Call with new scope
//                     eval.state.call_stack.push(HashMap::new());

//                     eval.evaluate(block.to_vec());

//                     if let Some(token) = eval.state.get_from_heap_or_pop() {
//                         eval.state.execution_stack.push(token)
//                     }
//                     eval.state.call_stack.pop();
//                 }
//                 Block::Procedure(block) => {
//                     // call in same scope
//                     eval.evaluate(block.to_vec())
//                 }
//                 Block::Lambda(_) => todo!(),
//                 Block::Auto(block) => {todo!()}
//             }
//         } else {
//             println!("Cant call this type");
//         }
//     }
// }

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
                Block::Auto(setup, logic) => {
                    // Call with new scope
                    eval.state.call_stack.push(HashMap::new());

                    // run the setup + logic
                    eval.evaluate(setup.to_vec());
                    eval.evaluate(logic.to_vec());

                    // get block values
                    let mut core_self = vec![];
                    if let Some(scope) = eval.state.call_stack.last_mut() {
                        for (ident, token) in scope {
                            core_self.push(Token::Identifier(ident.clone()));
                            core_self.push(token.clone());
                            core_self.push(Token::Op(Operator::VariableAssign))
                        }
                    }

                    // for t in block.iter() {
                    //     core_self.push(t.clone())
                    // }

                    // set last item to stack
                    if let Some(token) = eval.state.get_from_heap_or_pop() {
                        eval.state.execution_stack.push(token)
                    }

                    // exit scope
                    eval.state.call_stack.pop();

                    // overwrite old block

                    if let Some(scope) = eval.state.call_stack.last_mut() {
                        scope.insert(
                            function_name.to_string(),
                            Token::Block(Block::Auto(Rc::new(core_self), Rc::new(logic.to_vec()))),
                        );
                    }
                }
                Block::Object(_) => todo!(),
                Block::Method(_) => todo!(),
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
                    match &variable {
                        Token::Identifier(inner_ident) => {
                            if let Some(token) = eval.state.get_from_heap(inner_ident) {
                                if variable_name != "_" {
                                    if let Some(scope) = eval.state.call_stack.last_mut() {
                                        scope.insert(variable_name.to_string(), token.clone());
                                    }
                                }
                                for t in block.iter() {
                                    eval.eval(t.clone());

                                    if eval.state.exit_loop {
                                        if let Some(scope) = eval.state.call_stack.last_mut() {
                                            scope.remove(&variable_name);
                                        }
                                        break 'outer1;
                                    }
                                    if eval.state.continue_loop {
                                        eval.state.continue_loop = false;
                                        if let Some(scope) = eval.state.call_stack.last_mut() {
                                            scope.remove(&variable_name);
                                        }
                                        continue 'outer1;
                                    }
                                }

                                if let Some(scope) = eval.state.call_stack.last_mut() {
                                    scope.remove(&variable_name);
                                }
                                eval.state.exit_loop = false;
                            }
                        }
                        _ => {
                            if variable_name != "_" {
                                if let Some(scope) = eval.state.call_stack.last_mut() {
                                    scope.insert(variable_name.to_string(), variable.clone());
                                }
                            }
                            for t in block.iter() {
                                eval.eval(t.clone());

                                if eval.state.exit_loop {
                                    if let Some(scope) = eval.state.call_stack.last_mut() {
                                        scope.remove(&variable_name);
                                    }
                                    break 'outer1;
                                }
                                if eval.state.continue_loop {
                                    eval.state.continue_loop = false;
                                    if let Some(scope) = eval.state.call_stack.last_mut() {
                                        scope.remove(&variable_name);
                                    }
                                    continue 'outer1;
                                }
                            }

                            if let Some(scope) = eval.state.call_stack.last_mut() {
                                scope.remove(&variable_name);
                            }
                            eval.state.exit_loop = false;
                        }
                    }
                }
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
            (
                Token::Block(Block::Literal(block)),
                Token::Bool(bool),
                Token::Identifier(variable_name),
            ) => {
                if bool {
                    'outer:  loop {

                        for t in block.iter() {
                            eval.eval(t.clone());
    
                            if eval.state.exit_loop {
                                break 'outer;
                            }
                            if eval.state.continue_loop {
                                eval.state.continue_loop = false;
                                continue 'outer;
                            }
                        }
                        
                        if let Some(scope) = eval.state.call_stack.last_mut() {
                            scope.remove(&variable_name);
                        }
                        eval.state.exit_loop = false;
                    }

                }

            }
            _ => {
                println!("cant make a iterate from these types");
            }
        }
    }
}

pub fn user_chain_call(eval: &mut Evaluator) {
    if let Some(token) = eval.state.temp.clone() {
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
                Block::Auto(_, _) => todo!(),
                Block::Object(_) => todo!(),
                Block::Method(method) => {
                    if let Some(Token::Block(Block::Object(object_state))) =
                        eval.state.get_from_heap(&eval.state.current_object_name)
                    {
                        // Call with new scope
                        eval.state.call_stack.push(HashMap::new());

                        eval.evaluate(object_state.to_vec());
                        eval.evaluate(method.to_vec());

                        let mut core_self = vec![];
                        if let Some(scope) = eval.state.call_stack.last_mut() {
                            for (ident, token) in scope {
                                core_self.push(Token::Identifier(ident.clone()));
                                core_self.push(token.clone());
                                core_self.push(Token::Op(Operator::VariableAssign))
                            }
                        }

                        if let Some(token) = eval.state.get_from_heap_or_pop() {
                            eval.state.execution_stack.push(token)
                        }

                        eval.state.call_stack.pop();

                        if let Some(scope) = eval.state.call_stack.last_mut() {
                            scope.insert(
                                eval.state.current_object_name.to_string(),
                                Token::Block(Block::Object(Rc::new(core_self))),
                            );
                        }
                    }
                }
            }
        } else {
            println!("Cant call this type");
        }
    }
}

pub fn get_access(eval: &mut Evaluator) {
    match (
        eval.state.execution_stack.pop(),
        eval.state.execution_stack.pop(),
    ) {
        (Some(Token::Identifier(ident)), Some(Token::Identifier(block))) => match ident.as_str() {
            "len" => eval
                .state
                .execution_stack
                .push(Token::Integer(block.len() as i128)),
            _ => match eval.state.get_from_heap(&block) {
                Some(Token::Block(blocktype)) => match blocktype {
                    Block::Literal(block) => {
                        eval.state.call_stack.push(HashMap::new());

                        for t in block.iter() {
                            eval.eval(t.clone())
                        }

                        if let Some(tok) = eval.state.get_from_heap(&ident) {
                            eval.state.execution_stack.push(tok)
                        }

                        eval.state.call_stack.pop();
                    }
                    Block::Lambda(_) => todo!(),
                    Block::Procedure(proc) => {
                        eval.evaluate(proc.to_vec());

                        if let Some(tok) = eval.state.get_from_heap(&ident) {
                            eval.state.execution_stack.push(tok)
                        }
                    }
                    Block::Auto(setup, _) => {
                        eval.state.call_stack.push(HashMap::new());

                        eval.evaluate(setup.to_vec());

                        if let Some(tok) = eval.state.get_from_heap(&ident) {
                            eval.state.execution_stack.push(tok)
                        }

                        eval.state.call_stack.pop();
                    }
                    Block::Object(object) => {
                        eval.state.current_object_name = block.clone();

                        eval.state.call_stack.push(HashMap::new());

                        eval.evaluate(object.to_vec());

                        if let Some(tok) = eval.state.get_from_heap(&ident) {
                            eval.state.execution_stack.push(tok)
                        }

                        eval.state.call_stack.pop();
                    }
                    Block::Method(_) => todo!(),
                },

                _ => {
                    println!("{} does not exist, cannot access", block);
                }
            },
        },
        (Some(Token::Identifier(ident)), Some(Token::Block(block))) => match block {
            Block::Literal(block) => {
                eval.state.call_stack.push(HashMap::new());

                for t in block.iter() {
                    eval.eval(t.clone())
                }

                if let Some(tok) = eval.state.get_from_heap(&ident) {
                    eval.state.execution_stack.push(tok)
                }

                eval.state.call_stack.pop();
            }
            Block::Lambda(_) => todo!(),
            Block::Procedure(proc) => {
                eval.evaluate(proc.to_vec());

                if let Some(tok) = eval.state.get_from_heap(&ident) {
                    eval.state.execution_stack.push(tok)
                }
            }
            Block::Auto(setup, _) => {
                eval.state.call_stack.push(HashMap::new());

                eval.evaluate(setup.to_vec());

                if let Some(tok) = eval.state.get_from_heap(&ident) {
                    eval.state.execution_stack.push(tok)
                }

                eval.state.call_stack.pop();
            }
            Block::Object(object) => {
                eval.state.call_stack.push(HashMap::new());

                eval.evaluate(object.to_vec());

                if let Some(tok) = eval.state.get_from_heap(&ident) {
                    eval.state.execution_stack.push(tok)
                }

                eval.state.call_stack.pop();
            }
            Block::Method(_) => todo!(),
        },
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
