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

    pub fn shunt(&mut self, input: Vec<Token>) -> Vec<Token> {
        for token in input {
            match &token {
                Token::Integer(_) => {
                    self.output_stack.push(token);
                }
                Token::Float(_) => {
                    self.output_stack.push(token);
                }
                Token::String(_) => {
                    self.output_stack.push(token);
                }
                Token::Bool(_) => {
                    self.output_stack.push(token);
                }

                Token::Block(block) => {
                    match &block {
                        Block::Literal(shunted) => {
                            // Shunt blocks first time
                            let mut np = Parser::new();
                            if self.debug {
                                np.debug = true;
                            }

                            self.output_stack.push(Token::Block(Block::Literal(Rc::new(
                                np.shunt(shunted.to_vec()),
                            ))));
                        }
                        Block::Lambda(shunted) => {
                            // Shunt blocks first time
                            let mut np = Parser::new();
                            if self.debug {
                                np.debug = true;
                            }

                            self.operator_stack.push(Token::Block(Block::Lambda(Rc::new(
                                np.shunt(shunted.to_vec()),
                            ))));
                        }
                        _ => {
                            todo!()
                        }
                    }
                }
                Token::List(_) => {
                    self.output_stack.push(token);
                }

                Token::Symbol(symbol) => {
                    match symbol {
                        ',' => {
                            // pop temp off if its and check if its "("
                            if let Some(temp) = self.operator_stack.pop() {
                                if temp == Token::Symbol('(') {
                                    if let Some(func) = self.operator_stack.last().cloned() {
                                        // if it is function pop function
                                        self.output_stack.push(func)
                                    }
                                }
                                // put temp back
                                self.operator_stack.push(temp)
                            }
                        }
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
                                    Token::Op(fun) => match fun {
                                        // Operator::If => self.operator_stack.push(last.clone()),
                                        // Operator::For => self.operator_stack.push(last.clone()),
                                        _ => {
                                            self.output_stack.push(last.clone());
                                        }
                                    },
                                    Token::UserBlockCall(_) => self.output_stack.push(last.clone()),
                                    Token::Block(Block::Lambda(_)) => {
                                        self.output_stack.push(last.clone())
                                    }
                                    Token::Function(_) => self.output_stack.push(last.clone()),
                                    _ => self.operator_stack.push(last.clone()),
                                }
                            }
                        }
                        ';' => {
                            while let Some(tok) = self.operator_stack.pop() {
                                if tok != Token::Symbol('(') {
                                    self.output_stack.push(tok)
                                } else {
                                    self.operator_stack.push(tok);
                                    break;
                                }
                            }
                        }
                        // Macros
                        // '&' => {
                        //     if let Some(token) = self.output_stack.pop() {
                        //         match token.Token {
                        //             Token::Identifier(ident) => self.operator_stack.push(Token {
                        //                 Token: Token::UserMacro(ident),
                        //             }),
                        //             _ => self.operator_stack.push(token),
                        //         }
                        //     }
                        // }
                        // Functions
                        // ':' => {
                        //     if let Some(token) = self.output_stack.pop() {
                        //         match token.Token {
                        //             Token::Identifier(ident) => self.operator_stack.push(Token {
                        //                 Token: Token::UserFunction(ident),
                        //             }),
                        //             _ => self.operator_stack.push(token),
                        //         }
                        //     }
                        // }
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
                                // Functions::UserMacroCall => {
                                //     self.operator_stack.pop();
                                //     self.output_stack.push(last);
                                // }
                                // Functions::UserFunctionCall => {
                                //     self.operator_stack.pop();
                                //     self.output_stack.push(last);
                                // }
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
                    | Operator::Neg
                    | Operator::UserFunctionCall => {
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

        while let Some(t) = self.operator_stack.pop() {
            if t != Token::Symbol(':') {
                self.output_stack.push(t);
            }
        }

        self.output_stack.to_owned()
    }
}
