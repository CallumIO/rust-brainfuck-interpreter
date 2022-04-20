use std::io::{self};

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
