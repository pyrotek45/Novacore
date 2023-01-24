use std::rc::Rc;

use hashbrown::HashMap;

use crate::novacore::{
    core::{Block, Instructions, Operator, Token},
    evaluator::Evaluator,
    new,
    utilities::print_error,
};

pub fn create_struct(eval: &mut Evaluator) {
    match eval.state.get_from_heap_or_pop() {
        Some(Token::Block(Block::Literal(block))) => {
            eval.state.call_stack.push(HashMap::new());

            eval.evaluate(block.to_vec());

            if let Some(new_struct) = eval.state.call_stack.pop() {
                eval.state
                    .execution_stack
                    .push(Token::Block(Block::Struct(new_struct)));
            }
        }
        _ => {
            todo!()
        }
    }
}
pub fn block(eval: &mut Evaluator) {
    match eval.state.get_from_heap_or_pop() {
        Some(Token::Block(Block::List(block))) => {
            eval.state
                .execution_stack
                .push(Token::Block(Block::Literal(block)));
        }
        _ => {
            todo!()
        }
    }
}

pub fn list(eval: &mut Evaluator) {
    match eval.state.get_from_heap_or_pop() {
        Some(Token::Block(Block::Literal(block))) => {
            eval.state
                .execution_stack
                .push(Token::Block(Block::List(block)));
        }
        _ => {
            todo!()
        }
    }
}

pub fn func(eval: &mut Evaluator) {
    match eval.state.get_from_heap_or_pop() {
        Some(Token::Block(Block::Literal(block))) => {
            eval.state
                .execution_stack
                .push(Token::Block(Block::Function(block)));
        }
        Some(Token::Block(Block::List(block))) => {
            eval.state
                .execution_stack
                .push(Token::Block(Block::Function(block)));
        }
        _ => {
            todo!()
        }
    }
}

// pub fn object(eval: &mut Evaluator) {
//     if let Some(Token::Block(Block::Parsed(block))) = eval.state.get_from_heap_or_pop() {
//         eval.state
//             .execution_stack
//             .push(Token::Block(Block::Object(block)));
//     }
// }

pub fn modifier(eval: &mut Evaluator) {
    match eval.state.get_from_heap_or_pop() {
        Some(Token::Block(Block::Literal(block))) => {
            eval.state
                .execution_stack
                .push(Token::Block(Block::Modifier(None, block)));
        }
        _ => {
            todo!()
        }
    }
}

pub fn closure_let(eval: &mut Evaluator) {
    match eval.state.get_from_heap_or_pop() {
        Some(Token::Block(Block::Literal(block))) => {
            if let Some(scope) = eval.state.call_stack.last_mut() {
                let mut core_self = vec![];

                for (ident, token) in scope {
                    core_self.push(Token::Identifier(ident.clone()));
                    core_self.push(token.clone());
                    core_self.push(Token::Op(Operator::VariableAssign))
                }

                for t in block.iter() {
                    core_self.push(t.clone())
                }

                eval.state
                    .execution_stack
                    .push(Token::Block(Block::Function(Rc::new(core_self))))
            }
        }
        _ => {
            todo!()
        }
    }
}

pub fn closure_rec(eval: &mut Evaluator) {
    match (
        eval.state.get_from_heap_or_pop(),
        eval.state.execution_stack.pop(),
    ) {
        (Some(Token::Block(Block::Literal(block))), Some(Token::Identifier(ident))) => {
            if let Some(function_index) = eval.state.current_function_index.last() {
                let mut core_self = vec![
                    Token::Identifier(ident.clone()),
                    Token::Identifier(ident),
                    Token::Block(Block::Literal(block.clone())),
                    Token::Function(*function_index),
                    Token::Op(Operator::VariableAssign),
                ];
                for t in block.iter() {
                    core_self.push(t.clone());
                }

                eval.state
                    .execution_stack
                    .push(Token::Block(Block::Function(Rc::new(core_self))))
            }
        }
        _ => {
            todo!()
        }
    }
}

pub fn closure_auto(eval: &mut Evaluator) {
    match (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        (Some(Token::Block(Block::Literal(logic))), Some(Token::Block(Block::Literal(setup)))) => {
            eval.state.execution_stack.push(Token::Block(Block::Auto(
                Rc::new(setup.to_vec()),
                Rc::new(logic.to_vec()),
            )))
        }
        _ => {
            todo!()
        }
    }
}

pub fn include(eval: &mut Evaluator) {
    fn into(eval: &mut Evaluator, block: Instructions, list: Instructions) {
        let mut newlist = vec![];
        if let Some(scope) = eval.state.call_stack.last_mut() {
            for item in list.iter() {
                match item {
                    Token::Identifier(ident) => {
                        if let Some(token) = scope.get(ident) {
                            newlist.push(Token::Identifier(ident.clone()));
                            newlist.push(token.clone());
                            newlist.push(Token::Op(Operator::VariableAssign))
                        }
                    }
                    _ => {}
                }
            }

            for t in block.iter() {
                newlist.push(t.clone())
            }

            eval.state
                .execution_stack
                .push(Token::Block(Block::Function(Rc::new(newlist))))
        }
    }

    match (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        (Some(Token::Block(Block::Literal(block))), Some(Token::Block(Block::List(list)))) => {
            into(eval, block, list)
        }
        (Some(Token::Block(Block::Function(block))), Some(Token::Block(Block::List(list)))) => {
            into(eval, block, list)
        }
        _ => {
            todo!()
        }
    }
}
