use std::rc::Rc;

use crate::novacore::{
    core::{Block, Operator, Token},
    evaluator::Evaluator,
    state,
};

pub fn proc(mut state: Box<state::State>, eval: &mut Evaluator) -> Box<state::State> {
    if let Some(Token::Block(Block::Literal(block))) = state.get_from_heap_or_pop() {
        state
            .execution_stack
            .push(Token::Block(Block::Procedure(block)));
    }
    state
}

pub fn closure_let(mut state: Box<state::State>, eval: &mut Evaluator) -> Box<state::State> {
    if let Some(Token::Block(Block::Literal(block))) = state.get_from_heap_or_pop() {
        if let Some(scope) = state.call_stack.last_mut() {
            let mut core_self = vec![];

            for (ident, token) in scope {
                core_self.push(Token::Identifier(ident.clone()));
                core_self.push(token.clone());
                core_self.push(Token::Op(Operator::VariableAssign))
            }

            for t in block.iter() {
                core_self.push(t.clone())
            }

            state
                .execution_stack
                .push(Token::Block(Block::Literal(Rc::new(core_self))))
        }
    }
    state
}
