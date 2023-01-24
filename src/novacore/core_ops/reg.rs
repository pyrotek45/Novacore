use crate::novacore::{core::Token, evaluator::Evaluator, utilities::print_error};

pub fn register_operation(eval: &mut Evaluator, opcodes: Vec<usize>) {
    let offset = eval.state.execution_stack.len() - 1;
    match opcodes[0] {
        // remove index
        0 => {
            eval.state.execution_stack.remove(offset - opcodes[1]);
        }
        // integer operations
        // int_add index index destination
        1 => {
            eval.state.execution_stack[offset - opcodes[3]] = Token::Integer(
                eval.state.execution_stack[offset - opcodes[1]].get_int()
                    + eval.state.execution_stack[offset - opcodes[2]].get_int(),
            )
        }
        // int_sub index index destination
        2 => {
            eval.state.execution_stack[offset - opcodes[3]] = Token::Integer(
                eval.state.execution_stack[offset - opcodes[1]].get_int()
                    - eval.state.execution_stack[offset - opcodes[2]].get_int(),
            )
        }
        // int_mul index index destination
        3 => {
            eval.state.execution_stack[offset - opcodes[3]] = Token::Integer(
                eval.state.execution_stack[offset - opcodes[1]].get_int()
                    * eval.state.execution_stack[offset - opcodes[2]].get_int(),
            )
        }

        // float operations
        // float_add index index destination
        4 => {
            eval.state.execution_stack[offset - opcodes[3]] = Token::Float(
                eval.state.execution_stack[offset - opcodes[1]].get_float()
                    + eval.state.execution_stack[offset - opcodes[2]].get_float(),
            )
        }
        // float_sub index index destination
        5 => {
            eval.state.execution_stack[offset - opcodes[3]] = Token::Float(
                eval.state.execution_stack[offset - opcodes[1]].get_float()
                    - eval.state.execution_stack[offset - opcodes[2]].get_float(),
            )
        }
        // float_mul index index destination
        6 => {
            eval.state.execution_stack[offset - opcodes[3]] = Token::Float(
                eval.state.execution_stack[offset - opcodes[1]].get_float()
                    * eval.state.execution_stack[offset - opcodes[2]].get_float(),
            )
        }
        // float_mul index index destination
        7 => {
            eval.state.execution_stack[offset - opcodes[3]] = Token::Float(
                eval.state.execution_stack[offset - opcodes[1]].get_float()
                    / eval.state.execution_stack[offset - opcodes[2]].get_float(),
            )
        }

        // swap index index destination
        8 => {
            let temp = eval.state.execution_stack[offset - opcodes[1]].clone();

            eval.state.execution_stack[offset - opcodes[1]] =
                eval.state.execution_stack[offset - opcodes[2]].clone();

            eval.state.execution_stack[offset - opcodes[2]] = temp
        }
        a => print_error(&format!("Incorrect reg operation, got  [{:?}]", a)),
    }
}
