use modulo::Mod;

use crate::novacore::{core::Token, evaluator::Evaluator};

pub fn register_operation(eval: &mut Evaluator, opcodes: Vec<usize>) {
    let mut regi: usize = 0;
    loop {
        if eval.state.execution_stack.is_empty() {
            eval.state.show_error("RegVm cannot use stack size 0");
            break;
        }
        let offset = eval.state.execution_stack.len() - 1;
        match opcodes[regi] {
            // end
            // no arguments
            0 => {
                break;
            }
            // integer operations
            // iadd index index destination
            1 => {
                eval.state.execution_stack[offset - opcodes[regi + 3]] = Token::Integer(
                    eval.state.execution_stack[offset - opcodes[regi + 1]].get_int()
                        + eval.state.execution_stack[offset - opcodes[regi + 2]].get_int(),
                );
                regi += 4;
            }
            // isub index index destination
            2 => {
                eval.state.execution_stack[offset - opcodes[regi + 3]] = Token::Integer(
                    eval.state.execution_stack[offset - opcodes[regi + 1]].get_int()
                        - eval.state.execution_stack[offset - opcodes[regi + 2]].get_int(),
                );
                regi += 4;
            }
            // // imul index index destination
            3 => {
                eval.state.execution_stack[offset - opcodes[regi + 3]] = Token::Integer(
                    eval.state.execution_stack[offset - opcodes[regi + 1]].get_int()
                        * eval.state.execution_stack[offset - opcodes[regi + 2]].get_int(),
                );
                regi += 4;
            }

            // // float operations
            // // fadd index index destination
            4 => {
                eval.state.execution_stack[offset - opcodes[regi + 3]] = Token::Float(
                    eval.state.execution_stack[offset - opcodes[regi + 1]].get_float()
                        + eval.state.execution_stack[offset - opcodes[regi + 2]].get_float(),
                );
                regi += 4;
            }
            // // fsub index index destination
            5 => {
                eval.state.execution_stack[offset - opcodes[regi + 3]] = Token::Float(
                    eval.state.execution_stack[offset - opcodes[regi + 1]].get_float()
                        - eval.state.execution_stack[offset - opcodes[regi + 2]].get_float(),
                );
                regi += 4;
            }
            // // fmul index index destination
            6 => {
                eval.state.execution_stack[offset - opcodes[regi + 3]] = Token::Float(
                    eval.state.execution_stack[offset - opcodes[regi + 1]].get_float()
                        * eval.state.execution_stack[offset - opcodes[regi + 2]].get_float(),
                );
                regi += 4;
            }
            // // fmul index index destination
            7 => {
                eval.state.execution_stack[offset - opcodes[regi + 3]] = Token::Float(
                    eval.state.execution_stack[offset - opcodes[regi + 1]].get_float()
                        / eval.state.execution_stack[offset - opcodes[regi + 2]].get_float(),
                );
                regi += 4;
            }

            // // swap index index
            8 => {
                let temp = eval.state.execution_stack[offset - opcodes[regi + 1]].clone();

                eval.state.execution_stack[offset - opcodes[regi + 1]] =
                    eval.state.execution_stack[offset - opcodes[regi + 2]].clone();

                eval.state.execution_stack[offset - opcodes[regi + 2]] = temp;

                regi += 3;
            }
            // // copy index destination
            9 => {
                eval.state.execution_stack[offset - opcodes[regi + 2]] =
                    eval.state.execution_stack[offset - opcodes[regi + 1]].clone();

                regi += 3;
            }

            // jump
            // jump to target
            10 => regi = opcodes[regi + 1],

            // jump if equal
            // index1 index2 target
            11 => {
                if eval.state.execution_stack[offset - opcodes[regi + 1]]
                    == eval.state.execution_stack[offset - opcodes[regi + 2]]
                {
                    regi = opcodes[regi + 3]
                } else {
                    regi += 4;
                }
            }

            // jump if not equal
            // index1 index2 target
            12 => {
                if eval.state.execution_stack[offset - opcodes[regi + 1]]
                    != eval.state.execution_stack[offset - opcodes[regi + 2]]
                {
                    regi = opcodes[regi + 3]
                } else {
                    regi += 4;
                }
            }

            // jump if  >  (ints)
            // index1 index2 target
            13 => {
                if eval.state.execution_stack[offset - opcodes[regi + 2]].get_int()
                    > eval.state.execution_stack[offset - opcodes[regi + 1]].get_int()
                {
                    regi = opcodes[regi + 3]
                } else {
                    regi += 4;
                }
            }

            // jump if  >  (floats)
            // index1 index2 target
            14 => {
                if eval.state.execution_stack[offset - opcodes[regi + 2]].get_float()
                    > eval.state.execution_stack[offset - opcodes[regi + 1]].get_float()
                {
                    regi = opcodes[regi + 3]
                } else {
                    regi += 4;
                }
            }

            // mod index index destination
            15 => {
                eval.state.execution_stack[offset - opcodes[regi + 3]] = Token::Integer(
                    eval.state.execution_stack[offset - opcodes[regi + 1]]
                        .get_int()
                        .modulo(eval.state.execution_stack[offset - opcodes[regi + 2]].get_int()),
                );
                regi += 4;
            }

            // out index
            16 => {
                println!(
                    "{}",
                    eval.state.execution_stack[offset - opcodes[regi + 1]].to_str()
                );
                regi += 2;
            }
            // inc index
            17 => {
                eval.state.execution_stack[offset - opcodes[regi + 1]] = Token::Integer(
                    eval.state.execution_stack[offset - opcodes[regi + 1]].get_int() + 1,
                );
                regi += 2;
            }
            // dec index
            18 => {
                eval.state.execution_stack[offset - opcodes[regi + 1]] = Token::Integer(
                    eval.state.execution_stack[offset - opcodes[regi + 1]].get_int() - 1,
                );
                regi += 2;
            }
            // set index value
            19 => {
                eval.state.execution_stack[offset - opcodes[regi + 1]] =
                    Token::Integer(opcodes[regi + 2] as i128);
                regi += 3;
            }
            20 => {
                if eval.state.execution_stack[offset - opcodes[regi + 1]]
                    == eval.state.execution_stack[offset - opcodes[regi + 2]]
                {
                    regi += opcodes[regi + 3] + 4
                } else {
                    regi += 4;
                }
            }
            // relitive jump if not equal
            // index1 index2 target
            21 => {
                if eval.state.execution_stack[offset - opcodes[regi + 1]]
                    != eval.state.execution_stack[offset - opcodes[regi + 2]]
                {
                    regi += opcodes[regi + 3] + 4
                } else {
                    regi += 4;
                }
            }

            // relitive jump back
            22 => regi -= opcodes[regi + 1],
            23 => {
                if eval.state.execution_stack[offset - opcodes[regi + 1]]
                    == eval.state.execution_stack[offset - opcodes[regi + 2]]
                {
                    regi -= opcodes[regi + 3]
                } else {
                    regi += 4;
                }
            }
            // relitive jump if not equal
            // index1 index2 target
            24 => {
                if eval.state.execution_stack[offset - opcodes[regi + 1]]
                    != eval.state.execution_stack[offset - opcodes[regi + 2]]
                {
                    regi -= opcodes[regi + 3]
                } else {
                    regi += 4;
                }
            }

            // // dcopy index des des
            25 => {
                eval.state.execution_stack[offset - opcodes[regi + 2]] =
                    eval.state.execution_stack[offset - opcodes[regi + 1]].clone();
                eval.state.execution_stack[offset - opcodes[regi + 3]] =
                    eval.state.execution_stack[offset - opcodes[regi + 1]].clone();
                regi += 4;
            }

            a => eval
                .state
                .show_error(&format!("Incorrect reg operation, got  [{:?}]", a)),
        }
    }
}
