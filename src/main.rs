extern crate libc;

use std::{io, io::Write};

#[derive(Debug)]
enum Action {
    Advance,
    Reverse,
    Increment,
    Decrement,
    Output,
    Input,
    Loop(Option<usize>),
    LoopEnd(Option<usize>),
    Invalid,
}

struct Machine {
    actions: Vec<Action>,
}

impl Machine {
    fn new(actions: Vec<Action>) -> Self {
        Self { actions: actions }
    }

    fn set_loop(&mut self) -> Result<(), ()> {
        let mut loop_start: Vec<usize> = Vec::new();
        let mut loop_end: Vec<usize> = Vec::new();

        for n in 0..self.actions.len() {
            match &self.actions[n] {
                &Action::Loop(_) => loop_start.push(n),
                &Action::LoopEnd(_) => loop_end.push(n),
                _ => {}
            }

            if loop_start.len() < loop_end.len() {
                return Err(());
            }
        }

        if loop_start.len() != loop_end.len() {
            return Err(());
        }

        while let Some(p) = loop_start.pop() {
            let position = loop_end.iter().position(|end| p < *end).unwrap();
            let end = loop_end[position];
            loop_end.remove(position);

            self.actions[p] = Action::Loop(Some(end));
            self.actions[end] = Action::LoopEnd(Some(p));
        }

        Ok(())
    }

    fn evaluate(self) {
        use Action::*;

        let mut value: Vec<u8> = vec![0u8; 10000];
        let mut value_point: usize = 0;
        let mut action_point: usize = 0;

        loop {
            if action_point >= self.actions.len() {
                break;
            }
            let ref action = self.actions[action_point];
            let mut m = value[value_point];

            match action {
                Advance => value_point += 1,
                Reverse => value_point -= 1,
                Increment => value[value_point] += 1,
                Decrement => value[value_point] -= 1,
                Output => print!("{}", m as char),
                Input => {
                    let c = unsafe { libc::getchar() as u8 };
                    value[value_point] = c;
                }
                Loop(end) => {
                    if m == 0 {
                        action_point = end.unwrap();
                    }
                }
                LoopEnd(start) => {
                    action_point = start.unwrap();
                    continue;
                }
                Invalid => {}
            }

            action_point += 1;

            io::stdout().flush();
        }
    }
}

fn main() {
    let mut code = String::new();
    io::stdin().read_line(&mut code).unwrap();
    code = code.trim().to_string();

    let actions: Vec<Action> = code.chars().map(Action::from).collect();

    let mut machine = Machine::new(actions);
    machine.set_loop();
    machine.evaluate();
}

impl From<char> for Action {
    fn from(c: char) -> Action {
        use Action::*;

        match c {
            '>' => Advance,
            '<' => Reverse,
            '+' => Increment,
            '-' => Decrement,
            '.' => Output,
            ',' => Input,
            '[' => Loop(None),
            ']' => LoopEnd(None),
            _ => Invalid,
        }
    }
}
