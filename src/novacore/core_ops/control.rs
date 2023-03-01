use hashbrown::HashMap;

use crate::novacore::{
    core::{Block, Instructions, Token},
    evaluator::Evaluator,
};

pub fn break_loop(eval: &mut Evaluator) {
    eval.state.break_loop.push(true);
}

pub fn continue_loop(eval: &mut Evaluator) {
    eval.state.continue_loop.push(true);
}

pub fn block_call(eval: &mut Evaluator) {
    if let Some(token) = eval.state.get_from_heap_or_pop() {
        if let Token::Block(block) = token {
            match block {
                Block::Function(idlist, block) => {
                    let mut variable_stack: Vec<String> = Vec::with_capacity(10);

                    for toks in idlist.iter().rev() {
                        if let Token::Id(ident) = &toks {
                            variable_stack.push(ident.clone())
                        } else {
                            eval.state
                                .show_error("Can only bind identifiers in a function")
                        }
                    }

                    // Tie each Token into the call_stack using the tokens poped
                    let mut newscope = HashMap::new();
                    for tokens in variable_stack {
                        if let Some(tok) = eval.state.get_from_heap_or_pop() {
                            newscope.insert(tokens, tok.clone());
                        } else {
                            eval.state.show_error("Not enough arguments")
                        }
                    }
                    eval.state.call_stack.push(newscope);
                    eval.evaluate(block);
                    eval.state.call_stack.pop();
                }
                Block::Literal(block) => eval.evaluate(block),
                Block::List(list) => {
                    if let Some(Token::Integer(index)) = eval.state.get_from_heap_or_pop() {
                        if let Some(value) = list.get(index as usize) {
                            eval.state.execution_stack.push(value.clone())
                        } else {
                            eval.state.show_error("Index out of Bounds");
                        }
                    } else {
                        eval.state.show_error("Incorrect arguments for list")
                    }
                }
                Block::Struct(data) => {
                    if let Some(Token::Id(key)) = eval.state.execution_stack.pop() {
                        if let Some(value) = data.get(&key) {
                            eval.state.execution_stack.push(value.clone())
                        } else {
                            eval.state
                                .show_error(&format!("Key does not exist [{}]", &key))
                        }
                    } else {
                        eval.state.show_error("Incorrect arguments for struct")
                    }
                }
                _ => {
                    eval.state
                        .show_error(&format!("Cant call this type [{:?}]", block));
                }
            }
        } else {
            eval.state
                .show_error(&format!("Cant call this type [{:?}]", token));
        }
    } else {
        eval.state.show_error("Not enough arguments for call");
    }
}

pub fn user_block_call(eval: &mut Evaluator, function_name: &str) {
    if let Some(token) = eval.state.get_from_heap(function_name) {
        if let Token::Block(block) = token {
            match block {
                Block::Literal(block) => eval.evaluate(block),
                Block::Function(idlist, block) => {
                    let mut variable_stack: Vec<String> = Vec::with_capacity(10);

                    for toks in idlist.iter().rev() {
                        if let Token::Id(ident) = &toks {
                            variable_stack.push(ident.clone())
                        } else {
                            eval.state
                                .show_error("Can only bind identifiers in a function")
                        }
                    }

                    // Tie each Token into the call_stack using the tokens poped
                    let mut newscope = HashMap::new();
                    for tokens in variable_stack {
                        if let Some(tok) = eval.state.get_from_heap_or_pop() {
                            newscope.insert(tokens, tok.clone());
                        } else {
                            eval.state.show_error("Not enough arguments")
                        }
                    }
                    eval.state.call_stack.push(newscope);
                    eval.evaluate(block);
                    eval.state.call_stack.pop();
                }
                Block::List(list) => {
                    if let Some(Token::Integer(index)) = eval.state.get_from_heap_or_pop() {
                        if let Some(value) = list.get(index as usize) {
                            eval.state.execution_stack.push(value.clone())
                        } else {
                            eval.state.show_error("Index out of Bounds");
                        }
                    } else {
                        eval.state.show_error("Incorrect arguments for list")
                    }
                }
                Block::Lambda(_) => todo!(),
                Block::Struct(data) => {
                    if let Some(Token::Id(key)) = eval.state.execution_stack.pop() {
                        if let Some(value) = data.get(&key) {
                            eval.state.execution_stack.push(value.clone())
                        } else {
                            eval.state
                                .show_error(&format!("Key does not exist [{}]", &key))
                        }
                    } else {
                        eval.state.show_error("Incorrect arguments for struct")
                    }
                }
            }
        } else {
            eval.state
                .show_error(&format!("Cant call this type [{:?}]", token));
        }
    } else {
        eval.state.show_error("Unknown identifier");
    }
}

pub fn if_statement(eval: &mut Evaluator) {
    if let (Some(elseblock), Some(trueblock), Some(bool)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        match (bool, trueblock, elseblock) {
            (
                Token::Bool(bool),
                Token::Block(Block::Literal(trueblock)),
                Token::Block(Block::Literal(elseblock)),
            ) => {
                if bool {
                    eval.evaluate(trueblock)
                } else {
                    eval.evaluate(elseblock)
                }
            }
            (a, b, c) => eval.state.show_error(&format!(
                "Incorrect arguments for if, got [{:?},{:?},{:?}]",
                a, b, c
            )),
        }
    } else {
        eval.state.show_error("Not enough arguments for if");
    }
}

pub fn con_statement(eval: &mut Evaluator) {
    if let (Some(trueblock), Some(bool)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        match (bool, trueblock) {
            (Token::Bool(bool), Token::Block(Block::Literal(trueblock))) => {
                if bool {
                    eval.evaluate(trueblock);
                    eval.state.execution_stack.push(Token::Bool(true));
                } else {
                    eval.state.execution_stack.push(Token::Bool(false));
                }
            }
            (a, b) => eval.state.show_error(&format!(
                "Incorrect arguments for con, got [{:?},{:?}]",
                a, b
            )),
        }
    } else {
        eval.state.show_error("Not enough arguments for con");
    }
}

pub fn when_statement(eval: &mut Evaluator) {
    if let (Some(trueblock), Some(bool)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        match (bool, trueblock) {
            (Token::Bool(bool), Token::Block(Block::Literal(trueblock))) => {
                if bool {
                    eval.evaluate(trueblock)
                }
            }
            (a, b) => eval.state.show_error(&format!(
                "Incorrect arguments for when, got [{:?},{:?}]",
                a, b
            )),
        }
    } else {
        eval.state.show_error("Not enough arguments for when");
    }
}

pub fn unless_statement(eval: &mut Evaluator) {
    if let (Some(trueblock), Some(bool)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        match (bool, trueblock) {
            (Token::Bool(bool), Token::Block(Block::Literal(trueblock))) => {
                if !bool {
                    eval.evaluate(trueblock)
                }
            }
            (a, b) => eval.state.show_error(&format!(
                "Incorrect arguments for unless, got [{:?},{:?}]",
                a, b
            )),
        }
    } else {
        eval.state.show_error("Not enough arguments for unless");
    }
}

pub fn while_loop(eval: &mut Evaluator) {
    fn while_compute(eval: &mut Evaluator, test: Instructions, logic: Instructions) {
        'out: loop {
            // run test block
            eval.evaluate(test.clone());

            // get result and run logic block if true is on stack else break
            if let Some(Token::Bool(bool)) = eval.state.get_from_heap_or_pop() {
                if bool {
                    for t in &*logic {
                        eval.eval(t.clone());
                        if !eval.state.break_loop.is_empty() {
                            eval.state.break_loop.pop();
                            break 'out;
                        }
                        if !eval.state.continue_loop.is_empty() {
                            eval.state.continue_loop.pop();
                            continue 'out;
                        }
                    }
                } else {
                    break;
                }
            }
        }
    }

    if let (Some(logic), Some(testing)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        match (testing, logic) {
            (Token::Block(Block::Literal(test)), Token::Block(Block::Literal(logic))) => {
                while_compute(eval, test, logic)
            }
            (Token::Block(Block::Literal(test)), Token::Block(Block::List(logic))) => {
                while_compute(eval, test, logic)
            }
            (Token::Block(Block::List(test)), Token::Block(Block::Literal(logic))) => {
                while_compute(eval, test, logic)
            }
            (Token::Block(Block::List(test)), Token::Block(Block::List(logic))) => {
                while_compute(eval, test, logic)
            }
            (testing, logic) => eval.state.show_error(&format!(
                "Incorrect arguments for while, got [{:?},{:?}]",
                testing, logic
            )),
        }
    } else {
        eval.state.show_error("Not enough arguments for while");
    }
}

pub fn times(eval: &mut Evaluator) {
    fn times_compute(eval: &mut Evaluator, logic: Instructions, times: usize) {
        'out: for _ in 0..times {
            for t in &*logic {
                eval.eval(t.clone());
                if !eval.state.break_loop.is_empty() {
                    eval.state.break_loop.pop();
                    break 'out;
                }
                if !eval.state.continue_loop.is_empty() {
                    eval.state.continue_loop.pop();
                    continue 'out;
                }
            }
        }
    }

    if let (Some(logic), Some(times)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        match (logic, times) {
            (Token::Block(logic), Token::Integer(times)) => match logic {
                Block::Literal(logic) => times_compute(eval, logic, times as usize),
                Block::Function(_, logic) => {
                    eval.state.call_stack.push(HashMap::new());
                    times_compute(eval, logic, times as usize);
                    eval.state.call_stack.pop();
                }
                Block::List(logic) => times_compute(eval, logic, times as usize),
                _ => eval.state.show_error(&format!(
                    "Incorrect arguments for times, got [{:?},{:?}]",
                    logic, times
                )),
            },
            (logic, times) => eval.state.show_error(&format!(
                "Incorrect arguments for times, got [{:?},{:?}]",
                logic, times
            )),
        }
    } else {
        eval.state.show_error("Not enough arguments for times");
    }
}

pub fn each(eval: &mut Evaluator) {
    if let (Some(logic), Some(items)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        fn each_compute(eval: &mut Evaluator, items: Instructions, logic: Instructions) {
            'out: for item in items.iter() {
                eval.state.execution_stack.push(item.clone());
                for t in &*logic {
                    eval.eval(t.clone());
                    if !eval.state.break_loop.is_empty() {
                        eval.state.break_loop.pop();
                        break 'out;
                    }
                    if !eval.state.continue_loop.is_empty() {
                        eval.state.continue_loop.pop();
                        continue 'out;
                    }
                }
            }
        }

        fn each_compute_string(eval: &mut Evaluator, str: String, logic: Instructions) {
            'out: for item in str.chars() {
                eval.state.execution_stack.push(Token::Char(item));
                for t in &*logic {
                    eval.eval(t.clone());
                    if !eval.state.break_loop.is_empty() {
                        eval.state.break_loop.pop();
                        break 'out;
                    }
                    if !eval.state.continue_loop.is_empty() {
                        eval.state.continue_loop.pop();
                        continue 'out;
                    }
                }
            }
        }

        match (&items, logic) {
            (Token::Block(items), Token::Block(logic)) => match (items, logic) {
                (Block::List(items), Block::Literal(logic)) => {
                    each_compute(eval, items.clone(), logic)
                }
                (items, logic) => eval.state.show_error(&format!(
                    "Incorrect arguments for each, got [{:?},{:?}]",
                    items, logic
                )),
            },
            (Token::String(str), Token::Block(logic)) => match logic {
                Block::Literal(logic) => each_compute_string(eval, str.to_string(), logic),
                logic => eval.state.show_error(&format!(
                    "Incorrect arguments for each, got [{:?},{:?}]",
                    items, logic
                )),
            },
            (items, logic) => eval.state.show_error(&format!(
                "Incorrect arguments for each, got [{:?},{:?}]",
                items, logic
            )),
        }
    } else {
        eval.state.show_error("Not enough arguments for each");
    }
}

pub fn for_each(eval: &mut Evaluator) {
    fn for_compute(
        eval: &mut Evaluator,
        block: Instructions,
        list: Instructions,
        variable_name: String,
    ) {
        'out: for variable in list.iter() {
            match &variable {
                Token::Id(inner_ident) => {
                    if let Some(token) = eval.state.get_from_heap(inner_ident) {
                        eval.state.add_varaible(&variable_name, token.clone());
                        for t in &*block {
                            eval.eval(t.clone());
                            if !eval.state.break_loop.is_empty() {
                                eval.state.break_loop.pop();
                                eval.state.remove_varaible(&variable_name);
                                break 'out;
                            }
                            if !eval.state.continue_loop.is_empty() {
                                eval.state.continue_loop.pop();
                                eval.state.remove_varaible(&variable_name);
                                continue 'out;
                            }
                        }
                        eval.state.remove_varaible(&variable_name);
                    }
                }
                _ => {
                    eval.state.add_varaible(&variable_name, variable.clone());
                    for t in &*block {
                        eval.eval(t.clone());
                        if !eval.state.break_loop.is_empty() {
                            eval.state.break_loop.pop();
                            eval.state.remove_varaible(&variable_name);
                            break 'out;
                        }
                        if !eval.state.continue_loop.is_empty() {
                            eval.state.continue_loop.pop();
                            eval.state.remove_varaible(&variable_name);
                            continue 'out;
                        }
                        if eval.state.exit {
                            continue 'out;
                        }
                    }
                    eval.state.remove_varaible(&variable_name);
                }
            }
        }
    }

    fn for_compute_string(
        eval: &mut Evaluator,
        block: Instructions,
        str: String,
        variable_name: String,
    ) {
        'out: for variable in str.chars() {
            eval.state
                .add_varaible(&variable_name, Token::Char(variable));
            for t in &*block {
                eval.eval(t.clone());
                if !eval.state.break_loop.is_empty() {
                    eval.state.break_loop.pop();
                    eval.state.remove_varaible(&variable_name);
                    break 'out;
                }
                if !eval.state.continue_loop.is_empty() {
                    eval.state.continue_loop.pop();
                    eval.state.remove_varaible(&variable_name);
                    continue 'out;
                }
                if eval.state.exit {
                    continue 'out;
                }
            }
            eval.state.remove_varaible(&variable_name);
        }
    }

    if let (Some(block), Some(list), Some(variable)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
        eval.state.execution_stack.pop(),
    ) {
        match (block, list, variable) {
            (
                Token::Block(Block::Literal(block)),
                Token::Block(Block::List(list)),
                Token::Id(variable_name),
            ) => for_compute(eval, block, list, variable_name),
            (
                Token::Block(Block::Literal(block)),
                Token::String(list),
                Token::Id(variable_name),
            ) => for_compute_string(eval, block, list, variable_name),
            (a, b, c) => eval.state.show_error(&format!(
                "Incorrect arguments for [for], got [{:?},{:?},{:?}]",
                a, b, c
            )),
        }
    } else {
        eval.state.show_error("Not enough arguments for [for]");
    }
}

pub fn user_chain_call(eval: &mut Evaluator) {
    if let Some(token) = eval.state.auxiliary.last().cloned() {
        if let Token::Block(block) = token {
            match block {
                Block::Literal(block) => eval.evaluate(block),
                Block::Function(idlist, block) => {
                    let mut variable_stack: Vec<String> = Vec::with_capacity(10);

                    for toks in idlist.iter().rev() {
                        if let Token::Id(ident) = &toks {
                            variable_stack.push(ident.clone())
                        } else {
                            eval.state
                                .show_error("Can only bind identifiers in a function")
                        }
                    }

                    // Tie each Token into the call_stack using the tokens poped
                    let mut newscope = HashMap::new();
                    for tokens in variable_stack {
                        if let Some(tok) = eval.state.get_from_heap_or_pop() {
                            newscope.insert(tokens, tok.clone());
                        } else {
                            eval.state.show_error("Not enough arguments")
                        }
                    }
                    eval.state.call_stack.push(newscope);
                    eval.evaluate(block);
                    eval.state.call_stack.pop();
                }
                Block::List(list) => {
                    if let Some(Token::Integer(index)) = eval.state.get_from_heap_or_pop() {
                        if let Some(value) = list.get(index as usize) {
                            eval.state.execution_stack.push(value.clone())
                        } else {
                            eval.state.show_error("Index out of Bounds");
                        }
                    } else {
                        eval.state.show_error("Incorrect arguments for list")
                    }
                }
                Block::Lambda(_) => todo!(),
                Block::Struct(data) => {
                    if let Some(Token::Id(key)) = eval.state.execution_stack.pop() {
                        if let Some(value) = data.get(&key) {
                            eval.state.execution_stack.push(value.clone())
                        } else {
                            eval.state
                                .show_error(&format!("Key does not exist [{}]", &key))
                        }
                    } else {
                        eval.state.show_error("Incorrect arguments for struct")
                    }
                }
            }
        } else {
            eval.state.show_error(&format!(
                "Incorrect type for chain_call, got [{:?}]",
                eval.state.auxiliary.last()
            ));
        }
    } else {
        eval.state.show_error("Not enough arguments for chain_call");
    }
    eval.state.auxiliary.pop();
}

pub fn get_access(eval: &mut Evaluator) {
    if let (Some(top), Some(under)) = (
        eval.state.execution_stack.pop(),
        eval.state.execution_stack.pop(),
    ) {
        eval.state.execution_stack.push(top);
        eval.state.execution_stack.push(under)
    } else {
        eval.state.show_error("Not enough arguments for access...");
    }

    if let Some(Token::Block(token)) = eval.state.get_from_heap_or_pop() {
        match token {
            Block::Function(idlist, block) => {
                if let Some(Token::Id(content)) = eval.state.execution_stack.pop() {
                    match content.as_str() {
                        "logic" => eval
                            .state
                            .execution_stack
                            .push(Token::Block(Block::Literal(block))),
                        "input" => eval
                            .state
                            .execution_stack
                            .push(Token::Block(Block::List(idlist))),
                        _ => {
                            eval.state.show_error(
                                "Incorrect argument for function access, expected [input | logic]",
                            );
                        }
                    }
                } else {
                    eval.state
                        .show_error("Incorrect argument for function access, expected an id");
                }
            }
            Block::Literal(block) => eval.evaluate(block),
            Block::List(list) => {
                if let Some(Token::Integer(index)) = eval.state.get_from_heap_or_pop() {
                    if let Some(value) = list.get(index as usize) {
                        eval.state.execution_stack.push(value.clone())
                    } else {
                        eval.state.show_error("Index out of Bounds");
                    }
                } else {
                    eval.state.show_error("Incorrect arguments for list")
                }
            }
            Block::Struct(data) => {
                if let Some(Token::Id(key)) = eval.state.execution_stack.pop() {
                    if let Some(value) = data.get(&key) {
                        eval.state.execution_stack.push(value.clone())
                    } else {
                        eval.state
                            .show_error(&format!("Key does not exist [{}]", &key))
                    }
                } else {
                    eval.state.show_error("Incorrect arguments for struct")
                }
            }
            _ => {
                eval.state
                    .show_error(&format!("Cant call this type [{:?}]", token));
            }
        }
    } else {
        eval.state.show_error("Not enough arguments for access");
    }
}

pub fn module(eval: &mut Evaluator) {
    if let (Some(Token::Id(key)), Some(Token::Id(module))) = (
        eval.state.execution_stack.pop(),
        eval.state.execution_stack.pop(),
    ) {
        if let Some(table) = eval.state.modules.get(&module) {
            if let Some(token) = table.get(&key) {
                eval.state.execution_stack.push(token.clone())
            } else {
                eval.state
                    .show_error(&format!("{} is not located in {} ", key, module));
            }
        } else {
            eval.state
                .show_error(&format!("{} is not a module", module));
        }
    } else {
        eval.state.show_error("Not enough arguments for module");
    }
}

pub fn store_temp(eval: &mut Evaluator) {
    if let Some(token) = eval.state.get_from_heap_or_pop() {
        eval.state.auxiliary.push(token);
    } else {
        eval.state.show_error("Not enough arguments for store_temp");
    }
}

pub fn eval_top(eval: &mut Evaluator) {
    if let Some(token) = eval.state.get_from_heap_or_pop() {
        eval.eval(token)
    } else {
        eval.state.show_error("Not enough arguments for eval");
    }
}

pub fn exe(eval: &mut Evaluator) {
    if let Some(token) = eval.state.get_from_heap_or_pop() {
        match token {
            Token::Block(block) => match block {
                Block::Function(_, block) => {
                    eval.evaluate_function(block);
                }
                Block::Literal(block) => eval.evaluate(block),
                Block::List(list) => {
                    if let Some(Token::Integer(index)) = eval.state.get_from_heap_or_pop() {
                        if let Some(value) = list.get(index as usize) {
                            eval.state.execution_stack.push(value.clone())
                        } else {
                            eval.state.show_error("Index out of Bounds");
                        }
                    } else {
                        eval.state.show_error("Incorrect arguments for list")
                    }
                }
                Block::Struct(data) => {
                    if let Some(Token::Id(key)) = eval.state.get_from_heap_or_pop() {
                        if let Some(value) = data.get(&key) {
                            eval.state.execution_stack.push(value.clone())
                        } else {
                            eval.state
                                .show_error(&format!("Key does not exist [{}]", &key))
                        }
                    } else {
                        eval.state.show_error("Incorrect arguments for struct")
                    }
                }
                _ => {
                    eval.state
                        .show_error(&format!("Cant call this type [{:?}]", block));
                }
            },
            _ => eval.eval(token),
        }
    } else {
        eval.state.show_error("Not enough arguments for exe");
    }
}
