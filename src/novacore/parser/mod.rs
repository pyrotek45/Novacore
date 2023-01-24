use std::rc::Rc;

use crate::novacore::core::Block;

use super::core::{Operator, Token};

pub struct Parser {
    pub operator_stack: Vec<Token>,
    pub output_stack: Vec<Token>,
    pub debug: bool,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            operator_stack: Vec::new(),
            output_stack: Vec::new(),
            debug: false,
        }
    }

    pub fn clear(&mut self) {
        self.operator_stack.clear();
        self.output_stack.clear();
    }

    pub fn shunt_list(&mut self, input: Vec<Token>) -> Vec<Token> {
        for token in input {
            match &token {
                Token::Block(block) => match &block {
                    Block::Literal(shunted) => {
                        let mut np = Parser::new();
                        if self.debug {
                            np.debug = true;
                        }

                        self.output_stack.push(Token::Block(Block::Literal(Rc::new(
                            np.shunt(shunted.to_vec()),
                        ))));
                    }
                    Block::Lambda(shunted) => {
                        let mut np = Parser::new();
                        if self.debug {
                            np.debug = true;
                        }

                        self.output_stack.push(Token::Block(Block::Lambda(Rc::new(
                            np.shunt(shunted.to_vec()),
                        ))));
                    }
                    Block::List(shunted) => {
                        let mut np = Parser::new();
                        if self.debug {
                            np.debug = true;
                        }

                        self.output_stack.push(Token::Block(Block::List(Rc::new(
                            np.shunt_list(shunted.to_vec()),
                        ))));
                    }
                    _ => {
                        todo!()
                    }
                },
                _ => {
                    self.output_stack.push(token);
                }
            }
        }

        self.output_stack.retain(|x| *x != Token::Symbol(';'));
        self.output_stack.retain(|x| *x != Token::Symbol(' '));
        self.output_stack.retain(|x| *x != Token::Symbol(','));
        self.output_stack.to_owned()
    }

    pub fn shunt(&mut self, input: Vec<Token>) -> Vec<Token> {
        for token in input {
            match &token {
                Token::Integer(_) => {
                    self.output_stack.push(token);
                    if let Some(last) = self.operator_stack.last().cloned() {
                        if let Token::Op(function) = last.clone() {
                            match function {
                                Operator::AccessCall => {
                                    self.operator_stack.pop();
                                    self.output_stack.push(last);
                                }
                                _ => {
                                    continue;
                                }
                            }
                        }
                    }
                }
                Token::Float(_) => {
                    self.output_stack.push(token);
                    if let Some(last) = self.operator_stack.last().cloned() {
                        if let Token::Op(function) = last.clone() {
                            match function {
                                Operator::AccessCall => {
                                    self.operator_stack.pop();
                                    self.output_stack.push(last);
                                }
                                _ => {
                                    continue;
                                }
                            }
                        }
                    }
                }
                Token::String(_) => {
                    self.output_stack.push(token);
                    if let Some(last) = self.operator_stack.last().cloned() {
                        if let Token::Op(function) = last.clone() {
                            match function {
                                Operator::AccessCall => {
                                    self.operator_stack.pop();
                                    self.output_stack.push(last);
                                }
                                _ => {
                                    continue;
                                }
                            }
                        }
                    }
                }
                Token::Bool(_) => {
                    self.output_stack.push(token);
                    if let Some(last) = self.operator_stack.last().cloned() {
                        if let Token::Op(function) = last.clone() {
                            match function {
                                Operator::AccessCall => {
                                    self.operator_stack.pop();
                                    self.output_stack.push(last);
                                }
                                _ => {
                                    continue;
                                }
                            }
                        }
                    }
                }
                Token::Reg(_) => {
                    self.output_stack.push(token);
                }
                Token::Block(block) => match &block {
                    Block::Literal(shunted) => {
                        let mut np = Parser::new();
                        if self.debug {
                            np.debug = true;
                        }

                        self.output_stack.push(Token::Block(Block::Literal(Rc::new(
                            np.shunt(shunted.to_vec()),
                        ))));
                    }
                    Block::Lambda(shunted) => {
                        let mut np = Parser::new();
                        if self.debug {
                            np.debug = true;
                        }

                        self.operator_stack.push(Token::Block(Block::Lambda(Rc::new(
                            np.shunt(shunted.to_vec()),
                        ))));
                    }
                    Block::ListLambda(shunted) => {
                        let mut np = Parser::new();
                        if self.debug {
                            np.debug = true;
                        }

                        self.operator_stack.push(Token::Block(Block::Lambda(Rc::new(
                            np.shunt_list(shunted.to_vec()),
                        ))));
                    }
                    Block::List(shunted) => {
                        let mut np = Parser::new();
                        if self.debug {
                            np.debug = true;
                        }

                        self.output_stack.push(Token::Block(Block::List(Rc::new(
                            np.shunt_list(shunted.to_vec()),
                        ))));
                    }
                    _ => {
                        todo!()
                    }
                },
                Token::Symbol(symbol) => {
                    match symbol {
                        // ',' => {
                        //     // pop temp off operator stack and check if its "("
                        //     if let Some(temp) = self.operator_stack.pop() {
                        //         if temp == Token::Symbol('(') {
                        //             if let Some(func) = self.operator_stack.last().cloned() {
                        //                 self.output_stack.push(func)
                        //             }
                        //         }
                        //         // put temp back
                        //         self.operator_stack.push(temp)
                        //     }
                        // }
                        '(' => {
                            self.operator_stack.push(token);
                        }
                        ')' => {
                            // while the last item in the operator stack is not
                            // a "(", pop off items into output stack
                            while let Some(last) = self.operator_stack.pop() {
                                if last != Token::Symbol('(') {
                                    self.output_stack.push(last)
                                } else {
                                    break;
                                }
                            }

                            // // if last item on operator stack is a function pop
                            // // this is for leapfrog TM parsing
                            if let Some(ref last) = self.operator_stack.pop() {
                                match &last {
                                    Token::Op(_) => {
                                        self.output_stack.push(last.clone());
                                    }
                                    Token::UserBlockCall(_) => self.output_stack.push(last.clone()),
                                    Token::Block(block) => match block {
                                        Block::Literal(_) => todo!(),
                                        Block::Lambda(_) => self.output_stack.push(last.clone()),
                                        Block::Function(_) => todo!(),
                                        Block::Auto(_, _) => todo!(),
                                        Block::Modifier(_, _) => todo!(),
                                        Block::List(_) => todo!(),
                                        Block::ListLambda(_) => {
                                            self.output_stack.push(last.clone())
                                        }
                                        Block::Struct(_) => todo!(),
                                    },
                                    Token::Function(_) => self.output_stack.push(last.clone()),
                                    _ => self.operator_stack.push(last.clone()),
                                }
                            }
                        }
                        ',' => {
                            while let Some(tok) = self.operator_stack.pop() {
                                if tok != Token::Symbol('(') {
                                    self.output_stack.push(tok)
                                } else {
                                    self.operator_stack.push(tok);
                                    break;
                                }
                            }
                        }
                        ';' => {
                            while let Some(tok) = self.operator_stack.pop() {
                                self.output_stack.push(tok)
                            }
                        }
                        _ => self.operator_stack.push(token),
                    }
                }
                Token::Identifier(_) => {
                    self.output_stack.push(token);
                    if let Some(last) = self.operator_stack.last().cloned() {
                        if let Token::Op(function) = last.clone() {
                            match function {
                                Operator::AccessCall => {
                                    self.operator_stack.pop();
                                    self.output_stack.push(last);
                                }
                                _ => {
                                    continue;
                                }
                            }
                        }
                    }
                }
                Token::Op(function) => match function {
                    Operator::Add
                    | Operator::Sub
                    | Operator::Mul
                    | Operator::Div
                    | Operator::Equals
                    | Operator::VariableAssign
                    | Operator::Not
                    | Operator::Mod
                    | Operator::And
                    | Operator::Or
                    | Operator::Gtr
                    | Operator::Lss
                    | Operator::Neg => {
                        //Pop off higher precedence before adding

                        // if last item in operator stack is not a "("
                        // and while last item precedence is > than
                        // current token precedence pop until empty
                        if let Some(temp) = self.operator_stack.last().cloned() {
                            if temp != Token::Symbol('(') {
                                while let Some(op) = self.operator_stack.last() {
                                    if op.precedence() > token.precedence() {
                                        if let Some(t) = self.operator_stack.pop() {
                                            self.output_stack.push(t)
                                        }
                                    } else {
                                        break;
                                    }
                                }

                                // if operator last on the stack is of equal precedence, then pop
                                // until empty
                                while let Some(op) = self.operator_stack.last() {
                                    if op.precedence() == token.precedence()
                                        && token.is_left_associative()
                                    {
                                        if let Some(t) = self.operator_stack.pop() {
                                            self.output_stack.push(t)
                                        }
                                    } else {
                                        break;
                                    }
                                }
                            }
                        }

                        // push token onto operator stack
                        self.operator_stack.push(token);
                        continue;
                    }
                    Operator::StoreTemp
                    | Operator::UserFunctionChain
                    | Operator::SelfId
                    | Operator::FunctionVariableAssign => self.output_stack.push(token),
                    _ => self.operator_stack.push(token),
                },
                Token::Char(_) => self.output_stack.push(token),
                Token::UserBlockCall(_) => self.operator_stack.push(token),
                Token::Function(_) => self.operator_stack.push(token),
                Token::FlowFunction(_) => self.operator_stack.push(token),
                Token::FlowUserBlockCall(_) => self.operator_stack.push(token),
            }
        }

        // pop off all operators, skipping colons
        while let Some(t) = self.operator_stack.pop() {
            if t != Token::Symbol(':') {
                self.output_stack.push(t);
            }
        }

        self.output_stack.retain(|x| *x != Token::Symbol(','));
        self.output_stack.retain(|x| *x != Token::Symbol(';'));
        self.output_stack.retain(|x| *x != Token::Symbol(' '));
        self.output_stack.to_owned()
    }
}
