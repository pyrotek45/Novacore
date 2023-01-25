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
                    eval.evaluate_function(block.to_vec());
                }
                Block::Literal(block) => eval.evaluate(block.to_vec()),
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
                    if let Some(Token::Identifier(key)) = eval.state.execution_stack.pop() {
                        if let Some(value) = data.get(&key) {
                            eval.state.execution_stack.push(value.clone())
                        } else {
                            eval.state.show_error(&format!("Key does not exist [{}]", &key))
                        }
                    } else {
                        eval.state.show_error("Incorrect arguments for struct")
                    }
                }
                _ => {
                    eval.state.show_error(&format!("Cant call this type [{:?}]", block));
                }
            }
        } else {
            eval.state.show_error(&format!("Cant call this type [{:?}]", token));
        }
    } else {
        eval.state.show_error("Not enough arguments for call");
    }
}

pub fn user_block_call(eval: &mut Evaluator, function_name: &str) {
    if let Some(token) = eval.state.get_from_heap(function_name) {
        if let Token::Block(block) = token {
            match block {
                Block::Literal(block) => eval.evaluate(block.to_vec()),
                Block::Auto(setup, logic) => {
                    eval.state.call_stack.push(HashMap::new());

                    eval.evaluate(setup.to_vec());
                    eval.evaluate(logic.to_vec());

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
                        }
                    }
                }
                Block::Function(block) => {
                    eval.evaluate_function(block.to_vec());
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
                Block::ListLambda(_) => todo!(),
                Block::Struct(data) => {
                    if let Some(Token::Identifier(key)) = eval.state.execution_stack.pop() {
                        if let Some(value) = data.get(&key) {
                            eval.state.execution_stack.push(value.clone())
                        } else {
                            eval.state.show_error(&format!("Key does not exist [{}]", &key))
                        }
                    } else {
                        eval.state.show_error("Incorrect arguments for struct")
                    }
                }
            }
        } else {
            eval.state.show_error(&format!("Cant call this type [{:?}]", token));
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
                    eval.evaluate(trueblock.to_vec())
                } else {
                    eval.evaluate(elseblock.to_vec())
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

pub fn when_statement(eval: &mut Evaluator) {
    if let (Some(trueblock), Some(bool)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        match (bool, trueblock) {
            (Token::Bool(bool), Token::Block(Block::Literal(trueblock))) => {
                if bool {
                    eval.evaluate(trueblock.to_vec())
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
                    eval.evaluate(trueblock.to_vec())
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
        loop {
            // run test block
            eval.evaluate(test.to_vec());

            // get result and run logic block if true is on stack else break
            if let Some(Token::Bool(bool)) = eval.state.get_from_heap_or_pop() {
                if bool {
                    eval.evaluate(logic.to_vec())
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
        for _ in 0..times {
            eval.evaluate(logic.to_vec())
        }
    }

    if let (Some(logic), Some(times)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        match (logic, times) {
            (Token::Block(logic), Token::Integer(times)) => match logic {
                Block::Literal(logic) => times_compute(eval, logic, times as usize),
                Block::Function(logic) => {
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
            for item in items.iter() {
                eval.state.execution_stack.push(item.clone());
                for t in logic.iter() {
                    eval.eval(t.clone());
                }
            }
        }
        match (items, logic) {
            (Token::Block(items), Token::Block(logic)) => match (items, logic) {
                (Block::Literal(items), Block::Literal(logic)) => each_compute(eval, items, logic),
                (Block::List(items), Block::Literal(logic)) => each_compute(eval, items, logic),
                (Block::Literal(items), Block::Function(logic)) => {
                    eval.state.call_stack.push(HashMap::new());
                    each_compute(eval, items, logic);

                    eval.state.call_stack.pop();
                }
                (Block::List(items), Block::Function(logic)) => each_compute(eval, items, logic),
                (items, logic) => eval.state.show_error(&format!(
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

// pub fn break_loop(eval: &mut Evaluator) {
//     eval.state.break_loop.push(true);
// }

// pub fn for_loop(eval: &mut Evaluator) {
//     fn for_compute(
//         eval: &mut Evaluator,
//         block: Instructions,
//         list: Instructions,
//         variable_name: String,
//     ) {
//         'outer1: for variable in list.iter() {
//             match &variable {
//                 Token::Identifier(inner_ident) => {
//                     if let Some(token) = eval.state.get_from_heap(inner_ident) {
//                         if variable_name != "_" {
//                             if let Some(scope) = eval.state.call_stack.last_mut() {
//                                 scope.insert(variable_name.to_string(), token.clone());
//                             }
//                         }
//                         for t in block.iter() {
//                             eval.eval(t.clone());
//                             if eval.state.break_loop.pop().is_some() {
//                                 if let Some(scope) = eval.state.call_stack.last_mut() {
//                                     scope.remove(&variable_name);
//                                 }
//                                 break 'outer1;
//                             }

//                             if eval.state.continue_loop.pop().is_some() {
//                                 eval.state.continue_loop.pop();
//                                 if let Some(scope) = eval.state.call_stack.last_mut() {
//                                     scope.remove(&variable_name);
//                                 }
//                                 continue 'outer1;
//                             }
//                         }

//                         if let Some(scope) = eval.state.call_stack.last_mut() {
//                             scope.remove(&variable_name);
//                         }
//                         eval.state.break_loop.pop();
//                     }
//                 }
//                 _ => {
//                     if variable_name != "_" {
//                         if let Some(scope) = eval.state.call_stack.last_mut() {
//                             scope.insert(variable_name.to_string(), variable.clone());
//                         }
//                     }
//                     for t in block.iter() {
//                         eval.eval(t.clone());
//                         if eval.state.break_loop.pop().is_some() {
//                             if let Some(scope) = eval.state.call_stack.last_mut() {
//                                 scope.remove(&variable_name);
//                             }
//                             break 'outer1;
//                         }

//                         if eval.state.continue_loop.pop().is_some() {
//                             eval.state.continue_loop.pop();
//                             if let Some(scope) = eval.state.call_stack.last_mut() {
//                                 scope.remove(&variable_name);
//                             }
//                             continue 'outer1;
//                         }
//                     }

//                     if let Some(scope) = eval.state.call_stack.last_mut() {
//                         scope.remove(&variable_name);
//                     }
//                     eval.state.break_loop.pop();
//                 }
//             }
//         }
//     }

//     if let (Some(block), Some(list), Some(variable)) = (
//         eval.state.get_from_heap_or_pop(),
//         eval.state.get_from_heap_or_pop(),
//         eval.state.execution_stack.pop(),
//     ) {
//         match (block, list, variable) {
//             (
//                 Token::Block(Block::Literal(block)),
//                 Token::Block(Block::List(list)),
//                 Token::Identifier(variable_name),
//             ) => for_compute(eval, block, list, variable_name),
//             (
//                 Token::Block(Block::Literal(block)),
//                 Token::Block(Block::Literal(list)),
//                 Token::Identifier(variable_name),
//             ) => for_compute(eval, block, list, variable_name),
//             (
//                 Token::Block(Block::Literal(block)),
//                 Token::Bool(bool),
//                 Token::Identifier(variable_name),
//             ) => {
//                 if bool {
//                     'outer: loop {
//                         for t in block.iter() {
//                             eval.eval(t.clone());
//                             if eval.state.break_loop.pop().is_some() {
//                                 if let Some(scope) = eval.state.call_stack.last_mut() {
//                                     scope.remove(&variable_name);
//                                 }
//                                 break 'outer;
//                             }

//                             if eval.state.continue_loop.pop().is_some() {
//                                 eval.state.continue_loop.pop();
//                                 if let Some(scope) = eval.state.call_stack.last_mut() {
//                                     scope.remove(&variable_name);
//                                 }
//                                 continue 'outer;
//                             }
//                         }
//                         if let Some(scope) = eval.state.call_stack.last_mut() {
//                             scope.remove(&variable_name);
//                         }
//                         eval.state.break_loop.pop();
//                     }
//                 }
//             }
//             (a, b, c) => eval.state.show_error(&format!(
//                 "Incorrect arguments for iteration[for], got [{:?},{:?},{:?}]",
//                 a, b, c
//             )),
//         }
//     } else {
//         eval.state.show_error("Not enough arguments for iteration[for]");
//     }
// }

pub fn user_chain_call(eval: &mut Evaluator) {
    if let Some(token) = eval.state.auxiliary.last().cloned() {
        if let Token::Block(block) = token {
            match block {
                Block::Function(block) => {
                    eval.state.call_stack.push(HashMap::new());
                    eval.evaluate(block.to_vec());

                    eval.state.call_stack.pop();
                }
                Block::Modifier(object_name, method) => {
                    if let Some(object_name) = object_name {
                        if let Some(Token::Block(Block::Literal(object_state))) =
                            eval.state.get_from_heap(&object_name)
                        {
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
                        }
                    }
                }
                Block::List(list) => {
                    if let Some(Token::Integer(index)) = eval.state.get_from_heap_or_pop() {
                        if let Some(value) = list.get(index as usize) {
                            eval.state.execution_stack.push(value.clone())
                        }
                    }
                }
                Block::Struct(data) => {
                    if let Some(Token::Identifier(key)) = eval.state.execution_stack.pop() {
                        if let Some(value) = data.get(&key) {
                            eval.state.execution_stack.push(value.clone())
                        }
                    }
                }
                _ => eval.state.show_error(&format!(
                    "Incorrect arguments for chain_call, got [{:?}]",
                    block
                )),
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
        eval.state.show_error("Not enough arguments for access");
    }

    if let Some(token) = eval.state.get_from_heap_or_pop() {
        match token {
            Token::Block(Block::Function(block)) => {
                eval.evaluate_function(block.to_vec());
            }
            Token::Block(Block::Literal(block)) => eval.evaluate(block.to_vec()),
            Token::Block(Block::List(list)) => {
                if let Some(Token::Integer(index)) = eval.state.get_from_heap_or_pop() {
                    if let Some(value) = list.get(index as usize) {
                        eval.state.execution_stack.push(value.clone())
                    }
                }
            }
            Token::Block(Block::Struct(data)) => {
                if let Some(Token::Identifier(key)) = eval.state.execution_stack.pop() {
                    if let Some(value) = data.get(&key) {
                        eval.state.execution_stack.push(value.clone())
                    }
                }
            }
            Token::String(word) => {
                if let Some(Token::Integer(index)) = eval.state.get_from_heap_or_pop() {
                    if let Some(value) = word.chars().nth(index as usize) {
                        eval.state.execution_stack.push(Token::Char(value))
                    }
                }
            }
            token => {
                eval.state.show_error(&format!(
                    "Incorrect arguments for access, got [{:?}]",
                    token
                ));
            }
        }
    } else {
        eval.state.show_error("Not enough arguments for access");
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
    if let Some(token) = eval.state.execution_stack.pop() {
        eval.eval(token)
    } else {
        eval.state.show_error("Not enough arguments for eval");
    }
}
