use std::rc::Rc;

use hashbrown::HashMap;

use super::{
    core::{Block, CallBack, Operator, Token},
    core_ops, state,
};

pub struct Evaluator {
    functions: Vec<(CallBack, String)>,
    pub state: state::State,
    pub debug: bool,
}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            functions: vec![],
            state: *state::new(),
            debug: false,
        }
    }

    pub fn add_function(&mut self, name: String, function: CallBack) -> usize {
        self.functions.push((function, name));
        self.functions.len() - 1
    }

    pub fn eval(&mut self, expr: Token) {
        match expr {
            Token::Reg(opcodes) => core_ops::reg::register_operation(self, opcodes),
            Token::Function(index, line) => {
                self.state.current_function_index.push(index);
                if self.debug {
                    self.state
                        .traceback
                        .push((self.functions[index].1.clone(), line));
                }
                self.functions[index].0(self);
                self.state.current_function_index.pop();
                if self.debug {
                    self.state.traceback.pop();
                }
            }
            Token::BlockCall(function, line) => {
                if self.debug {
                    self.state.traceback.push((function.clone(), line));
                }

                core_ops::control::user_block_call(self, &function);
                if self.debug {
                    self.state.traceback.pop();
                }
            }
            Token::Block(Block::Lambda(block)) => {
                self.evaluate_function(block);
            }
            Token::Op(ref operator, line) => {
                if self.debug {
                    self.state.traceback.push((expr.to_str(), line));
                }
                match operator {
                    Operator::BindVar => core_ops::operator::bind_variables(self),
                    Operator::ResolveBind => core_ops::operator::resolve_binding(self),
                    Operator::PopBindings => core_ops::operator::pop_bindings(self),
                    Operator::Break => core_ops::control::break_loop(self),
                    Operator::Continue => core_ops::control::continue_loop(self),
                    Operator::Neg => core_ops::operator::neg(self),
                    Operator::AccessCall => core_ops::control::get_access(self),
                    Operator::UserFunctionChain => core_ops::control::user_chain_call(self),
                    Operator::StoreTemp => core_ops::control::store_temp(self),
                    Operator::And => core_ops::logical::logical_and(self),
                    Operator::Or => core_ops::logical::logical_or(self),
                    Operator::Not => core_ops::logical::logical_not(self),
                    Operator::Equals => core_ops::comparison::equality_comparison(self),
                    Operator::Gtr => core_ops::comparison::greater_than_comparison(self),
                    Operator::Lss => core_ops::comparison::less_than_comparison(self),
                    Operator::Invert => core_ops::operator::neg(self),
                    Operator::Mod => core_ops::operator::modulo(self),
                    Operator::Add => core_ops::operator::add(self),
                    Operator::Sub => core_ops::operator::sub(self),
                    Operator::Mul => core_ops::operator::mul(self),
                    Operator::Div => core_ops::operator::div(self),
                    Operator::VariableAssign => core_ops::operator::variable_assign(self),
                    Operator::New => core_ops::operator::get_new(self),
                    Operator::ModuleCall => core_ops::control::module(self),
                }
                if self.debug {
                    self.state.traceback.pop();
                }
            }
            Token::Symbol(_) => {}
            Token::Id(_) => self.state.execution_stack.push(expr),
            _ => {
                self.state.execution_stack.push(expr);
            }
        }
    }

    pub fn evaluate(&mut self, expr: Rc<Vec<Token>>) {
        for t in &*expr {
            self.eval(t.clone())
        }
    }

    pub fn evaluate_function(&mut self, expr: Rc<Vec<Token>>) {
        self.state.call_stack.push(HashMap::new());
        for t in &*expr {
            self.eval(t.clone());
        }
        self.state.call_stack.pop();
    }
}
