use std::{
    io,
    io::Write,
};

#[derive(Debug)]
enum Action {
    Advance,
    Reverse,
    Increment,
    Decrement,
    Output,
    Input,
    Loop,
    LoopEnd,
    Invalid,
}

fn main() {
    let mut code = String::new();
    io::stdin().read_line(&mut code).unwrap();
    code = code.trim().to_string();

    use Action::*;
    let actions: Vec<Action> = code.chars().map(Action::from).collect();

    let mut len = 1;
    actions.iter().for_each(|a| {
        match a {
            Advance => len += 1,
            Reverse => len -= 1,
            _ => {},
        }
    });

    let mut value: Vec<u8> = vec![0u8; len];
    let mut value_point: usize = 0;
    let mut action_point: usize = 0;

    loop {
        if action_point >= actions.len() {
            break;
        }
        let ref action = actions[action_point];
        let mut m = value[value_point];

        match action {
            Advance => value_point += 1,
            Reverse => value_point -= 1,
            Increment => m += 1,
            Decrement => m -= 1,
            Output => print!("{}", String::from_utf8(vec![m]).unwrap()),
            Input => {},
            Loop => {

            },
            LoopEnd => {

            },
            Invalid => {},
        }

        action_point += 1;
        value[value_point] = m;

        io::stdout().flush();
    }
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
            '[' => Loop,
            ']' => LoopEnd,
            _ => Invalid,
        }
    }
}