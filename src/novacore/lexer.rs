use std::rc::Rc;

use hashbrown::HashMap;

use super::{
    core::{Block, Operator, Token},
    utilities::is_string_number,
};

pub struct Lexer {
    // File and Token buffer
    source: String,
    buffer: String,

    // Operator added
    function_list: HashMap<String, usize>,

    // State
    is_parsing_list: bool,
    is_parsing_stringdq: bool,
    is_parsing_stringsq: bool,
    is_parsing_comment: bool,
    is_skip: bool,
    is_parsing_chain: bool,

    // Output
    pub block_stack: Vec<Vec<Token>>,
}

impl Lexer {
    // Creates a lexer using the file as input
    pub fn new_from_file(filename: &str) -> Self {
        if let Ok(content) = std::fs::read_to_string(filename) {
            Lexer {
                source: content,
                buffer: String::new(),
                is_parsing_stringdq: false,
                is_parsing_stringsq: false,
                block_stack: vec![vec![]],
                is_parsing_comment: false,
                is_skip: false,
                is_parsing_chain: false,
                function_list: HashMap::new(),
                is_parsing_list: false,
            }
        } else {
            println!(
                "ERROR: file: {} could not be found. Exiting with error code 1",
                filename
            );
            std::process::exit(1);
        }
    }

    // Creates a lexer using a string as input
    pub fn new_from_string(input: &str) -> Self {
        Lexer {
            source: input.to_string(),
            buffer: String::new(),
            is_parsing_stringdq: false,
            is_parsing_stringsq: false,
            block_stack: vec![vec![]],
            is_parsing_comment: false,
            is_skip: false,
            is_parsing_chain: false,
            function_list: HashMap::new(),
            is_parsing_list: false,
        }
    }

    pub fn new() -> Self {
        Lexer {
            source: "".to_string(),
            buffer: String::new(),
            is_parsing_stringdq: false,
            is_parsing_stringsq: false,
            block_stack: vec![vec![]],
            is_parsing_comment: false,
            is_skip: false,
            is_parsing_chain: false,
            function_list: HashMap::new(),
            is_parsing_list: false,
        }
    }

    pub fn insert_string(&mut self, input: &str) {
        self.source += input
    }

    pub fn add_function(&mut self, name: &str, index: usize) {
        self.function_list.insert(name.to_string(), index);
    }

    fn match_token(&self, token: &str) -> Token {
        match token {
            "return" => Token::Op(Operator::Return),
            "self" => Token::Op(Operator::SelfId),
            "break" => Token::Op(Operator::Break),
            "continue" => Token::Op(Operator::Continue),
            "if" => Token::Op(Operator::If),
            "for" => Token::Op(Operator::For),
            // keep for now
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            // change to && and ||
            "and" => Token::Op(Operator::And),
            "or" => Token::Op(Operator::Or),
            _ => Token::Identifier(self.buffer.to_lowercase()),
        }
    }

    pub fn clear(&mut self) {
        self.block_stack = vec![vec![]];
    }
    // // This Op is used to check to see if the current
    // // buffer is either a (number,Op,bool,identifier)
    fn check_token(&self) -> Option<Token> {
        // Checking if buffer is numerical
        if !self.buffer.is_empty() {
            if is_string_number(&self.buffer) {
                // Float
                if self.buffer.contains('.') {
                    if let Ok(v) = self.buffer.parse() {
                        return Some(Token::Float(v));
                    }
                } else {
                    // Int
                    if let Ok(v) = self.buffer.parse() {
                        return Some(Token::Integer(v));
                    }
                }
            } else {
                return Some(self.match_token(&self.buffer.to_lowercase()));
            }
        }
        Option::None
    }

    // // Going through each char in the file or string
    pub fn parse(&mut self) -> Vec<Token> {
        // Parsing strings double quote
        for c in self.source.chars() {
            if self.is_parsing_stringdq {
                if c == '\\' {
                    self.is_skip = true;
                    continue;
                }
                if c != '"' || self.is_skip {
                    self.buffer.push(c);
                    if self.is_skip {
                        self.is_skip = false;
                    }
                    continue;
                } else {
                    self.is_parsing_stringdq = false;
                    if let Some(vec_last) = self.block_stack.last_mut() {
                        if self.buffer.chars().count() == 1 {
                            if let Some(mychar) = self.buffer.chars().next() {
                                vec_last.push(Token::Char(mychar))
                            }
                        } else {
                            vec_last.push(Token::String(self.buffer.clone()))
                        }
                    }
                    self.buffer.clear();
                    continue;
                }
            }

            // Parsing comments
            if self.is_parsing_comment {
                if c != '\n' {
                    continue;
                } else {
                    self.is_parsing_comment = false;
                    if let Some(vec_last) = self.block_stack.last_mut() {
                        if let Some(last) = vec_last.last() {
                            if &Token::Symbol(';') != last {
                                vec_last.push(Token::Symbol(';'))
                            }
                        }
                    }
                    continue;
                }
            }

            // Main parsing Op going through each char and adding them to a buffer
            // if no match is found
            match c {
                // Newline
                '\n' => {
                    if let Some(t) = self.check_token() {
                        if let Some(vec_last) = self.block_stack.last_mut() {
                            vec_last.push(t)
                        }
                        self.buffer.clear();
                    }
                    if !self.is_parsing_list {
                        if let Some(vec_last) = self.block_stack.last_mut() {
                            if let Some(last) = vec_last.last() {
                                if &Token::Symbol(';') != last {
                                    vec_last.push(Token::Symbol(';'))
                                }
                            }
                        }
                    }
                    continue;
                }

                // Comment
                '#' => {
                    if let Some(t) = self.check_token() {
                        if let Some(vec_last) = self.block_stack.last_mut() {
                            vec_last.push(t)
                        }
                        self.buffer.clear();
                    }
                    self.is_parsing_comment = true;
                }

                // Letters and numbers
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                    self.buffer.push(c);
                }

                // Spaces
                ' ' => {
                    if let Some(t) = self.check_token() {
                        if let Some(vec_last) = self.block_stack.last_mut() {
                            vec_last.push(t)
                        }
                        self.buffer.clear();
                    }
                }
                '.' => {
                    if is_string_number(&self.buffer) && !(&self.buffer.contains('.')) {
                        self.buffer.push(c);
                        continue;
                    }

                    if let Some(t) = self.check_token() {
                        if let Some(vec_last) = self.block_stack.last_mut() {
                            vec_last.push(t);
                            vec_last.push(Token::Op(Operator::AccessCall))
                        }
                        self.buffer.clear();
                    }
                }

                // Symbols
                '+' | '-' | '*' | '/' | '(' | ')' | '<' | '>' | '`' | '~' | '@' | '%' | '^'
                | '&' | ',' | '?' | ';' | ':' | '=' | '!' | '$' => {
                    if let Some(t) = self.check_token() {
                        if let Some(vec_last) = self.block_stack.last_mut() {
                            vec_last.push(t)
                        }
                        self.buffer.clear();
                    }

                    if let Some(vec_last) = self.block_stack.last_mut() {
                        match c {
                            ':' => {
                                if let Some(ref last) = vec_last.pop() {
                                    match &last {
                                        Token::Identifier(ident) => {
                                            if let Some(index) = self.function_list.get(ident) {
                                                vec_last.push(Token::Function(*index));
                                                continue;
                                            } else {
                                                vec_last.push(Token::UserBlockCall(ident.clone()));
                                                vec_last.push(Token::Symbol(c));
                                                continue;
                                            }
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
                            ')' => {
                                if self.is_parsing_chain {
                                    vec_last.push(Token::Op(Operator::UserFunctionChain));
                                    self.is_parsing_chain = false;
                                }

                                vec_last.push(Token::Symbol(c));
                            }
                            '-' => {
                                if let Some(last) = vec_last.pop() {
                                    match last {
                                        Token::Identifier(_) => {
                                            vec_last.push(last);
                                            vec_last.push(Token::Op(Operator::Sub));
                                            continue;
                                        }
                                        Token::Integer(_) => {
                                            vec_last.push(last);
                                            vec_last.push(Token::Op(Operator::Sub));
                                            continue;
                                        }
                                        Token::Float(_) => {
                                            vec_last.push(last);
                                            vec_last.push(Token::Op(Operator::Sub));
                                            continue;
                                        }
                                        _ => {
                                            vec_last.push(last);
                                            vec_last.push(Token::Op(Operator::Neg))
                                        }
                                    }
                                } else {
                                    vec_last.push(Token::Op(Operator::Neg))
                                }
                            }
                            '(' => {
                                if let Some(ref last) = vec_last.pop() {
                                    match &last {
                                        Token::Identifier(ident) => {
                                            if let Some(index) = self.function_list.get(ident) {
                                                vec_last.push(Token::Function(*index));
                                                continue;
                                            } else {
                                                vec_last.push(Token::UserBlockCall(ident.clone()));
                                                vec_last.push(Token::Symbol(c));
                                                continue;
                                            }
                                        }
                                        Token::Symbol(')') => {
                                            vec_last.push(last.clone());
                                            self.is_parsing_chain = true;
                                            vec_last.push(Token::Op(Operator::StoreTemp));
                                            vec_last.push(Token::Symbol(c));

                                            continue;
                                        }
                                        Token::Block(Block::Literal(block)) => {
                                            vec_last
                                                .push(Token::Block(Block::Lambda(block.clone())));
                                            vec_last.push(Token::Symbol(c));
                                            continue;
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
                            // '&' => {
                            //     if let Some(last) = vec_last.pop() {
                            //         match last.Token {
                            //             Token::Symbol('&') => {
                            //                 vec_last.push(Token {
                            //                     Token: Token::Op(Operator::UserMacroChain),
                            //                 });
                            //                 continue;
                            //             }
                            //             _ => {
                            //                 vec_last.push(last);
                            //                 vec_last.push(Token {
                            //                     Token: Token::Symbol(c),
                            //                 })
                            //             }
                            //         }
                            //     }
                            // }
                            '<' => {
                                if let Some(last) = vec_last.pop() {
                                    match last {
                                        Token::Op(Operator::Lss) => {
                                            vec_last.push(Token::Op(Operator::PopStack));
                                            continue;
                                        }
                                        _ => {
                                            vec_last.push(last);
                                            vec_last.push(Token::Op(Operator::Lss))
                                        }
                                    }
                                }
                            }
                            '>' => {
                                if let Some(last) = vec_last.pop() {
                                    match last {
                                        // Token::Op(Operator::Neg) => {
                                        //     vec_last.push(Token {
                                        //         Token: Token::Op(Operator::UserOpCall),
                                        //     });
                                        //     continue;
                                        // }
                                        Token::Op(Operator::Gtr) => {
                                            vec_last.push(Token::Op(Operator::Dup));
                                            continue;
                                        }
                                        _ => {
                                            vec_last.push(last);
                                            vec_last.push(Token::Op(Operator::Gtr))
                                        }
                                    }
                                }
                            }
                            '!' => vec_last.push(Token::Op(Operator::Not)),
                            '%' => vec_last.push(Token::Op(Operator::Mod)),
                            '/' => vec_last.push(Token::Op(Operator::Div)),
                            '*' => vec_last.push(Token::Op(Operator::Mul)),
                            '+' => vec_last.push(Token::Op(Operator::Add)),
                            // '@' => vec_last.push(Token {
                            //     Token: Token::Op(Operator::UserOpCall),
                            // }),
                            // '$' => vec_last.push(Token {
                            //     Token: Token::Op(Operator::UserMacroCall),
                            // }),
                            '~' => vec_last.push(Token::Op(Operator::FunctionVariableAssign)),
                            // '?' => vec_last.push(Token {
                            //     Token: Token::Op(Operator::MacroVariableAssign),
                            // }),
                            '=' => {
                                if let Some(last) = vec_last.pop() {
                                    match last {
                                        Token::Op(Operator::VariableAssign) => {
                                            vec_last.push(Token::Op(Operator::Equals));
                                            continue;
                                        }
                                        _ => {
                                            vec_last.push(last);
                                            vec_last.push(Token::Op(Operator::VariableAssign))
                                        }
                                    }
                                }
                            }
                            _ => vec_last.push(Token::Symbol(c)),
                        }
                    }
                }

                // Double quotes (start parsing a string)
                '"' => {
                    if let Some(t) = self.check_token() {
                        if let Some(vec_last) = self.block_stack.last_mut() {
                            vec_last.push(t)
                        }
                        self.buffer.clear();
                    }
                    self.is_parsing_stringdq = true;
                }

                // Single quotes (starts parsing a string)
                '\'' => {
                    if let Some(t) = self.check_token() {
                        if let Some(vec_last) = self.block_stack.last_mut() {
                            vec_last.push(t)
                        }
                        self.buffer.clear();
                    }
                    self.is_parsing_stringsq = true;
                }

                // Parsing blocks
                '{' => {
                    if let Some(t) = self.check_token() {
                        if let Some(vec_last) = self.block_stack.last_mut() {
                            vec_last.push(t)
                        }
                        self.buffer.clear();
                    }

                    self.block_stack.push(vec![]);
                }

                '}' => {
                    if let Some(t) = self.check_token() {
                        if let Some(vec_last) = self.block_stack.last_mut() {
                            vec_last.push(t)
                        }
                        self.buffer.clear();
                    };

                    if let Some(list) = self.block_stack.pop() {
                        if let Some(vec_last) = self.block_stack.last_mut() {
                            vec_last.push(Token::Block(Block::Literal(Rc::new(list))));
                        }
                    }
                }

                //Parsing blocks
                '[' => {
                    if let Some(t) = self.check_token() {
                        if let Some(vec_last) = self.block_stack.last_mut() {
                            vec_last.push(t)
                        }
                        self.buffer.clear();
                    }
                    self.is_parsing_list = true;
                    self.block_stack.push(vec![]);
                }

                ']' => {
                    if let Some(t) = self.check_token() {
                        if let Some(vec_last) = self.block_stack.last_mut() {
                            vec_last.push(t)
                        }
                        self.buffer.clear();
                    };

                    self.is_parsing_list = false;
                    if let Some(list) = self.block_stack.pop() {
                        if let Some(vec_last) = self.block_stack.last_mut() {
                            vec_last.push(Token::List(Rc::new(list)))
                        }
                    }
                }

                _ => println!("what the"),
            }
        }

        // Add char to the buffer
        if let Some(t) = self.check_token() {
            if let Some(vec_last) = self.block_stack.last_mut() {
                vec_last.push(t)
            }
            self.buffer.clear();
        };

        self.block_stack[0].to_owned()
    }
}
