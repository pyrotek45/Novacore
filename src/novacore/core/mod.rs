use std::rc::Rc;

use hashbrown::HashMap;

use super::{evaluator::Evaluator, state};

pub type CallBack = fn(eval: &mut Evaluator);
pub type Instructions = Rc<Vec<Token>>;

#[derive(PartialEq, Clone, Debug)]
pub enum Block {
    Literal(Instructions),
    Lambda(Instructions),
    Function(Instructions),
    Auto(Instructions, Instructions),
    Modifier(Option<String>, Instructions),
    List(Instructions),
    ListLambda(Instructions),
    Struct(HashMap<String, Token>),
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

    // Registor code
    Reg(Vec<usize>),
}

impl Token {
    pub fn get_int(&self) -> i128 {
        match self {
            Token::Integer(num) => return *num,
            _ => 0,
        }
    }

    pub fn get_string(&self) -> &str {
        match self {
            Token::String(value) => return value,
            _ => "",
        }
    }

    pub fn get_float(&self) -> f64 {
        match self {
            Token::Float(num) => return *num,
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

    pub fn to_str_long(&self) -> String {
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
                Block::Literal(list) => format!("LBlock[{}]", list.len()),
                Block::Lambda(_) => "Lambda".to_string(),
                Block::Function(_) => "Function".to_string(),
                Block::Auto(_, _) => "Auto".to_string(),
                Block::Modifier(_, _) => "Modifier".to_string(),
                Block::List(_) => "List".to_string(),
                Block::ListLambda(_) => "ListLambda".to_string(),
                Block::Struct(_) => "Struct".to_string(),
            },
            Token::Op(operator) => {
                format!("Op -> {:?}", operator)
            }
            Token::FlowFunction(index) => {
                format!("FlowFunction -> {}", &index)
            }
            Token::FlowUserBlockCall(_) => "Flow User Block Call".to_string(),
            Token::Reg(opcodes) => format!("Register Operations({:?})", &opcodes),
        }
    }

    pub fn to_str_compact(&self) -> String {
        match self {
            Token::Identifier(id) => format!("ID[{}]", &id),
            Token::Function(index) => format!("F[{}]", &index),
            Token::UserBlockCall(_) => "UBC".to_string(),
            Token::Integer(int) => format!("INT[{}]", &int),
            Token::Float(float) => format!("FL[{}]", &float),
            Token::String(str) => format!("STR[{}]", &str),
            Token::Char(ch) => format!("CHAR[{}]", &ch),
            Token::Symbol(s) => format!("SYM[{}]", &s),
            Token::Bool(b) => format!("BOOL[{}]", &b),
            Token::Block(block) => match block {
                Block::Literal(_) => "LB".to_string(),
                Block::Lambda(_) => "PL".to_string(),
                Block::Function(_) => "F".to_string(),
                Block::Auto(_, _) => "A".to_string(),
                Block::Modifier(_, _) => "MD".to_string(),
                Block::List(_) => "L".to_string(),
                Block::ListLambda(_) => "LL".to_string(),
                Block::Struct(_) => "S".to_string(),
            },
            Token::Op(operator) => {
                format!("O[{:?}]", operator)
            }
            Token::FlowFunction(index) => {
                format!("FLF[{}]", &index)
            }
            Token::FlowUserBlockCall(_) => "FUBC".to_string(),
            Token::Reg(opcodes) => format!("ROP({:?})", &opcodes),
        }
    }
}
