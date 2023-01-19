use std::rc::Rc;

use crate::novacore::{
    core::{Block, Operator, Token},
    evaluator::Evaluator,
};

pub fn proc(eval: &mut Evaluator) {
    match eval.state.get_from_heap_or_pop() {
        Some(Token::Block(Block::Parsed(block))) => {
            eval.state
                .execution_stack
                .push(Token::Block(Block::Procedure(block)));
        }
        Some(Token::Block(Block::Raw(block))) => {
            eval.state
                .execution_stack
                .push(Token::Block(Block::Procedure(block)));
        }
        _ => {
            todo!()
        }
    }
}

pub fn list(eval: &mut Evaluator) {
    match eval.state.get_from_heap_or_pop() {
        Some(Token::Block(Block::Parsed(block))) => {
            eval.state
                .execution_stack
                .push(Token::Block(Block::List(block)));
        }
        Some(Token::Block(Block::Raw(block))) => {
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
        Some(Token::Block(Block::Parsed(block))) => {
            eval.state
                .execution_stack
                .push(Token::Block(Block::Function(block)));
        }
        Some(Token::Block(Block::Raw(block))) => {
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
        Some(Token::Block(Block::Parsed(block))) => {
            eval.state
                .execution_stack
                .push(Token::Block(Block::Modifier(None, block)));
        }
        Some(Token::Block(Block::Raw(block))) => {
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
        Some(Token::Block(Block::Parsed(block))) => {
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
        Some(Token::Block(Block::Raw(block))) => {
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
        (Some(Token::Block(Block::Parsed(block))), Some(Token::Identifier(ident))) => {
            if let Some(function_index) = eval.state.current_function_index.last() {
                let mut core_self = vec![
                    Token::Identifier(ident.clone()),
                    Token::Identifier(ident),
                    Token::Block(Block::Parsed(block.clone())),
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
        (Some(Token::Block(Block::Raw(block))), Some(Token::Identifier(ident))) => {
            if let Some(function_index) = eval.state.current_function_index.last() {
                let mut core_self = vec![
                    Token::Identifier(ident.clone()),
                    Token::Identifier(ident),
                    Token::Block(Block::Raw(block.clone())),
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
        (Some(Token::Block(Block::Parsed(logic))), Some(Token::Block(Block::Parsed(setup)))) => {
            eval.state.execution_stack.push(Token::Block(Block::Auto(
                Rc::new(setup.to_vec()),
                Rc::new(logic.to_vec()),
            )))
        }
        (Some(Token::Block(Block::Raw(logic))), Some(Token::Block(Block::Parsed(setup)))) => {
            eval.state.execution_stack.push(Token::Block(Block::Auto(
                Rc::new(setup.to_vec()),
                Rc::new(logic.to_vec()),
            )))
        }
        (Some(Token::Block(Block::Raw(logic))), Some(Token::Block(Block::Raw(setup)))) => {
            eval.state.execution_stack.push(Token::Block(Block::Auto(
                Rc::new(setup.to_vec()),
                Rc::new(logic.to_vec()),
            )))
        }
        (Some(Token::Block(Block::Parsed(logic))), Some(Token::Block(Block::Raw(setup)))) => {
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
