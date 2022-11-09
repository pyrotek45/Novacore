use core::time;
use std::thread;

use crate::novacore::{core::Token, evaluator::Evaluator};

pub fn sleep(eval: &mut Evaluator) {
    if let Some(Token::Integer(time)) = eval.state.get_from_heap_or_pop() {
        let delay = time::Duration::from_millis(time as u64);
        thread::sleep(delay);
    }
}
