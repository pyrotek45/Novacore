mod utilities;

mod core;
use std::{
    mem::{size_of, size_of_val},
    rc::Rc,
};

use crate::novacore::core::Token;

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
            .evaluate(self.parser.shunt(self.lexer.parse()), state::new(false));
    }

    pub fn run_string(mut self, input: &str) -> Vm {
        self.lexer = lexer::Lexer::new();
        self.lexer.insert_string(input);
        self.parser = parser::Parser::new(false);
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
        // io
        self.add_function("println", core_ops::io::println);
        self.add_function("print", core_ops::io::print);

        // readline
        // getch
        // rawread
        // clear

        // math
        self.add_function("sqrt", core_ops::math::sqrt);

        // pow

        // create
        self.add_function("range", core_ops::create::range);

        // random
        self.add_function("random", core_ops::random::random);

        // time
        self.add_function("sleep", core_ops::time::sleep);

        // list
        // push
        // last
        // pop
        // remove
        // extract
        // insert
        // append

        // modifier
        self.add_function("proc", core_ops::modifier::proc);
        self.add_function("let", core_ops::modifier::closure_let);
        // proc
        // rec
    }

    pub fn debug_file(&mut self, filename: &str) {
        let mut core = Vm {
            lexer: lexer::Lexer::new_from_file(filename, true),
            evaluator: evaluator::Evaluator::new(),
            parser: parser::Parser::new(true),
            state: state::new(true),
        };
        core.init();
        core.state = self.evaluator.evaluate(
            Rc::new(core.parser.shunt(core.lexer.parse())).to_vec(),
            core.state,
        );
        for errors in core.state.error_log.clone() {
            utilities::print_error(&errors.0, errors.1, filename);
        }
    }

    pub fn output(&mut self, filename: &str) {
        let mut core = Vm {
            lexer: lexer::Lexer::new_from_file(filename, false),
            evaluator: evaluator::Evaluator::new(),
            parser: parser::Parser::new(false),
            state: state::new(false),
        };
        core.init();
        println!("Lexer Debug");
        debugger::debug_output(0, Rc::new(core.lexer.parse()));
        println!("Parser Debug");
        core.lexer.clear();
        core.parser.clear();
        debugger::debug_output(0, Rc::new(core.parser.shunt(core.lexer.parse())));
    }

    pub fn outputd(&mut self, filename: &str) {
        let mut core = Vm {
            lexer: lexer::Lexer::new_from_file(filename, true),
            evaluator: evaluator::Evaluator::new(),
            parser: parser::Parser::new(true),
            state: state::new(true),
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
            parser: parser::Parser::new(true),
            state: state::new(true),
        };
        core.init();
        println!("Lexer Debug");
        debugger::debug_output(0, Rc::new(core.lexer.parse()));
        println!("Parser Debug");
        core.lexer.clear();
        core.parser.clear();
        debugger::debug_output(0, Rc::new(core.parser.shunt(core.lexer.parse())));
        println!("OUTPUT:");
        core.lexer.clear();
        core.parser.clear();
        core.state = self.evaluator.evaluate(
            Rc::new(core.parser.shunt(core.lexer.parse())).to_vec(),
            core.state,
        );
        println!();
        println!("ERROR LOG:");
        for errors in core.state.error_log.clone() {
            utilities::print_error_str(&errors.0, errors.1, filename);
        }
    }
}

pub fn new_from_file(filename: &str) -> Vm {
    let mut core = Vm {
        lexer: lexer::Lexer::new_from_file(filename, false),
        evaluator: evaluator::Evaluator::new(),
        parser: parser::Parser::new(false),
        state: state::new(false),
    };
    core.init();
    core
}

pub fn new() -> Vm {
    let mut core = Vm {
        lexer: lexer::Lexer::new(),
        evaluator: evaluator::Evaluator::new(),
        parser: parser::Parser::new(false),
        state: state::new(false),
    };
    core.init();
    core
}
