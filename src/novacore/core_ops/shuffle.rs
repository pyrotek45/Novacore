use crate::novacore::evaluator::Evaluator;

#[inline(always)]
pub fn dup(eval: &mut Evaluator) {
    if let Some(top) = eval.state.execution_stack.last() {
        eval.state.execution_stack.push(top.clone())
    } else {
        eval.state.show_error("Not enough arguments for dup");
    }
}

#[inline(always)]
pub fn ddup(eval: &mut Evaluator) {
    if let Some(top) = eval.state.execution_stack.pop() {
        eval.state.execution_stack.push(top.clone());
        eval.state.execution_stack.push(top.clone());
        eval.state.execution_stack.push(top);
    } else {
        eval.state.show_error("Not enough arguments for ddup");
    }
}

#[inline(always)]
pub fn swap(eval: &mut Evaluator) {
    if let (Some(top), Some(under)) = (
        eval.state.execution_stack.pop(),
        eval.state.execution_stack.pop(),
    ) {
        eval.state.execution_stack.push(top);
        eval.state.execution_stack.push(under)
    } else {
        eval.state.show_error("Not enough arguments for swap");
    }
}

#[inline(always)]
pub fn drop(eval: &mut Evaluator) {
    if eval.state.execution_stack.pop().is_none() {
        eval.state.show_error("Not enough arguments for drop");
    }
}

#[inline(always)]
pub fn nip(eval: &mut Evaluator) {
    if let (Some(top), Some(_)) = (
        eval.state.execution_stack.pop(),
        eval.state.execution_stack.pop(),
    ) {
        eval.state.execution_stack.push(top)
    } else {
        eval.state.show_error("Not enough arguments for nip");
    }
}

#[inline(always)]
pub fn over(eval: &mut Evaluator) {
    if let (Some(top), Some(under)) = (
        eval.state.execution_stack.pop(),
        eval.state.execution_stack.pop(),
    ) {
        eval.state.execution_stack.push(under.clone());
        eval.state.execution_stack.push(top);
        eval.state.execution_stack.push(under);
    } else {
        eval.state.show_error("Not enough arguments for over");
    }
}

#[inline(always)]
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
        eval.state.show_error("Not enough arguments for dover");
    }
}

#[inline(always)]
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
        eval.state.show_error("Not enough arguments for rot");
    }
}

#[inline(always)]
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
        eval.state.show_error("Not enough arguments for drot");
    }
}

#[inline(always)]
pub fn wipe(eval: &mut Evaluator) {
    eval.state.execution_stack.clear()
}
