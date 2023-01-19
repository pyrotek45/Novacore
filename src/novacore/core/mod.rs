use std::rc::Rc;

use super::{evaluator::Evaluator, state};

pub type CallBack = fn(eval: &mut Evaluator);
pub type Instructions = Rc<Vec<Token>>;

#[derive(PartialEq, Clone, Debug)]
pub enum Block {
    Raw(Instructions),
    Parsed(Instructions),
    ParsedLambda(Instructions),
    RawLambda(Instructions),
    Procedure(Instructions),
    Function(Instructions),
    Auto(Instructions, Instructions),
    Modifier(Option<String>, Instructions),
    List(Instructions),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Operator {
    VariableAssign,
    FunctionVariableAssign,

    SelfId,
    Include,
    AccessCall, // the dot Token::operator

    UserFunctionChain,
    StoreTemp,
    UserFunctionCall,

    And,
    Or,
    Not,

    Equals,
    Gtr,
    Lss,

    Neg,
    Mod,

    Add,
    Sub,
    Mul,
    Div,

    Break,
    Continue,

    PopStack,

    Dup,
    Pass,

    //terminal stuff
    EnableRawMode,
    RawRead,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    // Variables
    Identifier(String),

    // built in functions
    Function(usize),
    FlowFunction(usize),

    // symbols
    Op(Operator),

    // user defined functions
    UserBlockCall(String),
    FlowUserBlockCall(String), // Block calls

    // Basic Types
    Integer(i128),
    Float(f64),
    String(String),
    Char(char),
    Symbol(char),
    Bool(bool),

    // Raw type
    Block(Block),
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
            Token::Block(block) => match block {
                Block::Raw(_) => "Raw Block".to_string(),
                Block::Parsed(_) => "Parsed Block".to_string(),
                Block::ParsedLambda(_) => "Parsed Lambda".to_string(),
                Block::Procedure(_) => "Procedure".to_string(),
                Block::Function(_) => "Function".to_string(),
                Block::Auto(_, _) => "Auto".to_string(),
                Block::Modifier(_, _) => "Modifier".to_string(),
                Block::List(_) => "List".to_string(),
                Block::RawLambda(_) => "Raw Lambda".to_string(),
            },
            Token::Op(operator) => {
                format!("Op -> {:?}", operator)
            }
            Token::FlowFunction(index) => {
                format!("FlowFunction -> {}", &index)
            }
            Token::FlowUserBlockCall(_) => "Flow User Block Call".to_string(),
        }
    }

    pub fn to_str_compact(&self) -> String {
        match self {
            Token::Identifier(id) => format!("ID[{}]", &id),
            Token::Function(index) => format!("F[{}]", &index),
            Token::UserBlockCall(_) => "UBC".to_string(),
            Token::Integer(int) => format!("{}", &int),
            Token::Float(float) => format!("{}", &float),
            Token::String(str) => format!("{}", &str),
            Token::Char(ch) => format!("{}", &ch),
            Token::Symbol(s) => format!("{}", &s),
            Token::Bool(b) => format!("{}", &b),
            Token::Block(block) => match block {
                Block::Raw(_) => "RB".to_string(),
                Block::Parsed(_) => "PB".to_string(),
                Block::ParsedLambda(_) => "PL".to_string(),
                Block::Procedure(_) => "PR".to_string(),
                Block::Function(_) => "F".to_string(),
                Block::Auto(_, _) => "A".to_string(),
                Block::Modifier(_, _) => "MD".to_string(),
                Block::List(_) => "L".to_string(),
                Block::RawLambda(_) => "RL".to_string(),
            },
            Token::Op(operator) => {
                format!("O[{:?}]", operator)
            }
            Token::FlowFunction(index) => {
                format!("FLF[{}]", &index)
            }
            Token::FlowUserBlockCall(_) => "FUBC".to_string(),
        }
    }
}
