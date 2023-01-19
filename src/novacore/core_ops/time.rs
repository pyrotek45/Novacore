use core::time;
use std::{thread, time::Instant};

use colored::Colorize;
use hashbrown::HashMap;

use crate::novacore::{
    core::{Block, Token},
    evaluator::Evaluator,
};

pub fn sleep(eval: &mut Evaluator) {
    if let Some(Token::Integer(time)) = eval.state.get_from_heap_or_pop() {
        let delay = time::Duration::from_millis(time as u64);
        thread::sleep(delay);
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
                Block::Procedure(block) => {
                    // call in same scope
                    let start = Instant::now();
                    eval.evaluate(block.to_vec());
                    let duration = start.elapsed();
                    println!("{} {:?}", ">> Execution:".bright_green(), duration);
                }
                Block::Parsed(block) => {
                    // call in same scope
                    let start = Instant::now();
                    eval.evaluate(block.to_vec());
                    let duration = start.elapsed();
                    println!("{} {:?}", ">> Execution:".bright_green(), duration);
                }
                Block::Raw(block) => {
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
            println!("Cant call this type");
        }
    }
}
