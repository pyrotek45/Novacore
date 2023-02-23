mod utilities;

mod core;
use std::rc::Rc;

use self::{core::CallBack, evaluator::Evaluator};

mod core_ops;
mod debugger;
mod evaluator;
pub mod lexer;
mod parser;
mod state;

pub struct Vm {
    lexer: lexer::Lexer,
    parser: parser::Parser,
    pub evaluator: Evaluator,
}

impl Vm {

    
    pub fn run(&mut self) {
        self.evaluator
            .evaluate(Rc::new(self.parser.parse(self.lexer.parse())));
    }

    
    pub fn run_string(&mut self, input: &str) {
        self.lexer = lexer::new();
        self.lexer.insert_string(input);
        self.parser = parser::new();
        self.init();
        self.evaluator
            .evaluate(Rc::new(self.parser.parse(self.lexer.parse())))
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
            .add_function(name, self.evaluator.add_function(name.to_owned(), function));
    }

    
    pub fn init(&mut self) {
        // io
        self.add_function("println", core_ops::io::println);
        self.add_function("echo", core_ops::io::println);
        self.add_function("print", core_ops::io::print);
        self.add_function("readln", core_ops::io::readln);
        self.add_function("dump", core_ops::io::dump);
        self.add_function("load", core_ops::io::load);
        self.add_function("import", core_ops::io::import);

        // Operations
        self.add_function("free", core_ops::operator::free);
        self.add_function("return", core_ops::operator::resolve);
        self.add_function("def", core_ops::operator::variable_assign);
        self.add_function("set", core_ops::operator::variable_assign_set);
        // self.add_function("exit", core_ops::operator::exit);

        // Test
        self.add_function("ttos", core_ops::comparison::assert_stack_test);

        // readline
        // getch
        // rawread
        // clear

        // type cast
        self.add_function("int", core_ops::casting::as_int);
        // math
        self.add_function("sqrt", core_ops::math::sqrt);
        self.add_function("pow", core_ops::math::pow);
        self.add_function("round", core_ops::math::round);

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
        self.add_function("timeave", core_ops::time::time_avg);

        // // list
        self.add_function("push", core_ops::list::list_push);
        self.add_function("pop", core_ops::list::list_pop);
        self.add_function("last", core_ops::list::list_last);
        self.add_function("insert", core_ops::list::list_insert);
        self.add_function("remove", core_ops::list::list_remove);

        // //modifier
        // self.add_function("let", core_ops::modifier::closure_let);
        // self.add_function("rec", core_ops::modifier::closure_rec);
        // self.add_function("auto", core_ops::modifier::closure_auto);
        // self.add_function("mod", core_ops::modifier::modifier);

        self.add_function("func", core_ops::modifier::func);
        self.add_function("list", core_ops::modifier::list);
        self.add_function("struct", core_ops::modifier::create_struct);
        self.add_function("block", core_ops::modifier::block);
        self.add_function("include", core_ops::modifier::include);

        // //control flow
        self.add_function("if", core_ops::control::if_statement);
        self.add_function("when", core_ops::control::when_statement);
        self.add_function("unless", core_ops::control::unless_statement);
        self.add_function("for", core_ops::control::for_each);

        //self.add_function("for", core_ops::control::for_loop);
        self.add_function("call", core_ops::control::block_call);
        self.add_function("exe", core_ops::control::exe);
        self.add_function("each", core_ops::control::each);
        self.add_function("times", core_ops::control::times);
        self.add_function("while", core_ops::control::while_loop);
        self.add_function("eval", core_ops::control::eval_top);
    }

    
    pub fn debug_file(&mut self, filename: &str) {
        let mut core = Vm {
            lexer: lexer::new(),
            evaluator: evaluator::Evaluator::new(),
            parser: parser::new(),
        };
        core.lexer.add_file(filename);
        core.init();
        println!("Lexer:");
        debugger::debug_output(0, Rc::new(core.lexer.parse()));
        println!("Parser:");
        core.lexer.clear();
        core.parser.clear();
        debugger::debug_output(0, Rc::new(core.parser.parse(core.lexer.parse())));
    }

    
    pub fn debug_string(&mut self, filename: &str) {
        let mut core = Vm {
            lexer: lexer::new(),
            evaluator: evaluator::Evaluator::new(),
            parser: parser::new(),
        };
        core.lexer.insert_string(filename);
        core.init();
        println!("Lexer Debug");
        debugger::debug_output(0, Rc::new(core.lexer.parse()));
        println!("Parser Debug");
        core.lexer.clear();
        core.parser.clear();
        debugger::debug_output(0, Rc::new(core.parser.parse(core.lexer.parse())));
    }
}


pub fn new_from_file(filename: &str) -> Vm {
    let mut core = Vm {
        lexer: lexer::new(),
        evaluator: evaluator::Evaluator::new(),
        parser: parser::new(),
    };
    core.lexer.add_file(filename);
    core.evaluator.state.current_file = filename.to_owned();
    core.init();
    core.evaluator.state.function_list = core.lexer.get_function_list();
    core
}


pub fn new() -> Vm {
    let mut core = Vm {
        lexer: lexer::new(),
        evaluator: evaluator::Evaluator::new(),
        parser: parser::new(),
    };
    core.init();
    core.evaluator.state.function_list = core.lexer.get_function_list();
    core
}
