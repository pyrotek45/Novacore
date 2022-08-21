use rand::Rng;

use crate::novacore::{core::Token, evaluator::Evaluator, state};

pub fn random(mut state: Box<state::State>, eval: &mut Evaluator) -> Box<state::State> {
    if let (Some(end), Some(start)) = (state.get_from_heap_or_pop(), state.get_from_heap_or_pop()) {
        match (start, end) {
            (Token::Integer(left), Token::Integer(right)) => {
                if left <= right {
                    let mut rng = rand::thread_rng();
                    state
                        .execution_stack
                        .push(Token::Integer(rng.gen_range(left..=right)));
                } else {
                    let mut rng = rand::thread_rng();
                    state
                        .execution_stack
                        .push(Token::Integer(rng.gen_range(right..=left)));
                }
            }
            _ => {
                println!("cant make random");
            }
        }
    }

    state
}
