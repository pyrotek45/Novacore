use std::rc::Rc;

use colored::Colorize;

use crate::novacore::core::{Block, Token};

pub fn debug_output(depth: usize, block: Rc<Vec<Token>>) {
    for t in block.iter() {
        //let ty = format!("{:?}", &t.Token);
        let mut sdep = String::new();
        sdep.push_str("|--");
        for _ in 0..depth {
            sdep.push_str("|--")
        }

        if let Token::Block(block) = &t {
            match block {
                Block::Literal(block) => {
                    println!(
                        "{}{}{}",
                        sdep.bright_cyan(),
                        "|--".bright_cyan(),
                        "Literal:".bright_cyan()
                    );
                    debug_output(depth + 1, block.clone());
                    continue;
                }
                Block::Lambda(block) => {
                    println!(
                        "{}{}{}",
                        sdep.bright_cyan(),
                        "|--".bright_cyan(),
                        "Lambda:".bright_cyan()
                    );
                    debug_output(depth + 1, block.clone());
                    continue;
                }
                Block::Auto(_, _) => todo!(),
                Block::Modifier(_, _) => todo!(),
                Block::Function(block) => {
                    println!(
                        "{}{}{}",
                        sdep.bright_cyan(),
                        "|--".bright_cyan(),
                        "List:".bright_cyan()
                    );
                    debug_output(depth + 1, block.clone());
                    continue;
                },
                Block::List(block) => {
                    println!(
                        "{}{}{}",
                        sdep.bright_cyan(),
                        "|--".bright_cyan(),
                        "List:".bright_cyan()
                    );
                    debug_output(depth + 1, block.clone());
                    continue;
                }
                Block::ListLambda(block) => {
                    println!(
                        "{}{}{}",
                        sdep.bright_cyan(),
                        "|--".bright_cyan(),
                        "ListLamda:".bright_cyan()
                    );
                    debug_output(depth + 1, block.clone());
                    continue;
                },
                Block::Struct(_) => {
                    println!(
                        "{}{}{}",
                        sdep.bright_cyan(),
                        "|--".bright_cyan(),
                        "Struct:".bright_cyan()
                    );
                    continue;
                },
            }
        }
        println!("{}[{}]", sdep.bright_cyan(), t.to_str_long().bright_blue());

        //printstack.push_str(&("[".to_owned() + &t.get_Token_as_string() + "]"));
        //printstack.push(' ');
    }
}
