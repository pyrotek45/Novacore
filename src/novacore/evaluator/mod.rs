use hashbrown::HashMap;

use super::{
    core::{Block, CallBack, Operator, Token},
    core_ops, state,
};

pub struct Evaluator {
    functions: Vec<CallBack>,
    pub state: state::State,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            functions: vec![],
            state: *state::new(),
        }
    }

    pub fn add_function(&mut self, function: CallBack) -> usize {
        self.functions.push(function);
        self.functions.len() - 1
    }

    pub fn eval(&mut self, expr: Token) {
        match expr {
            Token::Reg(opcodes) => core_ops::reg::register_operation(self, opcodes),
            Token::Function(index) => {
                self.state.current_function_index.push(index);
                self.functions[index](self);
                self.state.current_function_index.pop();
            }
            Token::FlowFunction(index) => {
                self.state.current_function_index.push(index);
                self.functions[index](self);
                self.state.current_function_index.pop();
            }
            Token::UserBlockCall(function) => core_ops::control::user_block_call(self, &function),
            Token::FlowUserBlockCall(function) => {
                core_ops::control::user_block_call(self, &function)
            }
            Token::Block(Block::Lambda(block)) => {
                self.state.call_stack.push(HashMap::new());
                self.evaluate(block.to_vec());
                if let Some(token) = self.state.get_from_heap_or_pop() {
                    self.state.execution_stack.push(token)
                }

                self.state.call_stack.pop();
            }
            Token::Block(Block::ListLambda(list)) => {
                if let Some(Token::Integer(index)) = self.state.get_from_heap_or_pop() {
                    if let Some(value) = list.get(index as usize) {
                        self.state.execution_stack.push(value.clone())
                    }
                }
            }
            Token::Op(operator) => match operator {
                Operator::AccessCall => core_ops::control::get_access(self),
                Operator::UserFunctionChain => core_ops::control::user_chain_call(self),
                Operator::StoreTemp => core_ops::control::store_temp(self),
                //Operator::Break => core_ops::control::break_loop(self),
                Operator::And => core_ops::logical::logical_and(self),
                Operator::Or => core_ops::logical::logical_or(self),
                Operator::Not => core_ops::logical::logical_not(self),
                Operator::Equals => core_ops::comparison::equality_comparison(self),
                Operator::Gtr => core_ops::comparison::greater_than_comparison(self),
                Operator::Lss => core_ops::comparison::less_than_comparison(self),
                Operator::Neg => core_ops::operator::neg(self),
                Operator::Mod => core_ops::operator::modulo(self),
                Operator::Add => core_ops::operator::add(self),
                Operator::Sub => core_ops::operator::sub(self),
                Operator::Mul => core_ops::operator::mul(self),
                Operator::Div => core_ops::operator::div(self),
                Operator::VariableAssign => core_ops::operator::variable_assign(self),
                Operator::FunctionVariableAssign => {
                    core_ops::operator::function_variable_assign(self)
                }
                Operator::SelfId => core_ops::operator::get_self(self),
                _ => {}
            },
            Token::Symbol(_) => {}
            _ => {
                self.state.execution_stack.push(expr);
            }
        }
    }

    pub fn evaluate(&mut self, expr: Vec<Token>) {
        for t in expr {
            self.eval(t);
        }
    }

    pub fn evaluate_function(&mut self, expr: Vec<Token>) {
        self.state.call_stack.push(HashMap::new());
        for t in expr {
            self.eval(t);
        }
        self.state.call_stack.pop();
    }
}
