use core::time;
use std::{thread, time::Instant};

use colored::Colorize;

use crate::novacore::{
    core::{Block, Token},
    evaluator::Evaluator,
};

#[inline(always)]
pub fn sleep(eval: &mut Evaluator) {
    if let Some(Token::Integer(time)) = eval.state.get_from_heap_or_pop() {
        let delay = time::Duration::from_millis(time as u64);
        thread::sleep(delay);
    } else {
        eval.state.show_error("Not enough arguments for sleep");
    }
}

#[inline(always)]
pub fn time(eval: &mut Evaluator) {
    if let Some(token) = eval.state.get_from_heap_or_pop() {
        if let Token::Block(block) = token {
            match block {
                Block::Function(_, block) => {
                    let start = Instant::now();
                    eval.evaluate_function(block);
                    let duration = start.elapsed();
                    println!("{} {:?}", ">> Execution:".bright_green(), duration);
                }
                Block::Literal(block) => {
                    // call in same scope
                    let start = Instant::now();
                    eval.evaluate(block);
                    let duration = start.elapsed();
                    println!("{} {:?}", ">> Execution:".bright_green(), duration);
                }
                Block::List(block) => {
                    // call in same scope
                    let start = Instant::now();
                    eval.evaluate(block);
                    let duration = start.elapsed();
                    println!("{} {:?}", ">> Execution:".bright_green(), duration);
                }
                _ => {
                    todo!()
                }
            }
        } else {
            eval.state.show_error(&format!("Cannot time {:?}", token));
        }
    } else {
        eval.state.show_error("Not enough arguments for time");
    }
}

#[inline(always)]
pub fn time_avg(eval: &mut Evaluator) {
    if let (Some(token), Some(times)) = (
        eval.state.get_from_heap_or_pop(),
        eval.state.get_from_heap_or_pop(),
    ) {
        if let Token::Integer(times) = times {
            if let Token::Block(block) = token {
                match block {
                    Block::Literal(block) => {
                        let mut timeslistseconds = vec![];
                        let mut timeslistmilli = vec![];
                        let mut timeslistnano = vec![];
                        for _i in 0..times {
                            let start = Instant::now();
                            eval.evaluate(block.clone());
                            let duration = start.elapsed();
                            timeslistseconds.push(duration.as_secs());
                            timeslistmilli.push(duration.as_millis());
                            timeslistnano.push(duration.as_nanos());
                        }
                        let mut secave = 1;
                        for i in timeslistseconds.iter() {
                            secave += i
                        }
                        let mut milave = 1;
                        for i in timeslistmilli.iter() {
                            milave += i
                        }
                        let mut nanave = 1;
                        for i in timeslistnano.iter() {
                            nanave += i
                        }
                        println!(
                            "{} {}s {}ms {}n ",
                            ">> Execution average:".bright_green(),
                            secave / timeslistseconds.len() as u64,
                            milave / timeslistmilli.len() as u128,
                            nanave / timeslistnano.len() as u128
                        );
                    }
                    a => eval.state.show_error(&format!(
                        "Incorrect arguments for timeave, got [{:?},{:?}]",
                        a, times
                    )),
                }
            } else {
                eval.state
                    .show_error(&format!("Cannot timeave {:?}", token));
            }
        } else {
            eval.state
                .show_error(&format!("Cannot timeave {:?}, is not an integer", times));
        }
    } else {
        eval.state.show_error("Not enough arguments for timeave");
    }
}
