use rand::Rng;

use crate::novacore::{core::Token, evaluator::Evaluator};


pub fn random(eval: &mut Evaluator) {
    if let (Some(ref end), Some(ref start)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        match (start, end) {
            (Token::Integer(left), Token::Integer(right)) => {
                if left <= right {
                    let mut rng = rand::thread_rng();
                    eval.state
                        .execution_stack
                        .push(Token::Integer(rng.gen_range(*left..=*right)));
                } else {
                    let mut rng = rand::thread_rng();
                    eval.state
                        .execution_stack
                        .push(Token::Integer(rng.gen_range(*right..=*left)));
                }
            }
            (a, b) => eval.state.show_error(&format!(
                "Incorrect arguments for random, got [{:?},{:?}]",
                a, b
            )),
        }
    } else {
        eval.state.show_error("Not enough arguments for random");
    }
}
