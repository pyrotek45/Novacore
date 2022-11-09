use std::rc::Rc;

use crate::novacore::{core::Token, evaluator::Evaluator};

pub fn range(eval: &mut Evaluator) {
    if let (Some(end), Some(start)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        match (start, end) {
            (Token::Integer(start), Token::Integer(end)) => {
                let mut new_list: Vec<Token> = Vec::new();
                for x in start..=end {
                    new_list.push(Token::Integer(x));
                }
                eval.state
                    .execution_stack
                    .push(Token::List(Rc::new(new_list.to_vec())));
            }
            _ => {
                println!("cant make a range from these types");
            }
        }
    }
}
