use hashbrown::HashMap;

use super::{
    core::{Block, CallBack, Operator, Token},
    core_ops, state,
};

pub struct Evaluator {
    functions: Vec<CallBack>,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator { functions: vec![] }
    }

    pub fn add_function(&mut self, function: CallBack) -> usize {
        self.functions.push(function);
        self.functions.len() - 1
    }

    pub fn eval(&mut self, mut state: Box<state::State>, expr: Token) -> Box<state::State> {
        match expr {
            Token::Function(index) => self.functions[index](state, self),
            Token::UserBlockCall(function) => {
                core_ops::control::user_block_call(state, self, &function)
            }
            Token::Block(Block::Lambda(block)) => {
                // Call with new scope
                state.call_stack.push(HashMap::new());
                state = self.evaluate(block.to_vec(), state);
                if let Some(token) = state.get_from_heap_or_pop() {
                    state.execution_stack.push(token)
                }
                state.call_stack.pop();
                state
            }
            Token::Op(operator) => match operator {
                Operator::AccessCall => core_ops::control::get_access(state, self),
                Operator::For => core_ops::control::for_loop(state, self),
                Operator::If => core_ops::control::if_statement(state, self),
                Operator::UserFunctionChain => core_ops::control::user_chain_call(state, self),
                Operator::Return => core_ops::operator::return_top(state),
                Operator::StoreTemp => core_ops::control::store_temp(state),
                Operator::Break => core_ops::control::break_loop(state),
                Operator::And => core_ops::logical::logical_and(state),
                Operator::Or => core_ops::logical::logical_or(state),
                Operator::Not => core_ops::logical::logical_not(state),
                Operator::Equals => core_ops::comparison::equals(state),
                Operator::Gtr => core_ops::comparison::gtr_comparison(state),
                Operator::Lss => core_ops::comparison::lss_comparison(state),
                Operator::Neg => core_ops::operator::neg(state),
                Operator::Mod => core_ops::operator::modulo(state),
                Operator::Add => core_ops::operator::add(state),
                Operator::Sub => core_ops::operator::sub(state),
                Operator::Mul => core_ops::operator::mul(state),
                Operator::Div => core_ops::operator::div(state),
                Operator::VariableAssign => core_ops::operator::variable_assign(state),
                Operator::FunctionVariableAssign => {
                    core_ops::operator::function_variable_assign(state)
                }
                Operator::SelfId => core_ops::operator::get_self(state),
                _ => state,
            },
            _ => {
                state.execution_stack.push(expr);
                state
            }
        }
    }

    pub fn evaluate(
        &mut self,
        expr: Vec<Token>,
        mut state: Box<state::State>,
    ) -> Box<state::State> {
        for token in &expr {
            state = self.eval(state, token.clone());
        }
        state
    }
}
