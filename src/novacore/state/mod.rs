use super::core::Token;
use colored::Colorize;
use hashbrown::HashMap;

pub fn read_lines<P>(
    filename: P,
) -> std::io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>>
where
    P: AsRef<std::path::Path>,
{
    let file = std::fs::File::open(filename)?;
    Ok(std::io::BufRead::lines(std::io::BufReader::new(file)))
}

pub fn print_line(line: usize, file: &str) {
    if let Ok(lines) = read_lines(file) {
        // Consumes the iterator, returns an (Optional) String
        let mut linenumber = 0;
        for l in lines {
            linenumber += 1;
            if linenumber == line {
                if let Ok(ip) = l {
                    println!("  {}  ", ip.white());
                }
            }
        }
    }
}

pub struct State {
    pub debug: bool,
    pub execution_stack: Vec<Token>,
    pub auxiliary: Vec<Token>,
    pub call_stack: Vec<HashMap<String, Token>>,
    pub error_log: Vec<String>,
    pub current_function_index: Vec<usize>,
    pub traceback: Vec<String>,
    pub current_file: String,
    pub function_list: HashMap<String, usize>,
}

impl State {
    pub fn add_varaible(&mut self, ident: &str, item: Token) {
        if ident != "_" {
            if let Some(scope) = self.call_stack.last_mut() {
                scope.insert(ident.to_string(), item);
            }
        }
    }

    pub fn show_error(&mut self, err: &str) {
        // Function call traceback/ show each function line

        // line and position of error

        // type of error: output

        for function_call in &self.traceback {
            //print_line( &self.current_file);
            println!("Last call: {}", &function_call.bright_yellow());
        }

        println!("{}: {}", "Error".red(), &err.bright_yellow());
    }

    pub fn remove_varaible(&mut self, ident: &str) {
        if let Some(scope) = self.call_stack.last_mut() {
            scope.remove(ident);
        }
    }
    pub fn move_varaible(&mut self, ident: &str, newident: &str) {
        if let Some(scope) = self.call_stack.last_mut() {
            if let Some(moved) = scope.remove(ident) {
                scope.insert(newident.to_string(), moved);
            }
        }
    }

    pub fn get_from_heap_or_pop(&mut self) -> Option<Token> {
        let tok = self.execution_stack.pop()?;

        if let Token::Identifier(ident) = tok {
            for scopes in self.call_stack.iter().rev() {
                if let Some(func) = self.function_list.get(&ident) {
                    return Some(Token::Function(*func));
                }
                if let Some(token) = scopes.get(&ident) {
                    return Some(token.clone());
                }
            }
            self.show_error(&format!("Unknown identifier {}", ident));
            None
        } else {
            Some(tok)
        }
    }

    pub fn get_from_heap(&mut self, ident: &str) -> Option<Token> {
        for scopes in self.call_stack.iter().rev() {
            if let Some(func) = self.function_list.get(ident) {
                return Some(Token::Function(*func));
            }
            if let Some(token) = scopes.get(ident) {
                return Some(token.clone());
            }
        }
        self.show_error(&format!("Unknown identifier {}", ident));
        None
    }
}

pub fn new() -> Box<State> {
    Box::new(State {
        execution_stack: Vec::with_capacity(1024),
        call_stack: vec![HashMap::new()],
        auxiliary: vec![],
        debug: false,
        error_log: vec![],
        current_function_index: vec![],
        traceback: vec![],
        current_file: "".to_string(),
        function_list: HashMap::new(),
    })
}
