use core::time;
use std::thread;

use crate::novacore::{core::Token, evaluator::Evaluator, state};

pub fn sleep(mut state: Box<state::State>, eval: &mut Evaluator) -> Box<state::State> {
    match state.get_from_heap_or_pop() {
        Some(time) => {
            if let Token::Integer(time) = time {
                let delay = time::Duration::from_millis(time as u64);
                thread::sleep(delay);
            }
            state
        }
        None => state,
    }
}
