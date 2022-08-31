use rand::Rng;

use crate::novacore::{core::Token, evaluator::Evaluator, state};

pub fn random(mut state: Box<state::State>, eval: &mut Evaluator) -> Box<state::State> {
    if let (Some(end), Some(start)) = (state.get_from_heap_or_pop(), state.get_from_heap_or_pop()) {
        match (&start, &end) {
            (Token::Integer(left), Token::Integer(right)) => {
                if left <= right {
                    let mut rng = rand::thread_rng();
                    state
                        .execution_stack
                        .push(Token::Integer(rng.gen_range(*left..=*right)));
                } else {
                    let mut rng = rand::thread_rng();
                    state
                        .execution_stack
                        .push(Token::Integer(rng.gen_range(*right..=*left)));
                }
            }
            _ => {
                if state.debug {
                    // Log error
                    if start == Token::Break || end == Token::Break {
                        state.error_log.push((
                            "Not enough arguments for random()".to_string(),
                            state.line_number,
                        ));
                    } else {
                        state.error_log.push((
                            format!(
                                "random() cannot use types [{} :: {}]: Expected type [Integer]",
                                end.to_str(),
                                start.to_str()
                            ),
                            state.line_number,
                        ));
                    }
                }
            }
        }
    } else if state.debug {
        state.error_log.push((
            "Not enough arguments for random()".to_string(),
            state.line_number,
        ));
    }

    state
}
