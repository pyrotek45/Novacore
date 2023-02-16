//use std::{env};

use crate::novacore::{
    self,
    core::{Block, Token},
    evaluator::Evaluator,
    utilities::{is_string_number, trim_newline},
};

pub fn println(eval: &mut Evaluator) {
    if let Some(token) = eval.state.get_from_heap_or_pop() {
        match token {
            Token::Id(token) => {
                print!("{}\r\n", &token)
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
                print!("{}\r\n", token.to_str())
            }
            _ => eval
                .state
                .show_error(&format!("Incorrect argument for println, got {:?}", token)),
        }
    } else {
        eval.state.show_error("Not enough arguments for println");
    }
}

pub fn print(eval: &mut Evaluator) {
    if let Some(token) = eval.state.get_from_heap_or_pop() {
        match token {
            Token::Id(token) => {
                print!("{}", &token)
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
                print!("{}", token.to_str())
            }
            _ => eval
                .state
                .show_error(&format!("Incorrect argument for print, got {:?}", token)),
        }
    } else {
        eval.state.show_error("Not enough arguments for print");
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
        output_string.push_str(&stack_output.to_str());
        output_string.push(',')
    }
    output_string.pop();
    if !output_string.is_empty() {
        output_string.push(']');
        println!("{}", output_string);
    }
}

pub fn load(eval: &mut Evaluator) {
    if let (Some(Token::String(filepath)), Some(Token::Id(id))) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.execution_stack.pop(),
    ) {
        let mut vm = novacore::new_from_file(&filepath);
        vm.evaluator.state.current_file = filepath.clone();
        vm.evaluator
            .evaluate(vm.parser.parse(vm.lexer.parse()).into());
        if let Some(scope) = vm.evaluator.state.call_stack.pop() {
            eval.state.modules.insert(id, scope);
            for (key, item) in vm.evaluator.state.modules {
                eval.state.modules.insert(key, item);
            }
        } else {
            eval.state.show_error(&format!(
                "Incorrect argument for load, got {:?} {:?}",
                filepath, id
            ))
        }
    } else {
        eval.state.show_error("Not enough arguments for load");
    }
}

pub fn import(eval: &mut Evaluator) {
    if let Some(Token::Block(Block::List(list))) = eval.state.get_from_heap_or_pop() {
        for modules in &*list {
            if let Token::Id(module) = modules {
                let mut vm = novacore::new_from_file(&format!("std/{}.core", module));
                vm.evaluator.state.current_file = format!("std/{}.core", module);
                vm.evaluator
                    .evaluate(vm.parser.parse(vm.lexer.parse()).into());
                if let Some(scope) = vm.evaluator.state.call_stack.pop() {
                    eval.state.modules.insert(module.to_string(), scope);
                    for (key, item) in vm.evaluator.state.modules {
                        eval.state.modules.insert(key, item);
                    }
                } else {
                    eval.state
                        .show_error(&format!("Incorrect argument for import, got {:?}", module))
                }
            }
        }
    } else {
        eval.state.show_error("Not enough arguments for import");
    }
}

// pub fn args(eval: &mut Evaluator) {
//     let args: Vec<String> = env::args().collect();
//     dbg!(args);
// }
