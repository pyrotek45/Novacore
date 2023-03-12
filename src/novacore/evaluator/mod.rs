use std::rc::Rc;

use super::{
    core::{Block, CallBack, Operator, Token},
    core_ops::{self},
    state,
};
use fxhash::FxHashMap as HashMap;

pub struct Evaluator {
    functions: Vec<(CallBack, String)>,
    pub state: state::State,
    pub debug: bool,
}

impl Evaluator {
    pub fn add_function(&mut self, name: String, function: CallBack) -> usize {
        self.functions.push((function, name));
        self.functions.len() - 1
    }

    pub fn eval(&mut self, expr: Token) {
        match expr {
            Token::Reg(opcodes, main) => core_ops::reg::register_operation(self, opcodes, main),
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

                // experimental automatic memoization
                // if self.state.memoize {};

                // if let Some(cache) = self.state.cache.clone() {
                //     //println!("{} -> {}",cache.0,function);
                //     if cache.0 == function {
                //         let lastt = self.state.execution_stack.last().cloned();
                //         let stacksize = self.state.execution_stack.len();

                //         if let Some(Token::Integer(v)) = lastt {
                //             if let Some(t) = self.state.memo.get(&v) {
                //                 //println!("got memo");
                //                 self.state.execution_stack.pop();
                //                 self.state.execution_stack.push(t.clone())
                //             } else {
                //                 match cache.1 {
                //                     Token::Block(Block::Function(input, logic)) => {
                //                         let mut variable_stack: Vec<String> =
                //                             Vec::with_capacity(10);

                //                         for toks in input.iter().rev() {
                //                             if let Token::Id(ident) = &toks {
                //                                 variable_stack.push(ident.clone())
                //                             } else {
                //                                 self.state.show_error(
                //                                     "Can only bind identifiers in a function",
                //                                 )
                //                             }
                //                         }

                //                         // Tie each Token into the call_stack using the tokens poped
                //                         let mut newscope = HashMap::default();
                //                         for tokens in variable_stack {
                //                             if let Some(tok) = self.state.get_from_heap_or_pop() {
                //                                 newscope.insert(tokens, tok.clone());
                //                             } else {
                //                                 self.state.show_error("Not enough arguments")
                //                             }
                //                         }
                //                         self.state.call_stack.push(newscope);
                //                         self.evaluate(logic);
                //                         self.state.call_stack.pop();
                //                     }
                //                     Token::Block(Block::Literal(block)) => self.evaluate(block),
                //                     _ => {}
                //                 }
                //             }
                //         } else {
                //             match cache.1 {
                //                 Token::Block(Block::Function(input, logic)) => {
                //                     let mut variable_stack: Vec<String> = Vec::with_capacity(10);

                //                     for toks in input.iter().rev() {
                //                         if let Token::Id(ident) = &toks {
                //                             variable_stack.push(ident.clone())
                //                         } else {
                //                             self.state.show_error(
                //                                 "Can only bind identifiers in a function",
                //                             )
                //                         }
                //                     }

                //                     // Tie each Token into the call_stack using the tokens poped
                //                     let mut newscope = HashMap::default();
                //                     for tokens in variable_stack {
                //                         if let Some(tok) = self.state.get_from_heap_or_pop() {
                //                             newscope.insert(tokens, tok.clone());
                //                         } else {
                //                             self.state.show_error("Not enough arguments")
                //                         }
                //                     }
                //                     self.state.call_stack.push(newscope);
                //                     self.evaluate(logic);
                //                     self.state.call_stack.pop();
                //                 }
                //                 Token::Block(Block::Literal(block)) => self.evaluate(block),
                //                 _ => {}
                //             }
                //         }

                //         if let Some(Token::Integer(v)) = lastt {
                //             if let Some(t) = self.state.execution_stack.last() {
                //                 if stacksize == self.state.execution_stack.len() {
                //                     //println!("memoized normal");
                //                     self.state.memo.insert(v, t.clone());
                //                 }
                //             }
                //         }

                //     } else if let Some(func) = self.state.get_from_heap(&function) {
                //         let lastt = self.state.execution_stack.last().cloned();
                //         let stacksize = self.state.execution_stack.len();
                //         self.state.cache = Some((function.clone(), func));

                //         //println!("clearing memo cache");
                //         self.state.memo.clear();

                //         core_ops::control::user_block_call(self, &function);

                //         if let Some(Token::Integer(v)) = lastt {
                //             if let Some(t) = self.state.execution_stack.last() {
                //                 if stacksize == self.state.execution_stack.len() {
                //                     //println!("memoized newfunction/memo cache");
                //                     self.state.memo.insert(v, t.clone());
                //                 }
                //             }
                //         }
                //     }
                // } else if let Some(func) = self.state.get_from_heap(&function) {
                //     let lastt = self.state.execution_stack.last().cloned();
                //     let stacksize = self.state.execution_stack.len();
                //     self.state.cache = Some((function.clone(), func));
                //     //println!("caching fucntion {}", function);

                //     //println!("clearing memo cache");
                //     self.state.memo.clear();

                //     core_ops::control::user_block_call(self, &function);

                //     if let Some(Token::Integer(v)) = lastt {
                //         if let Some(t) = self.state.execution_stack.last() {
                //             if stacksize == self.state.execution_stack.len() {
                //                 //println!("memoized first for {}", function);
                //                 self.state.memo.insert(v, t.clone());
                //             }
                //         }
                //     }
                // }

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

    pub fn _get_stack_output(&mut self) -> Option<String> {
        let mut output_string = String::new();
        output_string.push('[');
        for stack_output in self.state.execution_stack.iter() {
            output_string.push_str(&stack_output.to_str());
            output_string.push(',');
        }
        output_string.pop();
        if !output_string.is_empty() {
            output_string.push(']');
            Some(output_string)
        } else {
            None
        }
    }

    pub fn evaluate(&mut self, expr: Rc<Vec<Token>>) {
        for t in &*expr {
            self.eval(t.clone());
            // if let Some(last) = self.get_stack_output() {
            //     println!(" ---> {}", last)
            // }
        }
    }

    pub fn evaluate_function(&mut self, expr: Rc<Vec<Token>>) {
        self.state.call_stack.push(HashMap::default());
        for t in &*expr {
            self.eval(t.clone());
        }
        self.state.call_stack.pop();
    }
}

pub fn new() -> Evaluator {
    Evaluator {
        functions: vec![],
        state: *state::new(),
        debug: false,
    }
}
