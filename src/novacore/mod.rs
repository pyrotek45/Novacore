mod utilities;

mod core;
use std::rc::Rc;

use self::{core::CallBack, evaluator::Evaluator};

mod core_ops;
mod debugger;
mod evaluator;
mod lexer;
mod parser;
mod state;

pub struct Vm {
    state: Box<state::State>,
    lexer: lexer::Lexer,
    parser: parser::Parser,
    evaluator: Evaluator,
    //instructions: Vec<Token>,
}

impl Vm {
    pub fn run(&mut self) {
        self.evaluator
            .evaluate(self.parser.shunt(self.lexer.parse()), state::new());
    }

    pub fn run_string(mut self, input: &str) -> Vm {
        self.lexer = lexer::Lexer::new();
        self.lexer.insert_string(input);
        self.parser = parser::Parser::new();
        self.init();
        Vm {
            state: self
                .evaluator
                .evaluate(self.parser.shunt(self.lexer.parse()), self.state),
            lexer: self.lexer,
            parser: self.parser,
            evaluator: self.evaluator,
        }
    }

    pub fn get_last_in_state(&mut self) -> Option<String> {
        self.state
            .get_from_heap_or_pop()
            .map(|tok| format!(" ---> [{}]", tok.to_str()))
    }

    pub fn add_function(&mut self, name: &str, function: CallBack) {
        self.lexer
            .add_function(name, self.evaluator.add_function(function));
    }

    pub fn init(&mut self) {
        // basic output
        self.add_function("println", core_ops::io::println);
        self.add_function("print", core_ops::io::print);

        // basic maths
        self.add_function("sqrt", core_ops::math::sqrt);

        // basic functions
        self.add_function("range", core_ops::create::range);
        self.add_function("random", core_ops::random::random);
        self.add_function("sleep", core_ops::time::sleep);
    }

    pub fn debug_file(&mut self, filename: &str) {
        let mut core = Vm {
            lexer: lexer::Lexer::new_from_file(filename),
            evaluator: evaluator::Evaluator::new(),
            parser: parser::Parser::new(),
            state: state::new(),
        };
        core.init();
        println!("Lexer Debug");
        debugger::debug_output(0, Rc::new(core.lexer.parse()));
        println!("Parser Debug");
        core.lexer.clear();
        core.parser.clear();
        debugger::debug_output(0, Rc::new(core.parser.shunt(core.lexer.parse())));
    }

    pub fn debug_string(&mut self, filename: &str) {
        let mut core = Vm {
            lexer: lexer::Lexer::new_from_string(filename),
            evaluator: evaluator::Evaluator::new(),
            parser: parser::Parser::new(),
            state: state::new(),
        };
        core.init();
        println!("Lexer Debug");
        debugger::debug_output(0, Rc::new(core.lexer.parse()));
        println!("Parser Debug");
        core.lexer.clear();
        core.parser.clear();
        debugger::debug_output(0, Rc::new(core.parser.shunt(core.lexer.parse())));
    }
}

pub fn new_from_file(filename: &str) -> Vm {
    let mut core = Vm {
        lexer: lexer::Lexer::new_from_file(filename),
        evaluator: evaluator::Evaluator::new(),
        parser: parser::Parser::new(),
        state: state::new(),
    };
    core.init();
    core
}

pub fn new() -> Vm {
    let mut core = Vm {
        lexer: lexer::Lexer::new(),
        evaluator: evaluator::Evaluator::new(),
        parser: parser::Parser::new(),
        state: state::new(),
    };
    core.init();
    core
}
