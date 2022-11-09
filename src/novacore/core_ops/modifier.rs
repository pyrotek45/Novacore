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
