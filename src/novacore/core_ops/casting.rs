use crate::novacore::{
    core::{Token},
    evaluator::Evaluator,
};

#[inline(always)]
pub fn as_int(eval: &mut Evaluator) {
    match eval.state.get_from_heap_or_pop() {
        Some(Token::Float(value)) => {
            eval.state
                .execution_stack
                .push(Token::Integer(value as i128));
        }

        Some(Token::String(value)) => {
            if let Ok(value) = value.parse::<i128>(){
                eval.state
                .execution_stack
                .push(Token::Integer(value));
            } else {
                eval.state.show_error(&format!(
                    "Could not parse, but got [{:?}]",
                    value
                )) 
            }

        }
        a => eval.state.show_error(&format!(
            "Incorrect argument for list. Expected Type [Block], but got [{:?}]",
            a
        )),
    }
}