use crate::novacore::{core::Token, evaluator::Evaluator};


pub fn logical_and(eval: &mut Evaluator) {
    if let (Some(right), Some(left)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        match (&right, &left) {
            (Token::Bool(right), Token::Bool(left)) => {
                eval.state
                    .execution_stack
                    .push(Token::Bool(*left && *right));
            }
            _ => eval.state.show_error(&format!(
                "Incorrect argument for and, got [{:?},{:?}]",
                left, right
            )),
        }
    } else {
        eval.state.show_error("Not enough arguments for and");
    }
}


pub fn logical_or(eval: &mut Evaluator) {
    if let (Some(right), Some(left)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        match (&right, &left) {
            (Token::Bool(right), Token::Bool(left)) => {
                eval.state
                    .execution_stack
                    .push(Token::Bool(*left || *right));
            }
            _ => eval.state.show_error(&format!(
                "Incorrect argument for or, got [{:?},{:?}]",
                left, right
            )),
        }
    } else {
        eval.state.show_error("Not enough arguments for or");
    }
}


pub fn logical_not(eval: &mut Evaluator) {
    if let Some(token) = eval.state.get_from_heap_or_pop() {
        if let Token::Bool(bool) = token {
            eval.state.execution_stack.push(Token::Bool(!bool));
        } else {
            eval.state
                .show_error(&format!("Incorrect argument for not, got [{:?}]", token))
        }
    } else {
        eval.state.show_error("Not enough arguments for not");
    }
}
