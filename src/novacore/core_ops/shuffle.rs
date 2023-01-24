use crate::novacore::{evaluator::Evaluator, utilities::print_error};

pub fn dup(eval: &mut Evaluator) {
    if let Some(top) = eval.state.execution_stack.last() {
        eval.state.execution_stack.push(top.clone())
    } else {
        print_error("Not enough arguments for dup");
    }
}

pub fn ddup(eval: &mut Evaluator) {
    if let Some(top) = eval.state.execution_stack.pop() {
        eval.state.execution_stack.push(top.clone());
        eval.state.execution_stack.push(top.clone());
        eval.state.execution_stack.push(top);
    } else {
        print_error("Not enough arguments for ddup");
    }
}

pub fn swap(eval: &mut Evaluator) {
    if let (Some(top), Some(under)) = (
        eval.state.execution_stack.pop(),
        eval.state.execution_stack.pop(),
    ) {
        eval.state.execution_stack.push(top);
        eval.state.execution_stack.push(under)
    } else {
        print_error("Not enough arguments for swap");
    }
}

pub fn drop(eval: &mut Evaluator) {
    if eval.state.execution_stack.pop().is_none() {
        print_error("Not enough arguments for drop");
    }
}

pub fn nip(eval: &mut Evaluator) {
    if let (Some(top), Some(_)) = (
        eval.state.execution_stack.pop(),
        eval.state.execution_stack.pop(),
    ) {
        eval.state.execution_stack.push(top)
    } else {
        print_error("Not enough arguments for nip");
    }
}

pub fn over(eval: &mut Evaluator) {
    if let (Some(top), Some(under)) = (
        eval.state.execution_stack.pop(),
        eval.state.execution_stack.pop(),
    ) {
        eval.state.execution_stack.push(under.clone());
        eval.state.execution_stack.push(top);
        eval.state.execution_stack.push(under);
    } else {
        print_error("Not enough arguments for over");
    }
}

pub fn dover(eval: &mut Evaluator) {
    if let (Some(b), Some(a)) = (
        eval.state.execution_stack.pop(),
        eval.state.execution_stack.pop(),
    ) {
        eval.state.execution_stack.push(a.clone());
        eval.state.execution_stack.push(b.clone());
        eval.state.execution_stack.push(a);
        eval.state.execution_stack.push(b);
    } else {
        print_error("Not enough arguments for dover");
    }
}

pub fn rot(eval: &mut Evaluator) {
    if let (Some(top), Some(mid), Some(bottom)) = (
        eval.state.execution_stack.pop(),
        eval.state.execution_stack.pop(),
        eval.state.execution_stack.pop(),
    ) {
        eval.state.execution_stack.push(mid);
        eval.state.execution_stack.push(top);
        eval.state.execution_stack.push(bottom);
    } else {
        print_error("Not enough arguments for rot");
    }
}

pub fn drot(eval: &mut Evaluator) {
    if let (Some(top), Some(mid), Some(bottom)) = (
        eval.state.execution_stack.pop(),
        eval.state.execution_stack.pop(),
        eval.state.execution_stack.pop(),
    ) {
        eval.state.execution_stack.push(top);
        eval.state.execution_stack.push(bottom);
        eval.state.execution_stack.push(mid);
    } else {
        print_error("Not enough arguments for drot");
    }
}

pub fn wipe(eval: &mut Evaluator) {
    eval.state.execution_stack.clear()
}
