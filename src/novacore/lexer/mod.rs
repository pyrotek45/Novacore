use super::{super::novacore};
use crate::novacore::{utilities::print_line};
use colored::Colorize;
use fxhash::FxHashMap as HashMap;
use std::{rc::Rc, vec};

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
    charpair: Vec<usize>,
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
        function_list: HashMap::default(),
        line: 1,
        _col: 1,
        curly: vec![],
        filename: "".to_string(),
        paren: vec![],
        sqaure: vec![],
        stringpair: vec![],
        bindpair: vec![],
        charpair: vec![],
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

            "-" => Token::Op(Operator::Sub, self.line),
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
    pub fn parse(&mut self) -> Result<Vec<Token>, &str> {
        for c in self.file.clone().chars() {
            if self.is_parsing_stringsq {
                if c == '\\' {
                    self.is_skip = true;
                    continue;
                }
                if c != '\'' || self.is_skip {
                    self.token_buffer.push(c);
                    if self.is_skip {
                        self.is_skip = false;
                    }
                    continue;
                } else {
                    self.charpair.pop();
                    self.is_parsing_stringsq = false;
                    if let Some(vec_last) = self.tokens.last_mut() {
                        if self.token_buffer.chars().count() == 1 {
                            if let Some(mychar) = self.token_buffer.chars().next() {
                                vec_last.push(Token::Char(mychar))
                            }
                        } else {
                            println!();
                            println!(
                                "{}: Char cannot have more than one character",
                                "LEXING ERROR".red()
                            );
                            std::process::exit(1)
                        }
                    }
                    self.token_buffer.clear();
                    continue;
                }
            }

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
                        // if self.token_buffer.chars().count() == 1 {
                        //     if let Some(mychar) = self.token_buffer.chars().next() {
                        //         vec_last.push(Token::Char(mychar))
                        //     }
                        // } else {
                        vec_last.push(Token::String(self.token_buffer.clone()))
                        //}
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
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' | '-' => {
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
                '+' | '*' | '/' | '(' | ')' | '<' | '>' | '`' | '~' | '@' | '%' | '^' | '&'
                | ',' | '?' | ';' | ':' | '=' | '!' | '$' | '|' => {
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
                                        Token::Op(Operator::Sub, _) => {
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
                    self.charpair.push(self.line);
                    self.check_token();
                    self.is_parsing_stringsq = true;
                }

                // Parsing blocks
                '{' => {
                    self.curly.push(self.line);
                    self.check_token();
                    match self.last_token() {
                        Some(Token::Op(Operator::VariableAssign, _)) => {}
                        Some(Token::Symbol(':')) => {}
                        Some(Token::Symbol('$')) => {}
                        _ => self.add_token(Token::Symbol(',')),
                    }

                    self.tokens.push(vec![]);
                }

                '}' => {
                    self.curly.pop();
                    self.check_token();
                    if let Some(list) = self.tokens.pop() {
                        match self.last_token() {
                            Some(Token::Symbol(':')) => {
                                if let Some(vec_last) = self.tokens.last_mut() {
                                    vec_last.pop();
                                    if let Some(Token::Block(Block::List(inputs))) = vec_last.pop()
                                    {
                                        vec_last.push(Token::Block(Block::Function(
                                            inputs,
                                            Rc::new(list),
                                        )))
                                    } else {
                                        todo!()
                                    }
                                }
                            }
                            Some(Token::Symbol('$')) => {
                                //println!("comptime eval:");
                                let mut vm = novacore::new();
                                vm.init();
                                
                                vm.evaluator.evaluate(Rc::new(vm.parser.parse(list)));
                                if let Some(vec_last) = self.tokens.last_mut() {
                                    vec_last.pop();
                                    for token in vm.evaluator.state.execution_stack.iter() {
                                        vec_last.push(token.clone())
                                    }
                                }
                            }
                            Some(_) => {self.add_token(Token::Block(Block::Literal(Rc::new(list))))}
                            None => self.add_token(Token::Block(Block::Literal(Rc::new(list)))),
                        }
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

                    if let Some(mut list) = self.tokens.pop() {
                        if let Some(vec_last) = self.tokens.last_mut() {
                            if let Some(Token::Symbol('$')) = vec_last.last() {
                                vec_last.pop();
                                if let Some(Token::Symbol(':')) = vec_last.last() {
                                    vec_last.pop();
                                    if let Some(vec_last) = self.tokens.last_mut() {
                                        if let Some(Token::Block(Block::List(inputs))) =
                                            vec_last.pop()
                                        {
                                            list.retain(|x| *x != Token::Symbol(' '));
                                            list.retain(|x| *x != Token::Symbol(','));
                                            let mut unknown_words: Vec<(String, usize)> = vec![];
                                            let mut rjuststack = vec![];
                                            let mut labelindex: Option<usize> = None;
                                            let mut labels = HashMap::default();
                                            let mut opcodes = vec![];
                                            'out: for rso in list.into_iter() {
                                                match rso {
                                                    Token::Symbol('@') => {
                                                        labelindex = Some(opcodes.len());
                                                    }
                                                    Token::Integer(number) => {
                                                        opcodes.push(number as usize)
                                                    }
                                                    id => {
                                                        let mut index = inputs.len();
                                                        for var in inputs.iter() {
                                                            if *var == id {
                                                                opcodes.push(index - 1);
                                                                continue 'out;
                                                            }
                                                            index -= 1;
                                                        }
                                                        if let Token::Id(id) = id {
                                                            match id.as_str() {
                                                                "to" => {
                                                                    //println!("Pushed: {}", currentindex);
                                                                    rjuststack.push(opcodes.len())
                                                                }
                                                                "end" => {
                                                                    if let Some(rplace) =
                                                                        rjuststack.pop()
                                                                    {
                                                                        //println!("placed: {}", currentindex - rplace);
                                                                        opcodes.insert(
                                                                            rplace,
                                                                            opcodes.len() - rplace,
                                                                        )
                                                                    }
                                                                }
                                                                "exit" => opcodes.push(0),

                                                                "iadd" => opcodes.push(1),
                                                                "isub" => opcodes.push(2),
                                                                "imul" => opcodes.push(3),

                                                                "fadd" => opcodes.push(4),
                                                                "fsub" => opcodes.push(5),
                                                                "fmul" => opcodes.push(6),
                                                                "fdiv" => opcodes.push(7),

                                                                "swap" => opcodes.push(8),
                                                                "copy" => opcodes.push(9),
                                                                "jmp" => opcodes.push(10),

                                                                "jeq" => opcodes.push(11),
                                                                "jnq" => opcodes.push(12),

                                                                "ijgt" => opcodes.push(13),
                                                                "fjgt" => opcodes.push(14),

                                                                "mod" => opcodes.push(15),
                                                                "out" => opcodes.push(16),

                                                                "inc" => opcodes.push(17),
                                                                "dec" => opcodes.push(18),

                                                                "set" => opcodes.push(19),

                                                                "rjeq" => opcodes.push(20),
                                                                "rjnq" => opcodes.push(21),

                                                                "jmpb" => opcodes.push(22),

                                                                "dcopy" => opcodes.push(23),

                                                                "call" => opcodes.push(24),
                                                                "ret" => opcodes.push(25),

                                                                // id must be label
                                                                v => {
                                                                    if let Some(index) = labelindex
                                                                    {
                                                                        labels.insert(
                                                                            v.to_string(),
                                                                            index,
                                                                        );
                                                                        labelindex = None;
                                                                    } else if let Some(code) =
                                                                        labels.get(v)
                                                                    {
                                                                        opcodes.push(*code);
                                                                    } else {
                                                                        // add to unknown word list with index
                                                                        unknown_words.push((
                                                                            v.to_string(),
                                                                            opcodes.len(),
                                                                        ));
                                                                        opcodes.push(777)
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            // once done, check all unknown_words
                                            for words in unknown_words.iter() {
                                                if let Some(index) = labels.get(&words.0) {
                                                    opcodes.remove(words.1);
                                                    opcodes.insert(words.1, *index);
                                                    continue;
                                                } else {
                                                    println!();
                                                    println!(
                                                        "{}: Reg vm has no label : {}",
                                                        "LEXING ERROR".red(),
                                                        words.0
                                                    );
                                                    std::process::exit(1)
                                                }
                                            }

                                            // for ci in jumpstack {
                                            //     println!("{}", ci)
                                            // }
                                            // print!("opcodes: ");
                                            // for ci in opcodes.iter() {
                                            //     print!("{} ", ci)
                                            // }
                                            // println!();
                                            if let Some(main) = labels.get("main") {
                                                vec_last.push(Token::Reg(opcodes, *main));
                                            } else {
                                                println!();
                                                println!(
                                                    "{}: Reg vm has no entry point, include a main label",
                                                    "LEXING ERROR".red()
                                                );
                                                std::process::exit(1)
                                            }
                                        } else {
                                            println!();
                                            println!(
                                                "{}: Missing list before :",
                                                "LEXING ERROR".red()
                                            );
                                            std::process::exit(1)
                                        }
                                    }
                                } else {
                                    println!();
                                    println!(
                                        "{}: Reg vm missing register list before $[], note: try []: $[]",
                                        "LEXING ERROR".red()
                                    );
                                    std::process::exit(1)
                                }
                            } else {
                                vec_last.push(Token::Block(Block::List(Rc::new(list))));
                            }
                        }
                    }
                }
                _ => println!("Unknown char {}", c),
            }
        }

        self.check_token();

        // Error checking for missing pairs
        if self.is_parsing_stringdq {
            println!();
            println!(
                "{}: String left open, missing matching \"\" : still parsing string",
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

        Ok(self.tokens[0].to_owned())
    }
}
