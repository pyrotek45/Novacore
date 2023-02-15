use std::rc::Rc;

use hashbrown::HashMap;

use super::evaluator::Evaluator;

pub type CallBack = fn(eval: &mut Evaluator);
pub type Instructions = Rc<Vec<Token>>;

#[derive(PartialEq, Clone, Debug)]
pub enum Block {
    Literal(Instructions),
    Lambda(Instructions),
    Function(Instructions, Instructions),
    List(Instructions),
    Struct(Rc<HashMap<String, Token>>),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Operator {
    VariableAssign,
    BindVar,
    New,
    AccessCall,
    ModuleCall,
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
    PopBindings,
    Neg,
    Break,
    Continue,
    ResolveBind,
}

#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    // Variables
    Id(String),

    // built in functions
    Function(usize, usize),

    //FlowFunction(usize),

    // user defined functions
    BlockCall(String, usize),

    //FlowUserBlockCall(String), // Block calls

    // symbols
    Op(Operator, usize),

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
            Token::Op(Operator::VariableAssign, _) => 2,
            Token::Op(Operator::And, _) => 6,
            Token::Op(Operator::Or, _) => 7,
            Token::Op(Operator::Not, _) => 8,
            Token::Op(Operator::Equals, _)
            | Token::Op(Operator::Gtr, _)
            | Token::Op(Operator::Lss, _) => 9,
            Token::Op(Operator::Add, _) | Token::Op(Operator::Sub, _) => 12,
            Token::Op(Operator::Mul, _)
            | Token::Op(Operator::Div, _)
            | Token::Op(Operator::Mod, _) => 13,
            Token::Op(Operator::Invert, _) => 15,
            _ => 0,
        }
    }

    pub fn is_left_associative(&self) -> bool {
        match self {
            Token::Op(Operator::Invert, _) => false,
            Token::Op(Operator::Or, _) => true,
            Token::Op(Operator::And, _) => true,
            Token::Op(Operator::Not, _) => true,
            Token::Op(Operator::VariableAssign, _) => false,
            Token::Op(Operator::Add, _) | Token::Op(Operator::Sub, _) => true,
            Token::Op(Operator::Mul, _)
            | Token::Op(Operator::Div, _)
            | Token::Op(Operator::Mod, _) => true,
            _ => true,
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            Token::Id(block) => block.to_string(),
            Token::Function(block, _) => format!("Func[{}]", block),
            Token::BlockCall(block, _) => block.to_string(),
            Token::Integer(block) => format!("{}", block),
            Token::Float(block) => format!("{}", block),
            Token::String(block) => block.to_string(),
            Token::Char(block) => format!("{}", block),
            Token::Symbol(block) => format!("{}", block),
            Token::Bool(block) => format!("Bool[{}]", block),
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
                Block::Function(_, block) => {
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
            Token::Op(operator, _) => {
                let op = operator;
                format!("{:?}", op)
            }
            //Token::FlowFunction(block) => format!("{}", block),
            //Token::FlowUserBlockCall(block) => format!("{:?}", block),
            Token::Reg(block) => format!("R{:?}", block),
        }
    }

    pub fn to_str_debug(&self) -> String {
        match self {
            Token::Id(_) => format!("{:?}", self),
            Token::Function(_, _) => format!("{:?}", self),
            Token::BlockCall(_, _) => format!("{:?}", self),
            Token::Integer(_) => format!("{:?}", self),
            Token::Float(_) => format!("{:?}", self),
            Token::String(_) => format!("{:?}", self),
            Token::Char(_) => format!("{:?}", self),
            Token::Symbol(_) => format!("{:?}", self),
            Token::Bool(_) => format!("{:?}", self),
            Token::Block(_) => format!("{:?}", self),
            Token::Op(_, _) => format!("{:?}", self),
            //Token::FlowFunction(_) => format!("{:?}", self),
            //Token::FlowUserBlockCall(_) => format!("{:?}", self),
            Token::Reg(_) => format!("{:?}", self),
        }
    }
}
