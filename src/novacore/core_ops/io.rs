use crossterm::event::Event;

use crate::novacore::{
    core::Token,
    evaluator::Evaluator,
    utilities::{is_string_number, trim_newline},
};

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
            Token::UserBlockCall(_) => {
                print!("Block Call\r\n")
            }
            Token::Op(optype) => {
                print!("Operator -> {:?}\r\n", optype)
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

pub fn readln(eval: &mut Evaluator) {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let line = trim_newline(&mut line);

    if is_string_number(&line) {
        // Float
        if line.contains('.') {
            if let Ok(v) = line.parse() {
                eval.state.execution_stack.push(Token::Float(v));
            }
        } else {
            // Int
            if let Ok(v) = line.parse() {
                eval.state.execution_stack.push(Token::Integer(v));
            }
        }
    } else if line.chars().count() == 1 {
        if let Some(char) = line.chars().next() {
            eval.state.execution_stack.push(Token::Char(char));
        }
    } else {
        eval.state.execution_stack.push(Token::String(line));
    }
}

pub fn dump(eval: &mut Evaluator) {
    let mut output_string = String::new();
    output_string.push('[');
    for stack_output in eval.state.execution_stack.iter() {
        output_string.push_str(&stack_output.to_str_compact());
        output_string.push(',')
    }
    output_string.pop();
    if !output_string.is_empty() {
        output_string.push(']');
        println!("{}", output_string);
    }
}
