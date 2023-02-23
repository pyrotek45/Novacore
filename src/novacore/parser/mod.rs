use super::core::{Operator, Token};
use crate::novacore::core::Block;
use std::rc::Rc;

pub struct Parser {
    pub operator_stack: Vec<Token>,
    pub output_stack: Vec<Token>,
    pub debug: bool,
}

pub fn new() -> Parser {
    Parser {
        operator_stack: Vec::new(),
        output_stack: Vec::new(),
        debug: false,
    }
}

impl Parser {
    pub fn clear(&mut self) {
        self.operator_stack.clear();
        self.output_stack.clear();
    }

    
    pub fn parse_list(&mut self, input: Vec<Token>) -> Vec<Token> {
        for token in input {
            match &token {
                Token::Block(block) => match &block {
                    Block::Literal(shunted) => {
                        let mut np = new();
                        if self.debug {
                            np.debug = true;
                        }

                        self.output_stack.push(Token::Block(Block::Literal(Rc::new(
                            np.parse(shunted.to_vec()),
                        ))));
                    }
                    Block::Lambda(shunted) => {
                        let mut np = new();
                        if self.debug {
                            np.debug = true;
                        }

                        self.operator_stack.push(Token::Block(Block::Lambda(Rc::new(
                            np.parse(shunted.to_vec()),
                        ))));
                    }
                    Block::List(shunted) => {
                        let mut np = new();
                        if self.debug {
                            np.debug = true;
                        }

                        self.output_stack.push(Token::Block(Block::List(Rc::new(
                            np.parse_list(shunted.to_vec()),
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

        self.output_stack.retain(|x| *x != Token::Symbol(' '));
        self.output_stack.retain(|x| *x != Token::Symbol(','));
        self.output_stack.to_owned()
    }

    
    pub fn parse(&mut self, input: Vec<Token>) -> Vec<Token> {
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
                Token::Reg(_) => {
                    self.output_stack.push(token);
                }
                Token::Block(block) => match &block {
                    Block::Literal(shunted) => {
                        let mut np = new();
                        if self.debug {
                            np.debug = true;
                        }

                        self.output_stack.push(Token::Block(Block::Literal(Rc::new(
                            np.parse(shunted.to_vec()),
                        ))));
                    }
                    Block::Lambda(shunted) => {
                        let mut np = new();
                        if self.debug {
                            np.debug = true;
                        }

                        self.operator_stack.push(Token::Block(Block::Lambda(Rc::new(
                            np.parse(shunted.to_vec()),
                        ))));
                    }
                    Block::List(shunted) => {
                        let mut np = new();
                        if self.debug {
                            np.debug = true;
                        }

                        self.output_stack.push(Token::Block(Block::List(Rc::new(
                            np.parse_list(shunted.to_vec()),
                        ))));
                    }
                    _ => {
                        todo!()
                    }
                },
                Token::Symbol(symbol) => {
                    match symbol {
                        ',' => {
                            self.emtpy_operators();
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
                                    Token::Op(Operator::StoreTemp, line) => self
                                        .output_stack
                                        .push(Token::Op(Operator::UserFunctionChain, *line)),
                                    Token::BlockCall(_, _) => self.output_stack.push(last.clone()),
                                    Token::Block(block) => match block {
                                        Block::Literal(_) => todo!(),
                                        Block::Lambda(_) => self.output_stack.push(last.clone()),
                                        Block::Function(_, _) => todo!(),
                                        Block::List(_) => todo!(),
                                        Block::Struct(_) => todo!(),
                                    },
                                    Token::Function(_, _) => self.output_stack.push(last.clone()),
                                    _ => self.operator_stack.push(last.clone()),
                                }
                            }
                        }
                        _ => self.operator_stack.push(token),
                    }
                }
                Token::Id(_) => {
                    self.output_stack.push(token);
                    if let Some(last) = self.operator_stack.last().cloned() {
                        if let Token::Op(function, _) = last.clone() {
                            match function {
                                Operator::AccessCall | Operator::ModuleCall => {
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
                Token::Op(function, _) => match function {
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
                    | Operator::Invert => {
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
                    Operator::PopBindings => {
                        self.emtpy_operators();
                        self.output_stack.push(token)
                    }
                    Operator::UserFunctionChain
                    | Operator::New
                    | Operator::ResolveBind
                    | Operator::BindVar => self.output_stack.push(token),
                    Operator::StoreTemp => {
                        self.operator_stack.push(token.clone());
                        self.output_stack.push(token)
                    }
                    _ => self.operator_stack.push(token),
                },
                Token::Char(_) => self.output_stack.push(token),
                Token::BlockCall(_, _) => self.operator_stack.push(token),
                Token::Function(_, _) => self.operator_stack.push(token),
            }
        }

        self.emtpy_all_operators();
        self.output_stack.to_owned()
    }

    
    fn emtpy_operators(&mut self) {
        while let Some(last) = self.operator_stack.last().cloned() {
            if last != Token::Symbol('(') {
                if let Some(tok) = self.operator_stack.pop() {
                    self.output_stack.push(tok)
                }
            } else {
                break;
            }
        }
    }

    
    fn emtpy_all_operators(&mut self) {
        while let Some(t) = self.operator_stack.pop() {
            self.output_stack.push(t);
        }
    }
}
