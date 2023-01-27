use std::rc::Rc;

use hashbrown::HashMap;

use super::evaluator::Evaluator;

//#[derive(PartialEq, Clone, Debug)]
// pub struct LinePos {
//     pub line: usize,
//     pub col: usize,
// }

pub type CallBack = fn(eval: &mut Evaluator);
pub type Instructions = Rc<Vec<Token>>;

#[derive(PartialEq, Clone, Debug)]
pub enum Block {
    Literal(Instructions),
    Lambda(Instructions),
    Function(Instructions),
    List(Instructions),
    Struct(HashMap<String, Token>),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Operator {
    VariableAssign,
    FunctionVariableAssign,
    SelfId,
    AccessCall, // the dot Token::operator
    UserFunctionChain,
    StoreTemp,
    And,
    Or,
    Not,
    Equals,
    Gtr,
    Lss,
    Invert,
    Mod,
    Add,
    Sub,
    Mul,
    Div,
    PopStack,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    // Variables
    Identifier(String),

    // built in functions
    Function(usize),
    FlowFunction(usize),

    // user defined functions
    UserBlockCall(String),
    FlowUserBlockCall(String), // Block calls

    // symbols
    Op(Operator),

    // Basic Types
    Integer(i128),
    Float(f64),
    String(String),
    Char(char),
    Symbol(char),
    Bool(bool),

    // Raw type
    Block(Block),

    // Registor code
    Reg(Vec<usize>),
}

impl Token {
    pub fn get_int(&self) -> i128 {
        match self {
            Token::Integer(num) => *num,
            _ => 0,
        }
    }

    pub fn _get_string(&self) -> &str {
        match self {
            Token::String(value) => value,
            _ => "",
        }
    }

    pub fn get_float(&self) -> f64 {
        match self {
            Token::Float(num) => *num,
            _ => 0.0,
        }
    }

    pub fn precedence(&self) -> usize {
        match self {
            Token::Op(Operator::VariableAssign) => 2,
            Token::Op(Operator::And) => 6,
            Token::Op(Operator::Or) => 7,
            Token::Op(Operator::Not) => 8,
            Token::Op(Operator::Equals) | Token::Op(Operator::Gtr) | Token::Op(Operator::Lss) => 9,
            Token::Op(Operator::Add) | Token::Op(Operator::Sub) => 12,
            Token::Op(Operator::Mul) | Token::Op(Operator::Div) | Token::Op(Operator::Mod) => 13,
            Token::Op(Operator::Invert) => 15,
            _ => 0,
        }
    }

    pub fn is_left_associative(&self) -> bool {
        match self {
            Token::Op(Operator::Invert) => false,
            Token::Op(Operator::Or) => true,
            Token::Op(Operator::And) => true,
            Token::Op(Operator::Not) => true,
            Token::Op(Operator::VariableAssign) => false,
            Token::Op(Operator::Add) | Token::Op(Operator::Sub) => true,
            Token::Op(Operator::Mul) | Token::Op(Operator::Div) | Token::Op(Operator::Mod) => true,
            _ => true,
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            Token::Identifier(block) => block.to_string(),
            Token::Function(block) => format!("Func[{}]", block),
            Token::UserBlockCall(block) => block.to_string(),
            Token::Integer(block) => format!("{}", block),
            Token::Float(block) => format!("{}", block),
            Token::String(block) => block.to_string(),
            Token::Char(block) => format!("{}", block),
            Token::Symbol(block) => format!("{}", block),
            Token::Bool(block) => format!("Bool{}", block),
            Token::Block(block) => match block {
                Block::Literal(block) => {
                    let mut list = String::new();
                    list.push('{');
                    if !block.is_empty() {
                        for item in block.iter() {
                            list.push_str(&item.to_str());
                            list.push(',');
                        }
                        list.pop();
                        list.push('}');
                    } else {
                        list.push('}');
                    }
                    list.to_string()
                }
                Block::Lambda(block) => {
                    let mut list = String::new();
                    list.push_str("L{");
                    if !block.is_empty() {
                        for item in block.iter() {
                            list.push_str(&item.to_str());
                            list.push(',');
                        }
                        list.pop();
                        list.push('}');
                    } else {
                        list.push('}');
                    }
                    list.to_string()
                }
                Block::Function(block) => {
                    let mut list = String::new();
                    list.push_str("Func{");
                    if !block.is_empty() {
                        for item in block.iter() {
                            list.push_str(&item.to_str());
                            list.push(',');
                        }
                        list.pop();
                        list.push('}');
                    } else {
                        list.push('}');
                    }
                    list.to_string()
                }
                Block::List(block) => {
                    let mut list = String::new();
                    list.push('[');
                    if !block.is_empty() {
                        for item in block.iter() {
                            list.push_str(&item.to_str());
                            list.push(',');
                        }
                        list.pop();
                        list.push(']');
                    } else {
                        list.push(']');
                    }
                    list.to_string()
                }
                Block::Struct(block) => {
                    let mut list = String::new();
                    list.push_str("S{");
                    if !block.is_empty() {
                        for (key, value) in block.iter() {
                            list.push_str(key);
                            list.push_str(" => ");
                            list.push_str(&value.to_str());
                            list.push(',');
                        }
                        list.pop();
                        list.push('}');
                    } else {
                        list.push('}');
                    }
                    list.to_string()
                }
            },
            Token::Op(operator) => {
                let op = operator;
                format!("{:?}", op)
            }
            Token::FlowFunction(block) => format!("{}", block),
            Token::FlowUserBlockCall(block) => format!("{:?}", block),
            Token::Reg(block) => format!("R{:?}", block),
        }
    }

    pub fn to_str_debug(&self) -> String {
        match self {
            Token::Identifier(_) => format!("{:?}", self),
            Token::Function(_) => format!("{:?}", self),
            Token::UserBlockCall(_) => format!("{:?}", self),
            Token::Integer(_) => format!("{:?}", self),
            Token::Float(_) => format!("{:?}", self),
            Token::String(_) => format!("{:?}", self),
            Token::Char(_) => format!("{:?}", self),
            Token::Symbol(_) => format!("{:?}", self),
            Token::Bool(_) => format!("{:?}", self),
            Token::Block(_) => format!("{:?}", self),
            Token::Op(_) => format!("{:?}", self),
            Token::FlowFunction(_) => format!("{:?}", self),
            Token::FlowUserBlockCall(_) => format!("{:?}", self),
            Token::Reg(_) => format!("{:?}", self),
        }
    }
}
