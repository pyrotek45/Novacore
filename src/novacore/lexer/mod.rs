use std::{rc::Rc, vec};

use crate::novacore::utilities::print_line;
use colored::Colorize;
use hashbrown::HashMap;

use super::{
    core::{Block, Operator, Token},
    utilities::is_string_number,
};

pub struct Lexer {
    file: String,
    filename: String,
    token_buffer: String,

    function_list: HashMap<String, usize>,

    // State
    is_parsing_stringdq: bool,
    is_parsing_stringsq: bool,
    is_parsing_comment: bool,
    is_skip: bool,

    // Output
    tokens: Vec<Vec<Token>>,

    line: usize,
    _col: usize,

    // Error handling
    curly: Vec<usize>,
    paren: Vec<usize>,
    sqaure: Vec<usize>,
    stringpair: Vec<usize>,
    bindpair: Vec<usize>,
}

pub fn new() -> Lexer {
    Lexer {
        file: "".to_string(),
        token_buffer: String::new(),
        is_parsing_stringdq: false,
        is_parsing_stringsq: false,
        tokens: vec![vec![]],
        is_parsing_comment: false,
        is_skip: false,
        function_list: HashMap::new(),
        line: 1,
        _col: 1,
        curly: vec![],
        filename: "".to_string(),
        paren: vec![],
        sqaure: vec![],
        stringpair: vec![],
        bindpair: vec![],
    }
}

impl Lexer {
    pub fn get_function_list(&self) -> HashMap<String, usize> {
        self.function_list.clone()
    }

    pub fn add_file(&mut self, filename: &str) {
        self.filename = filename.to_owned();
        if let Ok(content) = std::fs::read_to_string(filename) {
            self.file = content;
        } else {
            println!(
                "ERROR: file: {} could not be found. Exiting with error code 1",
                filename
            );
            std::process::exit(1);
        }
    }

    pub fn insert_string(&mut self, input: &str) {
        self.file += input
    }

    pub fn add_function(&mut self, name: &str, index: usize) {
        self.function_list.insert(name.to_string(), index);
    }

    fn match_token(&self, token: &str) -> Token {
        match token {
            "break" => Token::Op(Operator::Break, self.line),
            "continue" => Token::Op(Operator::Continue, self.line),

            "new" => Token::Op(Operator::New, self.line),

            "true" => Token::Bool(true),
            "false" => Token::Bool(false),

            "and" => Token::Op(Operator::And, self.line),
            "or" => Token::Op(Operator::Or, self.line),

            _ => {
                if token.contains('.') {
                    println!();
                    println!("{}: Is not a valid FLoat", "LEXING ERROR".red());
                    std::process::exit(1)
                }
                Token::Id(self.token_buffer.to_lowercase())
            }
        }
    }

    pub fn clear(&mut self) {
        self.tokens = vec![vec![]];
    }

    // // This Op is used to check to see if the current
    // // buffer is either a (number,Op,bool,identifier)
    fn check_token_buffer(&self) -> Option<Token> {
        if !self.token_buffer.is_empty() {
            if is_string_number(&self.token_buffer) {
                // Float
                if self.token_buffer.contains('.') {
                    if let Ok(v) = self.token_buffer.parse() {
                        return Some(Token::Float(v));
                    }
                } else {
                    // Int
                    if let Ok(v) = self.token_buffer.parse() {
                        return Some(Token::Integer(v));
                    }
                }
            } else {
                return Some(self.match_token(&self.token_buffer.to_lowercase()));
            }
        }
        Option::None
    }

    pub fn check_token(&mut self) {
        if let Some(t) = self.check_token_buffer() {
            if let Some(vec_last) = self.tokens.last_mut() {
                vec_last.push(t)
            }
            self.token_buffer.clear();
        }
    }

    fn add_token(&mut self, token: Token) {
        if let Some(vec_last) = self.tokens.last_mut() {
            vec_last.push(token)
        }
    }

    fn last_token(&self) -> Option<&Token> {
        if let Some(vec_last) = self.tokens.last() {
            vec_last.last()
        } else {
            None
        }
    }

    // // Going through each char in the file or string
    #[inline(always)]
    pub fn parse(&mut self) -> Vec<Token> {
        for c in self.file.clone().chars() {
            if self.is_parsing_stringdq {
                if c == '\\' {
                    self.is_skip = true;
                    continue;
                }
                if c != '"' || self.is_skip {
                    self.token_buffer.push(c);
                    if self.is_skip {
                        self.is_skip = false;
                    }
                    continue;
                } else {
                    self.stringpair.pop();
                    self.is_parsing_stringdq = false;
                    if let Some(vec_last) = self.tokens.last_mut() {
                        if self.token_buffer.chars().count() == 1 {
                            if let Some(mychar) = self.token_buffer.chars().next() {
                                vec_last.push(Token::Char(mychar))
                            }
                        } else {
                            vec_last.push(Token::String(self.token_buffer.clone()))
                        }
                    }
                    self.token_buffer.clear();
                    continue;
                }
            }

            // Parsing comments
            if self.is_parsing_comment {
                if c != '\n' {
                    continue;
                } else {
                    self.is_parsing_comment = false;
                    self.add_token(Token::Symbol(','));
                    self.line += 1;
                    continue;
                }
            }

            // Main parsing Op going through each char and adding them to a buffer
            // if no match is found
            match c {
                // Newline
                '\n' => {
                    self.check_token();
                    self.add_token(Token::Symbol(','));
                    self.line += 1;
                    continue;
                }

                //Comment
                '#' => {
                    self.check_token();
                    self.is_parsing_comment = true;
                }

                // Letters and numbers
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                    self.token_buffer.push(c);
                }

                // Spaces
                ' ' => {
                    self.check_token();
                }

                '.' => {
                    if is_string_number(&self.token_buffer) && !(&self.token_buffer.contains('.')) {
                        self.token_buffer.push(c);
                        continue;
                    }

                    if let Some(t) = self.check_token_buffer() {
                        if let Some(vec_last) = self.tokens.last_mut() {
                            vec_last.push(t);
                            vec_last.push(Token::Op(Operator::AccessCall, self.line))
                        }
                        self.token_buffer.clear();
                    } else if let Some(vec_last) = self.tokens.last_mut() {
                        vec_last.push(Token::Op(Operator::AccessCall, self.line))
                    }
                }

                // Symbols
                '+' | '-' | '*' | '/' | '(' | ')' | '<' | '>' | '`' | '~' | '@' | '%' | '^'
                | '&' | ',' | '?' | ';' | ':' | '=' | '!' | '$' | '|' => {
                    self.check_token();

                    if let Some(vec_last) = self.tokens.last_mut() {
                        match c {
                            ':' => {
                                if let Some(last) = vec_last.pop() {
                                    match last {
                                        Token::Symbol(':') => {
                                            vec_last
                                                .push(Token::Op(Operator::ModuleCall, self.line));
                                            continue;
                                        }
                                        _ => {
                                            vec_last.push(last);
                                            vec_last.push(Token::Symbol(':'));
                                            continue;
                                        }
                                    }
                                } else {
                                    vec_last.push(Token::Symbol(':'))
                                }
                            }
                            ')' => {
                                self.paren.pop();
                                vec_last.push(Token::Symbol(c));
                            }
                            '-' => {
                                if let Some(last) = vec_last.pop() {
                                    match last {
                                        Token::Id(_) => {
                                            vec_last.push(last);
                                            vec_last.push(Token::Op(Operator::Sub, self.line));
                                            continue;
                                        }
                                        Token::Integer(_) => {
                                            vec_last.push(last);
                                            vec_last.push(Token::Op(Operator::Sub, self.line));
                                            continue;
                                        }
                                        Token::Float(_) => {
                                            vec_last.push(last);
                                            vec_last.push(Token::Op(Operator::Sub, self.line));
                                            continue;
                                        }
                                        Token::Symbol(')') => {
                                            vec_last.push(last);
                                            vec_last.push(Token::Op(Operator::Sub, self.line));
                                            continue;
                                        }
                                        _ => {
                                            vec_last.push(last);
                                            vec_last.push(Token::Op(Operator::Neg, self.line))
                                        }
                                    }
                                } else {
                                    vec_last.push(Token::Op(Operator::Neg, self.line))
                                }
                            }
                            '(' => {
                                self.paren.push(self.line);
                                if let Some(ref last) = vec_last.pop() {
                                    match &last {
                                        Token::Id(ident) => {
                                            if let Some(index) = self.function_list.get(ident) {
                                                vec_last.push(Token::Function(*index, self.line));
                                                vec_last.push(Token::Symbol(c));
                                                continue;
                                            } else {
                                                // check if accesscall is
                                                if let Some(Token::Op(Operator::AccessCall, _)) =
                                                    vec_last.last()
                                                {
                                                    vec_last.push(Token::Id(ident.clone()));
                                                    vec_last.push(Token::Op(
                                                        Operator::StoreTemp,
                                                        self.line,
                                                    ));
                                                    vec_last.push(Token::Symbol(c));
                                                    continue;
                                                }
                                                // check if accesscall is
                                                if let Some(Token::Op(Operator::ModuleCall, _)) =
                                                    vec_last.last()
                                                {
                                                    //self.is_parsing_chain.push(true);
                                                    vec_last.push(Token::Id(ident.clone()));
                                                    vec_last.push(Token::Op(
                                                        Operator::StoreTemp,
                                                        self.line,
                                                    ));
                                                    vec_last.push(Token::Symbol(c));
                                                    continue;
                                                }

                                                vec_last.push(Token::BlockCall(
                                                    ident.clone(),
                                                    self.line,
                                                ));
                                                vec_last.push(Token::Symbol(c));
                                                continue;
                                            }
                                        }
                                        Token::Symbol(')') => {
                                            vec_last.push(last.clone());
                                            vec_last
                                                .push(Token::Op(Operator::StoreTemp, self.line));
                                            vec_last.push(Token::Symbol(c));
                                            continue;
                                        }
                                        Token::Op(Operator::ResolveBind, _) => {
                                            vec_last.push(last.clone());
                                            vec_last
                                                .push(Token::Op(Operator::StoreTemp, self.line));
                                            vec_last.push(Token::Symbol(c));
                                            continue;
                                        }
                                        Token::Op(Operator::StoreTemp, _) => {
                                            vec_last.push(last.clone());
                                            vec_last.push(Token::Symbol(c));
                                            continue;
                                        }
                                        Token::Block(Block::Literal(block)) => {
                                            vec_last
                                                .push(Token::Block(Block::Lambda(block.clone())));
                                            vec_last.push(Token::Symbol(c));
                                            continue;
                                        }
                                        Token::Integer(ident) => {
                                            if let Some(Token::Op(Operator::AccessCall, _)) =
                                                vec_last.last()
                                            {
                                                vec_last.push(Token::Integer(*ident));
                                                vec_last.push(Token::Op(
                                                    Operator::StoreTemp,
                                                    self.line,
                                                ));
                                                vec_last.push(Token::Symbol(c));
                                                continue;
                                            }

                                            vec_last.push(last.clone());
                                            vec_last.push(Token::Symbol(c))
                                        }
                                        _ => {
                                            vec_last.push(last.clone());
                                            vec_last.push(Token::Symbol(c))
                                        }
                                    }
                                } else {
                                    vec_last.push(Token::Symbol(c))
                                }
                            }
                            '<' => self.add_token(Token::Op(Operator::Lss, self.line)),
                            '>' => {
                                if let Some(last) = vec_last.pop() {
                                    match last {
                                        Token::Op(Operator::Neg, _) => {
                                            if let Some(Token::Block(Block::List(_))) =
                                                vec_last.last()
                                            {
                                                self.bindpair.push(self.line);
                                                vec_last
                                                    .push(Token::Op(Operator::BindVar, self.line));
                                                continue;
                                            } else {
                                                println!();
                                                println!(
                                                    "{}: Missing list before -> ",
                                                    "LEXING ERROR".red()
                                                );
                                                if let Some(top) = self.stringpair.pop() {
                                                    print_line(top, &self.filename);
                                                }
                                                std::process::exit(1)
                                            }
                                        }
                                        _ => {
                                            vec_last.push(last);
                                            vec_last.push(Token::Op(Operator::Gtr, self.line))
                                        }
                                    }
                                } else {
                                    vec_last.push(Token::Op(Operator::Gtr, self.line))
                                }
                            }
                            '^' => vec_last.push(Token::Op(Operator::ResolveBind, self.line)),
                            '!' => vec_last.push(Token::Op(Operator::Not, self.line)),
                            '%' => vec_last.push(Token::Op(Operator::Mod, self.line)),
                            '/' => vec_last.push(Token::Op(Operator::Div, self.line)),
                            '*' => vec_last.push(Token::Op(Operator::Mul, self.line)),
                            '+' => vec_last.push(Token::Op(Operator::Add, self.line)),
                            '~' => vec_last.push(Token::Op(Operator::Invert, self.line)),
                            ';' => {
                                self.bindpair.pop();
                                vec_last.push(Token::Op(Operator::PopBindings, self.line))
                            }
                            '=' => {
                                if let Some(Token::Op(Operator::VariableAssign, _)) =
                                    vec_last.last()
                                {
                                    vec_last.pop();
                                    vec_last.push(Token::Op(Operator::Equals, self.line))
                                } else {
                                    vec_last.push(Token::Op(Operator::VariableAssign, self.line))
                                }
                            }
                            _ => vec_last.push(Token::Symbol(c)),
                        }
                    }
                }

                // Double quotes (start parsing a string)
                '"' => {
                    self.stringpair.push(self.line);
                    self.check_token();
                    self.is_parsing_stringdq = true;
                }

                // Single quotes (starts parsing a string)
                '\'' => {
                    self.check_token();
                    self.is_parsing_stringsq = true;
                }

                // Parsing blocks
                '{' => {
                    self.curly.push(self.line);
                    self.check_token();
                    if let Some(Token::Op(Operator::VariableAssign, _)) = self.last_token() {
                    } else {
                        self.add_token(Token::Symbol(','));
                    }
                    self.tokens.push(vec![]);
                }

                '}' => {
                    self.curly.pop();
                    self.check_token();
                    if let Some(list) = self.tokens.pop() {
                        self.add_token(Token::Block(Block::Literal(Rc::new(list))));
                    }
                }

                //Parsing raw blocks
                '[' => {
                    self.sqaure.push(self.line);
                    self.check_token();
                    self.tokens.push(vec![]);
                }

                ']' => {
                    self.sqaure.pop();
                    self.check_token();

                    if let Some(list) = self.tokens.pop() {
                        if let Some(vec_last) = self.tokens.last_mut() {
                            if let Some(Token::Symbol('$')) = vec_last.last() {
                                let mut opcodes = vec![];
                                for rso in list {
                                    match rso {
                                        Token::Integer(number) => opcodes.push(number as usize),
                                        _ => todo!(),
                                    }
                                }
                                vec_last.pop();
                                vec_last.push(Token::Reg(opcodes));
                            } else {
                                vec_last.push(Token::Block(Block::List(Rc::new(list))));
                            }
                        }
                    }
                }
                _ => println!("what the FRICK is a {}", c),
            }
        }

        self.check_token();

        // Error checking for missing pairs
        if !self.curly.is_empty() {
            println!();
            println!(
                "{}: Block left open, missing matching {{}}",
                "LEXING ERROR".red()
            );
            if let Some(top) = self.curly.pop() {
                print_line(top, &self.filename);
            }
            std::process::exit(1)
        }

        if !self.paren.is_empty() {
            println!();
            println!(
                "{}: Expression left open, missing matching ()",
                "LEXING ERROR".red()
            );
            if let Some(top) = self.paren.pop() {
                print_line(top, &self.filename);
            }
            std::process::exit(1)
        }

        if !self.sqaure.is_empty() {
            println!();
            println!(
                "{}: List left open, missing matching []",
                "LEXING ERROR".red()
            );
            if let Some(top) = self.sqaure.pop() {
                print_line(top, &self.filename);
            }
            std::process::exit(1)
        }

        if !self.stringpair.is_empty() {
            println!();
            println!(
                "{}: String left open, missing matching \"\" ",
                "LEXING ERROR".red()
            );
            if let Some(top) = self.stringpair.pop() {
                print_line(top, &self.filename);
            }
            std::process::exit(1)
        }

        if !self.bindpair.is_empty() {
            println!();
            println!("{}: Missing ; for -> \"\" ", "LEXING ERROR".red());
            if let Some(top) = self.bindpair.pop() {
                print_line(top, &self.filename);
            }
            std::process::exit(1)
        }

        self.tokens[0].to_owned()
    }
}
