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
            Token::Function(index) => {
                self.state.current_function_index = index;
                self.functions[index](self);
            }
            Token::FlowFunction(index) => {
                self.state.current_function_index = index;
                self.functions[index](self);
            }
            Token::UserBlockCall(function) => core_ops::control::user_block_call(self, &function),
            Token::FlowUserBlockCall(function) => {
                core_ops::control::user_block_call(self, &function)
            }
            Token::Block(Block::Lambda(block)) => {
                // Call with new scope
                self.state.call_stack.push(HashMap::new());
                self.evaluate(block.to_vec());
                if let Some(token) = self.state.get_from_heap_or_pop() {
                    self.state.execution_stack.push(token)
                }
                self.state.call_stack.pop();
            }
            Token::Op(operator) => match operator {
                Operator::Continue => {
                    self.state.continue_loop = true;
                }
                Operator::BlockCall => core_ops::control::block_call(self),
                Operator::AccessCall => core_ops::control::get_access(self),
                Operator::For => core_ops::control::for_loop(self),
                Operator::If => core_ops::control::if_statement(self),
                Operator::UserFunctionChain => core_ops::control::user_chain_call(self),
                Operator::Return => core_ops::operator::return_top(self),
                Operator::StoreTemp => core_ops::control::store_temp(self),
                Operator::Break => core_ops::control::break_loop(self),
                Operator::And => core_ops::logical::logical_and(self),
                Operator::Or => core_ops::logical::logical_or(self),
                Operator::Not => core_ops::logical::logical_not(self),
                Operator::Equals => core_ops::comparison::equals(self),
                Operator::Gtr => core_ops::comparison::gtr_comparison(self),
                Operator::Lss => core_ops::comparison::lss_comparison(self),
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
            _ => {
                self.state.execution_stack.push(expr);
            }
        }
    }

    pub fn evaluate(&mut self, expr: Vec<Token>) {
        for token in &expr {
            self.eval(token.clone());
        }
    }
}
