mod utilities;

mod core;
use std::rc::Rc;

use self::{core::CallBack, evaluator::Evaluator, utilities::print_error};

mod core_ops;
mod debugger;
mod evaluator;
mod lexer;
mod parser;
mod state;

pub struct Vm {
    lexer: lexer::Lexer,
    parser: parser::Parser,
    evaluator: Evaluator,
}

impl Vm {
    pub fn run(&mut self) {
        self.evaluator
            .evaluate(self.parser.shunt(self.lexer.parse()));
        if !self.evaluator.state.error_log.is_empty() {
            for err in self.evaluator.state.error_log.iter() {
                print_error(err)
            }
        }
    }

    pub fn run_string(&mut self, input: &str) {
        self.lexer = lexer::Lexer::new();
        self.lexer.insert_string(input);
        self.parser = parser::Parser::new();
        self.init();
        self.evaluator
            .evaluate(self.parser.shunt(self.lexer.parse()))
    }

    pub fn _get_last_in_state(&mut self) -> Option<String> {
        self.evaluator
            .state
            .get_from_heap_or_pop()
            .map(|tok| format!(" ---> [{}]", tok.to_str_debug()))
    }

    pub fn get_stack_output(&mut self) -> Option<String> {
        let mut output_string = String::new();
        output_string.push('[');
        for stack_output in self.evaluator.state.execution_stack.iter() {
            output_string.push_str(&stack_output.to_str());
            output_string.push(',');
        }
        output_string.pop();
        if !output_string.is_empty() {
            output_string.push(']');
            Some(output_string)
        } else {
            None
        }
    }

    pub fn add_function(&mut self, name: &str, function: CallBack) {
        self.lexer
            .add_function(name, self.evaluator.add_function(function));
    }

    pub fn init(&mut self) {
        // io
        self.add_function("println", core_ops::io::println);
        self.add_function("echo", core_ops::io::println);
        self.add_function("print", core_ops::io::print);
        self.add_function("readln", core_ops::io::readln);
        self.add_function("dump", core_ops::io::dump);

        // Operations
        self.add_function("free", core_ops::operator::free);
        self.add_function("resolve", core_ops::operator::resolve);
        // self.add_function("exit", core_ops::operator::exit);

        // Test
        self.add_function("ttos", core_ops::comparison::assert_stack_test);

        // readline
        // getch
        // rawread
        // clear

        // math
        self.add_function("sqrt", core_ops::math::sqrt);

        // pow

        // create
        self.add_function("range", core_ops::create::create_range);
        self.add_function("collect", core_ops::create::collect);
        self.add_function("iota", core_ops::create::iota);

        // random
        self.add_function("random", core_ops::random::random);

        // stack operations

        self.add_function("dup", core_ops::shuffle::dup);
        self.add_function("ddup", core_ops::shuffle::ddup);
        self.add_function("swap", core_ops::shuffle::swap);
        self.add_function("drop", core_ops::shuffle::drop);
        self.add_function("nip", core_ops::shuffle::nip);
        self.add_function("over", core_ops::shuffle::over);
        self.add_function("dover", core_ops::shuffle::dover);
        self.add_function("rot", core_ops::shuffle::rot);
        self.add_function("drot", core_ops::shuffle::drot);
        self.add_function("wipe", core_ops::shuffle::wipe);

        // time
        self.add_function("sleep", core_ops::time::sleep);
        self.add_function("time", core_ops::time::time);

        // // list
        // push
        // last
        // pop
        // remove
        // extract
        // insert
        // append

        // //modifier
        self.add_function("let", core_ops::modifier::closure_let);
        self.add_function("rec", core_ops::modifier::closure_rec);
        self.add_function("auto", core_ops::modifier::closure_auto);
        self.add_function("mod", core_ops::modifier::modifier);
        self.add_function("func", core_ops::modifier::func);
        self.add_function("list", core_ops::modifier::list);
        self.add_function("struct", core_ops::modifier::create_struct);
        self.add_function("block", core_ops::modifier::block);
        self.add_function("include", core_ops::modifier::include);

        // //control flow
        self.add_function("if", core_ops::control::if_statement);
        self.add_function("when", core_ops::control::when_statement);
        self.add_function("unless", core_ops::control::unless_statement);
        //self.add_function("for", core_ops::control::for_loop);
        self.add_function("call", core_ops::control::block_call);
        self.add_function("each", core_ops::control::each);
        self.add_function("times", core_ops::control::times);
        self.add_function("while", core_ops::control::while_loop);
        self.add_function("eval", core_ops::control::eval_top);
    }

    pub fn debug_file(&mut self, filename: &str) {
        let mut core = Vm {
            lexer: lexer::Lexer::new_from_file(filename),
            evaluator: evaluator::Evaluator::new(),
            parser: parser::Parser::new(),
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
    };
    core.init();
    core
}

pub fn new() -> Vm {
    let mut core = Vm {
        lexer: lexer::Lexer::new(),
        evaluator: evaluator::Evaluator::new(),
        parser: parser::Parser::new(),
    };
    core.init();
    core
}
