use std::rc::Rc;

use hashbrown::HashMap;

use crate::novacore::{
    core::{Block, Instructions, Operator, Token},
    evaluator::Evaluator,
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
                Block::Procedure(block) => {
                    // call in same scope
                    eval.evaluate(block.to_vec())
                }
                Block::Parsed(block) => {
                    // call in same scope
                    eval.evaluate(block.to_vec())
                }
                Block::Raw(block) => {
                    // call in same scope
                    eval.evaluate(block.to_vec())
                }
                _ => {
                    todo!()
                }
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
                Block::Parsed(block) => eval.evaluate(block.to_vec()),
                Block::Procedure(block) => {
                    // call in same scope
                    eval.evaluate(block.to_vec())
                }
                Block::ParsedLambda(_) => todo!(),
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
                        if let Some(Token::Block(Block::Parsed(object_state))) =
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
                            //         Token::Block(Block::Parsed(Rc::new(core_self))),
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
                Block::List(block) => eval.evaluate(block.to_vec()),
                Block::Raw(block) => eval.evaluate(block.to_vec()),
                Block::RawLambda(_) => todo!(),
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
                match block {
                    Token::Block(Block::Parsed(block)) => eval.evaluate(block.to_vec()),
                    Token::Block(Block::Raw(block)) => eval.evaluate(block.to_vec()),
                    _ => todo!(),
                }
            }
        } else if let Some(Token::Bool(bool)) = eval.state.get_from_heap_or_pop() {
            if bool {
                match boolmaybe {
                    Token::Block(Block::Parsed(block)) => eval.evaluate(block.to_vec()),
                    Token::Block(Block::Raw(block)) => eval.evaluate(block.to_vec()),
                    _ => todo!(),
                }
            } else {
                match block {
                    Token::Block(Block::Parsed(block)) => eval.evaluate(block.to_vec()),
                    Token::Block(Block::Raw(block)) => eval.evaluate(block.to_vec()),
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
            (Token::Block(Block::Raw(test)), Token::Block(Block::Raw(logic))) => {
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
            (Token::Block(Block::Parsed(test)), Token::Block(Block::Raw(logic))) => {
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
            (Token::Block(Block::Parsed(test)), Token::Block(Block::Parsed(logic))) => {
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
            (Token::Block(Block::Raw(test)), Token::Block(Block::Parsed(logic))) => {
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
    if let (Some(logic), Some(times)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        match (times, logic) {
            (Token::Integer(times), Token::Block(logic)) => match logic {
                Block::Raw(logic) => {
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
                Block::Parsed(logic) => {
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
                Block::ParsedLambda(_) => todo!(),
                Block::RawLambda(_) => todo!(),
                Block::Procedure(logic) => {
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
                Block::Function(logic) => {
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
                Block::Auto(_, _) => todo!(),
                Block::Modifier(_, _) => todo!(),
                Block::List(_) => todo!(),
            },
            _ => {
                todo!()
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
                (Block::Raw(items), Block::Function(logic)) => each_compute(eval, items, logic),
                (Block::Raw(items), Block::Procedure(logic)) => each_compute(eval, items, logic),
                (Block::Raw(items), Block::Raw(logic)) => each_compute(eval, items, logic),
                (Block::Raw(items), Block::Parsed(logic)) => each_compute(eval, items, logic),
                (Block::Parsed(items), Block::Parsed(logic)) => each_compute(eval, items, logic),
                (Block::Parsed(items), Block::Procedure(logic)) => each_compute(eval, items, logic),
                (Block::Parsed(items), Block::Function(logic)) => each_compute(eval, items, logic),
                (Block::Parsed(items), Block::Raw(logic)) => each_compute(eval, items, logic),
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
                Token::Block(Block::Parsed(block)),
                Token::Block(Block::Raw(list)),
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
                Token::Block(Block::Raw(block)),
                Token::Block(Block::Raw(list)),
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
                Token::Block(Block::Parsed(block)),
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
                Block::Procedure(block) => {
                    // call in same scope
                    eval.evaluate(block.to_vec())
                }
                Block::Auto(_, _) => todo!(),
                Block::Modifier(object_name, method) => {
                    if let Some(object_name) = object_name {
                        if let Some(Token::Block(Block::Parsed(object_state))) =
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
                            //         Token::Block(Block::Parsed(Rc::new(core_self))),
                            //     );
                            // }
                        }
                    }
                }
                Block::Parsed(_) => todo!(),
                Block::List(_) => todo!(),
                Block::Raw(_) => todo!(),
                Block::ParsedLambda(_) => todo!(),
                Block::RawLambda(_) => todo!(),
            }
        } else {
            println!("Cant call this type");
        }
    }
    eval.state.temp.pop();
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
                    Block::Function(function) => {
                        eval.state.call_stack.push(HashMap::new());

                        eval.evaluate(function.to_vec());

                        if let Some(tok) = eval.state.get_from_heap(&ident) {
                            if let Token::Block(Block::Modifier(_, method)) = tok {
                                eval.state
                                    .execution_stack
                                    .push(Token::Block(Block::Modifier(Some(block), method)))
                            } else {
                                eval.state.execution_stack.push(tok)
                            }
                        }

                        eval.state.call_stack.pop();
                    }
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
                    Block::Modifier(_, _) => todo!(),
                    Block::Parsed(_) => todo!(),
                    Block::List(_) => todo!(),
                    Block::Raw(_) => todo!(),
                    Block::ParsedLambda(_) => todo!(),
                    Block::RawLambda(_) => todo!(),
                },

                _ => {
                    println!("{} does not exist, cannot access", block);
                }
            },
        },
        (Some(Token::Identifier(ident)), Some(Token::Block(block))) => match block {
            Block::Function(block) => {
                eval.state.call_stack.push(HashMap::new());

                for t in block.iter() {
                    eval.eval(t.clone())
                }

                if let Some(tok) = eval.state.get_from_heap(&ident) {
                    eval.state.execution_stack.push(tok)
                }

                eval.state.call_stack.pop();
            }
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
            Block::Modifier(_, _) => todo!(),
            Block::Parsed(_) => todo!(),
            Block::List(_) => todo!(),
            Block::Raw(_) => todo!(),
            Block::ParsedLambda(_) => todo!(),
            Block::RawLambda(_) => todo!(),
        },
        _ => {
            println!("Cant access this");
        }
    }
}

pub fn store_temp(eval: &mut Evaluator) {
    if let Some(token) = eval.state.get_from_heap_or_pop() {
        eval.state.temp.push(token);
    }
}
