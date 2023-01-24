use core::time;
use std::{thread, time::Instant};

use colored::Colorize;
use hashbrown::HashMap;

use crate::novacore::{
    core::{Block, Token},
    evaluator::Evaluator,
    utilities::print_error,
};

pub fn sleep(eval: &mut Evaluator) {
    if let Some(Token::Integer(time)) = eval.state.get_from_heap_or_pop() {
        let delay = time::Duration::from_millis(time as u64);
        thread::sleep(delay);
    } else {
        print_error("Not enough arguments for sleep");
    }
}

pub fn time(eval: &mut Evaluator) {
    if let Some(token) = eval.state.get_from_heap_or_pop() {
        if let Token::Block(block) = token {
            match block {
                Block::Function(block) => {
                    let start = Instant::now();
                    // Call with new scope
                    eval.state.call_stack.push(HashMap::new());

                    eval.evaluate(block.to_vec());

                    if let Some(token) = eval.state.get_from_heap_or_pop() {
                        eval.state.execution_stack.push(token)
                    }
                    eval.state.call_stack.pop();

                    let duration = start.elapsed();
                    println!("{} {:?}", ">> Execution:".bright_green(), duration);
                }
                Block::Literal(block) => {
                    // call in same scope
                    let start = Instant::now();
                    eval.evaluate(block.to_vec());
                    let duration = start.elapsed();
                    println!("{} {:?}", ">> Execution:".bright_green(), duration);
                }
                Block::List(block) => {
                    // call in same scope
                    let start = Instant::now();
                    eval.evaluate(block.to_vec());
                    let duration = start.elapsed();
                    println!("{} {:?}", ">> Execution:".bright_green(), duration);
                }
                _ => {
                    todo!()
                }
            }
        } else {
            print_error(&format!("Cannot time {:?}", token));
        }
    } else {
        print_error("Not enough arguments for time");
    }
}
