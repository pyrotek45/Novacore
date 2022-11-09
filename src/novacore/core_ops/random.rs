use rand::Rng;

use crate::novacore::{core::Token, evaluator::Evaluator};

pub fn random(eval: &mut Evaluator) {
    if let (Some(end), Some(start)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        match (start, end) {
            (Token::Integer(left), Token::Integer(right)) => {
                if left <= right {
                    let mut rng = rand::thread_rng();
                    eval.state
                        .execution_stack
                        .push(Token::Integer(rng.gen_range(left..=right)));
                } else {
                    let mut rng = rand::thread_rng();
                    eval.state
                        .execution_stack
                        .push(Token::Integer(rng.gen_range(right..=left)));
                }
            }
            _ => {
                println!("cant make random");
            }
        }
    }
}
