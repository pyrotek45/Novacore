use crate::novacore::{core::Token, evaluator::Evaluator};

pub fn println(eval: &mut Evaluator) {
    if let Some(token) = eval.state.get_from_heap_or_pop() {
        match token {
            Token::Identifier(token) => {
                print!("{}\r\n", &token)
            }
            Token::Function(index) => {
                print!("{}\r\n", index)
            }
            Token::Integer(token) => {
                print!("{}\r\n", &token);
            }
            Token::Float(token) => {
                print!("{}\r\n", &token)
            }
            Token::String(token) => {
                print!("{}\r\n", &token)
            }
            Token::Symbol(token) => {
                print!("{}\r\n", &token)
            }
            Token::Bool(token) => {
                print!("{}\r\n", &token)
            }
            Token::Char(token) => {
                print!("{}\r\n", token)
            }
            Token::Block(_) => {
                print!("BLOCK\r\n")
            }
            Token::List(_) => {
                print!("LIST\r\n")
            }
            Token::UserBlockCall(_) => {
                print!("Block Call\r\n")
            }
            Token::Op(_) => {
                print!("Op\r\n")
            }
            Token::FlowFunction(_) => todo!(),
            Token::FlowUserBlockCall(_) => todo!(),
        }
    }
}

pub fn print(eval: &mut Evaluator) {
    if let Some(token) = eval.state.get_from_heap_or_pop() {
        match token {
            Token::Identifier(token) => {
                print!("{}", &token)
            }
            Token::Function(index) => {
                print!("{}", index)
            }
            Token::Integer(token) => {
                print!("{}", &token);
            }
            Token::Float(token) => {
                print!("{}", &token)
            }
            Token::String(token) => {
                print!("{}", &token)
            }
            Token::Symbol(token) => {
                print!("{}", &token)
            }
            Token::Bool(token) => {
                print!("{}", &token)
            }
            Token::Char(token) => {
                print!("{}", token)
            }
            Token::Block(_) => {
                print!("BLOCK")
            }
            Token::List(_) => {
                print!("LIST")
            }
            Token::UserBlockCall(_) => {
                print!("Block Call")
            }
            Token::Op(_) => {
                print!("Op")
            }
            Token::FlowFunction(_) => todo!(),
            Token::FlowUserBlockCall(_) => todo!(),
        }
    }
}
