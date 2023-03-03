use std::{io::stdout, time::Duration};

use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal::{self, ClearType},
};

use crate::novacore::{core::Token, evaluator::Evaluator};

pub fn rawmode(eval: &mut Evaluator) {
    if let Some(Token::Bool(bool)) = eval.state.get_from_heap_or_pop() {
        if bool {
            terminal::enable_raw_mode().expect("could not enable raw mode");
        } else {
            terminal::disable_raw_mode().expect("Could not disable raw mode")
        }
    }
}

pub fn clearscreen(_eval: &mut Evaluator) {
    execute!(stdout(), terminal::Clear(ClearType::All)).unwrap();
    execute!(stdout(), MoveTo(0, 0)).unwrap();
}

pub fn getch(eval: &mut Evaluator) {
    if let Event::Key(KeyEvent {
        code: KeyCode::Char(character),
        modifiers: event::KeyModifiers::NONE,
        kind: _,
        state: _,
    }) = event::read().expect("Failed to read line")
    {
        eval.state.execution_stack.push(Token::Char(character))
    }
}

pub fn rawread(eval: &mut Evaluator) {
    if let Some(Token::Id(id)) = eval.state.execution_stack.pop() {
        if event::poll(Duration::from_millis(100)).expect("Error") {
            if let Event::Key(KeyEvent {
                code: KeyCode::Char(character),
                modifiers: event::KeyModifiers::NONE,
                kind: _,
                state: _,
            }) = event::read().expect("Failed to read line")
            {
                eval.state.add_varaible(&id, Token::Char(character));
            }
        }
    }
}
