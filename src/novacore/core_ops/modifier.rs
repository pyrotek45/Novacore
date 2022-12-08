use std::rc::Rc;

use crate::novacore::{
    core::{Block, Operator, Token},
    evaluator::Evaluator,
};

pub fn proc(eval: &mut Evaluator) {
    if let Some(Token::Block(Block::Literal(block))) = eval.state.get_from_heap_or_pop() {
        eval.state
            .execution_stack
            .push(Token::Block(Block::Procedure(block)));
    }
}

pub fn object(eval: &mut Evaluator) {
    if let Some(Token::Block(Block::Literal(block))) = eval.state.get_from_heap_or_pop() {
        eval.state
            .execution_stack
            .push(Token::Block(Block::Object(block)));
    }
}

pub fn method(eval: &mut Evaluator) {
    if let Some(Token::Block(Block::Literal(block))) = eval.state.get_from_heap_or_pop() {
        eval.state
            .execution_stack
            .push(Token::Block(Block::Method(block)));
    }
}

pub fn closure_let(eval: &mut Evaluator) {
    if let Some(Token::Block(Block::Literal(block))) = eval.state.get_from_heap_or_pop() {
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
                .push(Token::Block(Block::Literal(Rc::new(core_self))))
        }
    }
}

pub fn closure_rec(eval: &mut Evaluator) {
    if let (Some(Token::Block(Block::Literal(block))), Some(Token::Identifier(ident))) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.execution_stack.pop(),
    ) {
        let mut core_self = vec![
            Token::Identifier(ident.clone()),
            Token::Identifier(ident),
            Token::Block(Block::Literal(block.clone())),
            Token::Function(eval.state.current_function_index),
            Token::Op(Operator::VariableAssign),
        ];

        for t in block.iter() {
            core_self.push(t.clone());
        }

        eval.state
            .execution_stack
            .push(Token::Block(Block::Literal(Rc::new(core_self))))
    }
}

pub fn closure_auto(eval: &mut Evaluator) {
    if let (Some(Token::Block(Block::Literal(logic))), Some(Token::Block(Block::Literal(setup)))) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        eval.state.execution_stack.push(Token::Block(Block::Auto(
            Rc::new(setup.to_vec()),
            Rc::new(logic.to_vec()),
        )))
    }
}
