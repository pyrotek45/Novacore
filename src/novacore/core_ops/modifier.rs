use std::rc::Rc;

use fxhash::FxHashMap as HashMap;

use crate::novacore::{
    core::{Block, Instructions, Operator, Token},
    evaluator::Evaluator,
};

pub fn create_struct(eval: &mut Evaluator) {
    match eval.state.get_from_heap_or_pop() {
        Some(Token::Block(Block::Literal(block))) => {
            eval.state.call_stack.push(HashMap::default());
            eval.evaluate(block);
            if let Some(new_struct) = eval.state.call_stack.pop() {
                eval.state
                    .execution_stack
                    .push(Token::Block(Block::Struct(Rc::new(new_struct))));
            }
        }
        a => eval.state.show_error(&format!(
            "Incorrect argument for struct. Expected Type [Block], but got [{:?}]",
            a
        )),
    }
}

pub fn block(eval: &mut Evaluator) {
    match eval.state.get_from_heap_or_pop() {
        Some(Token::Block(Block::List(block))) => {
            eval.state
                .execution_stack
                .push(Token::Block(Block::Literal(block)));
        }
        a => eval.state.show_error(&format!(
            "Incorrect argument for block. Expected Type [List], but got [{:?}]",
            a
        )),
    }
}

pub fn list(eval: &mut Evaluator) {
    match eval.state.get_from_heap_or_pop() {
        Some(Token::Block(Block::Literal(block))) => {
            eval.state
                .execution_stack
                .push(Token::Block(Block::List(block)));
        }
        a => eval.state.show_error(&format!(
            "Incorrect argument for list. Expected Type [Block], but got [{:?}]",
            a
        )),
    }
}

pub fn func(eval: &mut Evaluator) {
    if let (Some(block), Some(list)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        match (block, list) {
            (Token::Block(Block::Literal(block)), Token::Block(Block::List(list))) => {
                eval.state
                    .execution_stack
                    .push(Token::Block(Block::Function(list, block)));
            }
            (a, b) => eval.state.show_error(&format!(
                "Incorrect argument for func. Expected Type [Block | List], but got [{:?} {:?}]",
                a, b
            )),
        }
    }
}

// pub fn modifier(eval: &mut Evaluator) {
//     match eval.state.get_from_heap_or_pop() {
//         Some(Token::Block(Block::Literal(block))) => {
//             eval.state
//                 .execution_stack
//                 .push(Token::Block(Block::Modifier(None, block)));
//         }
//         a => eval.state.show_error(&format!(
//             "Incorrect argument for mod. Expected Type [Block], but got [{:?}]",
//             a
//         )),
//     }
// }

// pub fn closure_let(eval: &mut Evaluator) {
//     match eval.state.get_from_heap_or_pop() {
//         Some(Token::Block(Block::Literal(block))) => {
//             if let Some(scope) = eval.state.call_stack.last_mut() {
//                 let mut core_self = vec![];

//                 for (ident, token) in scope {
//                     core_self.push(Token::Id(ident.clone()));
//                     core_self.push(token.clone());
//                     core_self.push(Token::Op(_,Operator::VariableAssign))
//                 }

//                 for t in block.iter() {
//                     core_self.push(t.clone())
//                 }

//                 eval.state
//                     .execution_stack
//                     .push(Token::Block(Block::Function(Rc::new(core_self))))
//             }
//         }
//         a => eval.state.show_error(&format!(
//             "Incorrect argument for let. Expected Type [Block], but got [{:?}]",
//             a
//         )),
//     }
// }

// pub fn closure_auto(eval: &mut Evaluator) {
//     match (
//         eval.state.get_from_heap_or_pop(),
//         eval.state.get_from_heap_or_pop(),
//     ) {
//         (Some(Token::Block(Block::Literal(logic))), Some(Token::Block(Block::Literal(setup)))) => {
//             eval.state.execution_stack.push(Token::Block(Block::Auto(
//                 Rc::new(setup.to_vec()),
//                 Rc::new(logic.to_vec()),
//             )))
//         }
//         (a, b) => eval.state.show_error(&format!(
//             "Incorrect argument for auto. Expected Types [Block , Block], but got [{:?},{:?}]",
//             a, b
//         )),
//     }
// }

pub fn include(eval: &mut Evaluator) {
    fn include_compute(
        eval: &mut Evaluator,
        block: Instructions,
        list: Instructions,
    ) -> Vec<Token> {
        let mut newlist = vec![];
        if let Some(scope) = eval.state.call_stack.last_mut() {
            for item in list.iter() {
                if let Token::Id(ident) = item {
                    if let Some(token) = scope.get(ident) {
                        newlist.push(Token::Id(ident.clone()));
                        newlist.push(token.clone());
                        newlist.push(Token::Op(Operator::VariableAssign, 0))
                    }
                }
            }

            for t in block.iter() {
                newlist.push(t.clone())
            }
        }
        newlist
    }

    match (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        (Some(Token::Block(Block::Literal(block))), Some(Token::Block(Block::List(list)))) => {
            let value = include_compute(eval, block, list);
            eval.state
                .execution_stack
                .push(Token::Block(Block::Literal(Rc::new(value))))
        }
        (
            Some(Token::Block(Block::Function(vars, block))),
            Some(Token::Block(Block::List(list))),
        ) => {
            let value = include_compute(eval, block, list);
            eval.state
                .execution_stack
                .push(Token::Block(Block::Function(vars, Rc::new(value))))
        }
        (a, b) => eval.state.show_error(&format!(
            "Incorrect argument for include, got [{:?},{:?}]",
            a, b
        )),
    }
}

pub fn memo(eval: &mut Evaluator) {
    if let Some(Token::Block(Block::Literal(block))) = eval.state.get_from_heap_or_pop() {
        eval.state.memoize = true;
        eval.evaluate(block);
        eval.state.memoize = false;
    }
}
