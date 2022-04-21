use std::io::{self};
use std::process::Command as Cmd;

// Enumerator for the different tokens in Brainfuck
#[derive(PartialEq, Eq)]
enum Token {
    Inc,
    Dec,
    PtrR,
    PtrL,
    In,
    Out,
    SLoop,
    ELoop,
    Ignore,
}

#[derive(PartialEq, Eq)]
enum Command {
    Inc,
    Dec,
    PtrR,
    PtrL,
    In,
    Out,
    Loop(Vec<Command>),
}

// Class representing the program and its tape

struct Program {
    tape: [u8; 4096],
    pointer: usize,
    output: String,
}
impl Program {
    pub fn new() -> Program {
        Program {
            tape: [0u8; 4096],
            pointer: 0,
            output: String::new(),
        }
    }
    pub fn command_parser(&mut self, commands: &[Command]) {
        for command in commands {
            match command {
                Command::Inc => self.tape[self.pointer] += 1,
                Command::Dec => self.tape[self.pointer] -= 1,
                Command::PtrR => self.pointer += 1,
                Command::PtrL => self.pointer -= 1,
                Command::In => {
                    let mut input = String::new();
                    print!("The program requires your input: ");
                    io::stdin().read_line(&mut input);
                    self.tape[self.pointer] = input.chars().next().unwrap() as u8;
                }
                Command::Out => {
                    clear_console();
                    self.output.push(self.tape[self.pointer] as char);
                    println!("{}", self.output);
                }
                Command::Loop(commands) => {
                    while self.tape[self.pointer] != 0 {
                        self.command_parser(commands);
                    }
                }
            }
        }
    }
}

fn clear_console() {
    Cmd::new("clear");
}

fn main() {
    let mut source = String::new();
    loop {
        let mut inp = String::new();
        io::stdin()
            .read_line(&mut inp)
            .expect("Error reading from stdin");
        inp = inp.trim().to_string();
        if inp == "" {
            break;
        }
        source = inp;
    }
}
