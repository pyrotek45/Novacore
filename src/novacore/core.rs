use std::rc::Rc;

use super::{evaluator::Evaluator, state};

pub type CallBack = fn(state: Box<state::State>, eval: &mut Evaluator) -> Box<state::State>;

#[derive(PartialEq, Clone, Debug)]
pub enum Block {
    Literal(Rc<Vec<Token>>),
    Lambda(Rc<Vec<Token>>),
    Procedure(Rc<Vec<Token>>),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Operator {
    VariableAssign,
    FunctionVariableAssign,

    SelfId,
    Include,
    Recursive,
    AccessCall, // the dot Token::operator

    UserFunctionChain,
    StoreTemp,
    UserFunctionCall,

    Proc,

    Readln,
    Flush,
    Clear,
    Getch,

    Range,

    And,
    Or,
    Not,

    Equals,
    Gtr,
    Lss,

    Neg,
    Mod,
    Pow,
    Sqrt,

    Add,
    Sub,
    Mul,
    Div,

    For,
    Match,
    Break,
    Continue,
    If,

    Let,
    Ret,

    PopStack,

    Dup,

    Random,

    Command,
    Sleep,

    Push,
    Pop,
    Insert,
    Remove,
    Append,

    Return,

    Exit,

    //terminal stuff
    EnableRawMode,
    RawRead,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    // Op
    Identifier(String), // Variables
    Function(usize),    // Built in Op
    Op(Operator),
    UserBlockCall(String), // Block calls

    // Basix Types
    Integer(i128),
    Float(f64),
    String(String),
    Char(char),
    Symbol(char),
    Bool(bool),
    Block(Block),
    List(Rc<Vec<Token>>),
}

impl Token {
    pub fn precedence(&self) -> usize {
        match self {
            Token::Op(Operator::VariableAssign) => 2,
            Token::Op(Operator::And) => 6,
            Token::Op(Operator::Or) => 7,
            Token::Op(Operator::Not) => 8,
            Token::Op(Operator::Equals) | Token::Op(Operator::Gtr) | Token::Op(Operator::Lss) => 9,
            Token::Op(Operator::Add) | Token::Op(Operator::Sub) => 12,
            Token::Op(Operator::Mul) | Token::Op(Operator::Div) | Token::Op(Operator::Mod) => 13,
            Token::Op(Operator::Neg) => 15,
            Token::Op(Operator::UserFunctionCall) => 14,
            _ => 0,
        }
    }

    pub fn is_left_associative(&self) -> bool {
        match self {
            Token::Op(Operator::UserFunctionCall) => false,
            Token::Op(Operator::Neg) => false,
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
            Token::Identifier(id) => format!("Identifier -> {}", &id),
            Token::Function(index) => format!("Function -> {}", &index),
            Token::UserBlockCall(_) => "User Block Call".to_string(),
            Token::Integer(int) => format!("Integer -> {}", &int),
            Token::Float(float) => format!("Float -> {}", &float),
            Token::String(str) => format!("String > {}", &str),
            Token::Char(ch) => format!("Char -> {}", &ch),
            Token::Symbol(s) => format!("Symbol -> {}", &s),
            Token::Bool(b) => format!("Bool -> {}", &b),
            Token::Block(_) => "Block".to_string(),
            Token::List(_) => "List".to_string(),
            Token::Op(operator) => {
                format!("Op -> {:?}", operator)
            }
        }
    }
}
