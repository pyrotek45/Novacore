use std::rc::Rc;

use crate::novacore::{
    core::{Token, LT},
    evaluator::Evaluator,
    state,
};

pub fn range(mut state: Box<state::State>, eval: &mut Evaluator) -> Box<state::State> {
    if let (Some(end), Some(start)) = (state.get_from_heap_or_pop(), state.get_from_heap_or_pop()) {
        match (start, end) {
            (Token::Integer(start), Token::Integer(end)) => {
                let mut new_list: Vec<Token> = Vec::new();
                for x in start..=end {
                    new_list.push(Token::Integer(x));
                }
                state
                    .execution_stack
                    .push(Token::List(LT::Packed(Rc::new(new_list.to_vec()))));
            }
            _ => {
                println!("cant make a range from these types");
            }
        }
    }

    state
}
