use std::{ops::Index, rc::Rc};

use hashbrown::HashMap;

use crate::novacore::{
    core::{Block, Instructions, Operator, Token},
    evaluator::Evaluator,
    utilities::print_error,
};

pub fn block_call(eval: &mut Evaluator) {
    if let Some(token) = eval.state.get_from_heap_or_pop() {
        if let Token::Block(block) = token {
            match block {
                Block::Function(block) => {
                    // Call with new scope
                    eval.state.call_stack.push(HashMap::new());

                    eval.evaluate(block.to_vec());

                    if let Some(token) = eval.state.get_from_heap_or_pop() {
                        eval.state.execution_stack.push(token)
                    }
                    eval.state.call_stack.pop();
                }
                Block::Literal(block) => {
                    // call in same scope
                    eval.evaluate(block.to_vec())
                }
                Block::List(list) => {
                    // call in same scope
                    if let Some(key) = eval.state.get_from_heap_or_pop() {
                        if let Token::Integer(index) = key {
                            if let Some(value) = list.get(index as usize) {
                                eval.state.execution_stack.push(value.clone())
                            }
                        }
                    }
                }
                Block::Struct(data) => {
                    if let Some(key) = eval.state.execution_stack.pop() {
                        if let Token::Identifier(key) = key {
                            if let Some(value) = data.get(&key) {
                                eval.state.execution_stack.push(value.clone())
                            }
                        }
                    }
                }
                _ => {
                    todo!()
                }
            }
        } else {
            println!("Cant call this type {:?}", token);
        }
    }
}

pub fn user_block_call(eval: &mut Evaluator, function_name: &str) {
    if let Some(token) = eval.state.get_from_heap(function_name) {
        if let Token::Block(block) = token {
            match block {
                Block::Literal(block) => eval.evaluate(block.to_vec()),
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
                    //println!("UBC saving to {}", function_name);
                    if let Some(scope) = eval.state.call_stack.last_mut() {
                        scope.insert(
                            function_name.to_string(),
                            Token::Block(Block::Auto(Rc::new(core_self), Rc::new(logic.to_vec()))),
                        );
                    }
                }
                Block::Modifier(object_name, method) => {
                    if let Some(object_name) = object_name {
                        if let Some(Token::Block(Block::Literal(object_state))) =
                            eval.state.get_from_heap(&object_name)
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
                            // if let Some(scope) = eval.state.call_stack.last_mut() {
                            //     scope.insert(
                            //         object_name.to_string(),
                            //         Token::Block(Block::Literal(Rc::new(core_self))),
                            //     );
                            // }
                        }
                    }
                }
                Block::Function(block) => {
                    // Call with new scope
                    eval.state.call_stack.push(HashMap::new());

                    eval.evaluate(block.to_vec());

                    if let Some(token) = eval.state.get_from_heap_or_pop() {
                        eval.state.execution_stack.push(token)
                    }
                    eval.state.call_stack.pop();
                }
                Block::List(list) => {
                    // call in same scope
                    if let Some(key) = eval.state.get_from_heap_or_pop() {
                        if let Token::Integer(index) = key {
                            if let Some(value) = list.get(index as usize) {
                                eval.state.execution_stack.push(value.clone())
                            }
                        }
                    }
                }
                Block::Lambda(_) => todo!(),
                Block::ListLambda(_) => todo!(),
                Block::Struct(data) => {
                    if let Some(key) = eval.state.execution_stack.pop() {
                        if let Token::Identifier(key) = key {
                            if let Some(value) = data.get(&key) {
                                eval.state.execution_stack.push(value.clone())
                            }
                        }
                    }
                }
            }
        } else {
            println!("Cant call this type {:?}", token);
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
                match block {
                    Token::Block(Block::Literal(block)) => eval.evaluate(block.to_vec()),
                    Token::Block(Block::List(block)) => eval.evaluate(block.to_vec()),
                    _ => todo!(),
                }
            }
        } else if let Some(Token::Bool(bool)) = eval.state.get_from_heap_or_pop() {
            if bool {
                match boolmaybe {
                    Token::Block(Block::Literal(block)) => eval.evaluate(block.to_vec()),
                    Token::Block(Block::List(block)) => eval.evaluate(block.to_vec()),
                    _ => todo!(),
                }
            } else {
                match block {
                    Token::Block(Block::Literal(block)) => eval.evaluate(block.to_vec()),
                    Token::Block(Block::List(block)) => eval.evaluate(block.to_vec()),
                    _ => todo!(),
                }
            }
        }
    }
}

pub fn while_loop(eval: &mut Evaluator) {
    if let (Some(logic), Some(testing)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        match (testing, logic) {
            (Token::Block(Block::Literal(test)), Token::Block(Block::Literal(logic))) => {
                'outer: loop {
                    // run test block
                    for t in test.iter() {
                        eval.eval(t.clone());
                        if eval.state.break_loop.pop().is_some() {
                            break 'outer;
                        }
                        if eval.state.continue_loop.pop().is_some() {
                            continue 'outer;
                        }
                    }

                    // get result and run logic block if true is on stack else break
                    if let Some(Token::Bool(bool)) = eval.state.get_from_heap_or_pop() {
                        if bool {
                            for t in logic.iter() {
                                eval.eval(t.clone());
                                if eval.state.break_loop.pop().is_some() {
                                    break 'outer;
                                }
                                if eval.state.continue_loop.pop().is_some() {
                                    continue 'outer;
                                }
                            }
                        } else {
                            break 'outer;
                        }
                    } else {
                        break 'outer;
                    }
                }
            }
            (_, _) => todo!(),
        }
    }
}

pub fn times(eval: &mut Evaluator) {
    let (logic, times) = match (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        (Some(Token::Block(logic)), Some(Token::Integer(times))) => (logic, times),
        _ => return todo!(),
    };

    let logic = match logic {
        Block::Literal(logic) => logic,
        Block::Lambda(_) => todo!(),
        Block::Function(logic) => logic,
        Block::Auto(_, _) => todo!(),
        Block::Modifier(_, _) => todo!(),
        Block::List(logic) => logic,
        Block::ListLambda(_) => todo!(),
        Block::Struct(_) => todo!(),
    };

    'outer: for _ in 0..times {
        for t in logic.iter() {
            eval.eval(t.clone());
            if eval.state.break_loop.pop().is_some() {
                break 'outer;
            }
            if eval.state.continue_loop.pop().is_some() {
                continue 'outer;
            }
        }
    }
}

pub fn each(eval: &mut Evaluator) {
    if let (Some(logic), Some(items)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        fn each_compute(eval: &mut Evaluator, items: Instructions, logic: Instructions) {
            'outer: for item in items.iter() {
                eval.state.execution_stack.push(item.clone());
                for t in logic.iter() {
                    eval.eval(t.clone());
                    if eval.state.break_loop.pop().is_some() {
                        break 'outer;
                    }
                    if eval.state.continue_loop.pop().is_some() {
                        continue 'outer;
                    }
                }
            }
        }
        match (items, logic) {
            (Token::Block(items), Token::Block(logic)) => match (items, logic) {
                (Block::Literal(items), Block::Literal(logic)) => each_compute(eval, items, logic),
                (Block::List(items), Block::Literal(logic)) => each_compute(eval, items, logic),
                (Block::Literal(items), Block::Function(logic)) => each_compute(eval, items, logic),
                (Block::List(items), Block::Function(logic)) => each_compute(eval, items, logic),
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}

pub fn break_loop(eval: &mut Evaluator) {
    eval.state.break_loop.push(true);
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
                Token::Block(Block::List(list)),
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
                                    if eval.state.break_loop.pop().is_some() {
                                        if let Some(scope) = eval.state.call_stack.last_mut() {
                                            scope.remove(&variable_name);
                                        }
                                        break 'outer1;
                                    }

                                    if eval.state.continue_loop.pop().is_some() {
                                        eval.state.continue_loop.pop();
                                        if let Some(scope) = eval.state.call_stack.last_mut() {
                                            scope.remove(&variable_name);
                                        }
                                        continue 'outer1;
                                    }
                                }

                                if let Some(scope) = eval.state.call_stack.last_mut() {
                                    scope.remove(&variable_name);
                                }
                                eval.state.break_loop.pop();
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
                                if eval.state.break_loop.pop().is_some() {
                                    if let Some(scope) = eval.state.call_stack.last_mut() {
                                        scope.remove(&variable_name);
                                    }
                                    break 'outer1;
                                }

                                if eval.state.continue_loop.pop().is_some() {
                                    eval.state.continue_loop.pop();
                                    if let Some(scope) = eval.state.call_stack.last_mut() {
                                        scope.remove(&variable_name);
                                    }
                                    continue 'outer1;
                                }
                            }

                            if let Some(scope) = eval.state.call_stack.last_mut() {
                                scope.remove(&variable_name);
                            }
                            eval.state.break_loop.pop();
                        }
                    }
                }
            }
            (
                Token::Block(Block::Literal(block)),
                Token::Block(Block::Literal(list)),
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
                                    if eval.state.break_loop.pop().is_some() {
                                        if let Some(scope) = eval.state.call_stack.last_mut() {
                                            scope.remove(&variable_name);
                                        }
                                        break 'outer1;
                                    }

                                    if eval.state.continue_loop.pop().is_some() {
                                        eval.state.continue_loop.pop();
                                        if let Some(scope) = eval.state.call_stack.last_mut() {
                                            scope.remove(&variable_name);
                                        }
                                        continue 'outer1;
                                    }
                                }

                                if let Some(scope) = eval.state.call_stack.last_mut() {
                                    scope.remove(&variable_name);
                                }
                                eval.state.break_loop.pop();
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
                                if eval.state.break_loop.pop().is_some() {
                                    if let Some(scope) = eval.state.call_stack.last_mut() {
                                        scope.remove(&variable_name);
                                    }
                                    break 'outer1;
                                }

                                if eval.state.continue_loop.pop().is_some() {
                                    eval.state.continue_loop.pop();
                                    if let Some(scope) = eval.state.call_stack.last_mut() {
                                        scope.remove(&variable_name);
                                    }
                                    continue 'outer1;
                                }
                            }

                            if let Some(scope) = eval.state.call_stack.last_mut() {
                                scope.remove(&variable_name);
                            }
                            eval.state.break_loop.pop();
                        }
                    }
                }
            }
            (
                Token::Block(Block::Literal(block)),
                Token::Bool(bool),
                Token::Identifier(variable_name),
            ) => {
                if bool {
                    'outer: loop {
                        for t in block.iter() {
                            eval.eval(t.clone());
                            if eval.state.break_loop.pop().is_some() {
                                if let Some(scope) = eval.state.call_stack.last_mut() {
                                    scope.remove(&variable_name);
                                }
                                break 'outer;
                            }

                            if eval.state.continue_loop.pop().is_some() {
                                eval.state.continue_loop.pop();
                                if let Some(scope) = eval.state.call_stack.last_mut() {
                                    scope.remove(&variable_name);
                                }
                                continue 'outer;
                            }
                        }
                        if let Some(scope) = eval.state.call_stack.last_mut() {
                            scope.remove(&variable_name);
                        }
                        eval.state.break_loop.pop();
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
    if let Some(token) = eval.state.temp.last().cloned() {
        if let Token::Block(block) = token {
            match block {
                Block::Function(block) => {
                    // Call with new scope
                    eval.state.call_stack.push(HashMap::new());

                    eval.evaluate(block.to_vec());

                    if let Some(token) = eval.state.get_from_heap_or_pop() {
                        eval.state.execution_stack.push(token)
                    }
                    eval.state.call_stack.pop();
                }
                Block::Auto(_, _) => todo!(),
                Block::Modifier(object_name, method) => {
                    if let Some(object_name) = object_name {
                        if let Some(Token::Block(Block::Literal(object_state))) =
                            eval.state.get_from_heap(&object_name)
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
                            // if let Some(scope) = eval.state.call_stack.last_mut() {
                            //     scope.insert(
                            //         object_name.to_string(),
                            //         Token::Block(Block::Literal(Rc::new(core_self))),
                            //     );
                            // }
                        }
                    }
                }
                Block::Literal(_) => todo!(),
                Block::List(list) => {
                    // call in same scope
                    if let Some(key) = eval.state.get_from_heap_or_pop() {
                        if let Token::Integer(index) = key {
                            if let Some(value) = list.get(index as usize) {
                                eval.state.execution_stack.push(value.clone())
                            }
                        }
                    }
                }
                Block::Lambda(_) => todo!(),
                Block::ListLambda(_) => todo!(),
                Block::Struct(data) => {
                    if let Some(key) = eval.state.execution_stack.pop() {
                        if let Token::Identifier(key) = key {
                            if let Some(value) = data.get(&key) {
                                eval.state.execution_stack.push(value.clone())
                            }
                        }
                    }
                }
            }
        } else {
            println!("Cant call this type chain call");
        }
    }
    eval.state.temp.pop();
}

pub fn get_access(eval: &mut Evaluator) {
    // janky swap, TODO fix this
    if let (Some(top), Some(under)) = (
        eval.state.execution_stack.pop(),
        eval.state.execution_stack.pop(),
    ) {
        eval.state.execution_stack.push(top);
        eval.state.execution_stack.push(under)
    } else {
        print_error("Not enough arguments for access");
    }

    if let Some(token) = eval.state.get_from_heap_or_pop() {
        match token {
            Token::Block(Block::Function(block)) => {
                // Call with new scope
                eval.state.call_stack.push(HashMap::new());

                eval.evaluate(block.to_vec());

                if let Some(token) = eval.state.get_from_heap_or_pop() {
                    eval.state.execution_stack.push(token)
                }
                eval.state.call_stack.pop();
            }
            Token::Block(Block::Literal(block)) => {
                // call in same scope
                eval.evaluate(block.to_vec())
            }
            Token::Block(Block::List(list)) => {
                if let Some(key) = eval.state.get_from_heap_or_pop() {
                    if let Token::Integer(index) = key {
                        if let Some(value) = list.get(index as usize) {
                            eval.state.execution_stack.push(value.clone())
                        }
                    }
                }
            }
            Token::Block(Block::Struct(data)) => {
                if let Some(key) = eval.state.execution_stack.pop() {
                    if let Token::Identifier(key) = key {
                        if let Some(value) = data.get(&key) {
                            eval.state.execution_stack.push(value.clone())
                        }
                    }
                }
            }

            Token::String(word) => {
                if let Some(key) = eval.state.get_from_heap_or_pop() {
                    if let Token::Integer(index) = key {
                        if let Some(value) = word.chars().nth(index as usize) {
                            eval.state.execution_stack.push(Token::Char(value))
                        }
                    }
                }
            }
            Token::Identifier(function) => {
                if let Some(function) = eval.state.get_from_heap(&function) {}
            }
            _ => {
                println!("Cant access this type {:?}", token);
            }
        }
    }
}

// pub fn get_access(eval: &mut Evaluator) {
//     match (
//         eval.state.execution_stack.pop(),
//         eval.state.execution_stack.pop(),
//     ) {
//         (Some(Token::Identifier(ident)), Some(Token::Identifier(block))) => match ident.as_str() {
//             "len" => eval
//                 .state
//                 .execution_stack
//                 .push(Token::Integer(block.len() as i128)),
//             _ => match eval.state.get_from_heap(&block) {
//                 Some(Token::Block(blocktype)) => match blocktype {
//                     Block::Function(function) => {
//                         eval.state.call_stack.push(HashMap::new());

//                         eval.evaluate(function.to_vec());

//                         if let Some(tok) = eval.state.get_from_heap(&ident) {
//                             if let Token::Block(Block::Modifier(_, method)) = tok {
//                                 eval.state
//                                     .execution_stack
//                                     .push(Token::Block(Block::Modifier(Some(block), method)))
//                             } else {
//                                 eval.state.execution_stack.push(tok)
//                             }
//                         }

//                         eval.state.call_stack.pop();
//                     }
//                     Block::Auto(setup, _) => {
//                         eval.state.call_stack.push(HashMap::new());
//                         eval.evaluate(setup.to_vec());

//                         if let Some(tok) = eval.state.get_from_heap(&ident) {
//                             eval.state.execution_stack.push(tok)
//                         }

//                         eval.state.call_stack.pop();
//                     }
//                     Block::Modifier(_, _) => todo!(),
//                     Block::Literal(_) => todo!(),
//                     Block::List(_) => todo!(),

//                     Block::Lambda(_) => todo!(),
//                     Block::ListLambda(_) => todo!(),
//                     Block::Struct(_) => todo!(),
//                 },

//                 _ => {
//                     println!("{} does not exist, cannot access", block);
//                 }
//             },
//         },
//         (Some(Token::Identifier(ident)), Some(Token::Block(block))) => match block {
//             Block::Function(block) => {
//                 eval.state.call_stack.push(HashMap::new());

//                 for t in block.iter() {
//                     eval.eval(t.clone())
//                 }

//                 if let Some(tok) = eval.state.get_from_heap(&ident) {
//                     eval.state.execution_stack.push(tok)
//                 }

//                 eval.state.call_stack.pop();
//             }
//             Block::Auto(setup, _) => {
//                 eval.state.call_stack.push(HashMap::new());

//                 eval.evaluate(setup.to_vec());

//                 if let Some(tok) = eval.state.get_from_heap(&ident) {
//                     eval.state.execution_stack.push(tok)
//                 }

//                 eval.state.call_stack.pop();
//             }
//             Block::Modifier(_, _) => todo!(),
//             Block::Literal(_) => todo!(),
//             Block::List(_) => todo!(),

//             Block::Lambda(_) => todo!(),
//             Block::ListLambda(_) => todo!(),
//             Block::Struct(_) => todo!(),
//         },
//         _ => {
//             println!("Cant access this");
//         }
//     }
// }

pub fn store_temp(eval: &mut Evaluator) {
    if let Some(token) = eval.state.get_from_heap_or_pop() {
        eval.state.temp.push(token);
    }
}

pub fn eval_top(eval: &mut Evaluator) {
    if let Some(token) = eval.state.execution_stack.pop() {
        eval.eval(token)
    }
}
