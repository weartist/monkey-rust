// use crate::crates::lexer::Lexer;

// use super::lexer;
// use std::io::{self, Write};

// pub fn run_repl() {
//     loop {
//         let mut input = String::new();
//         print!("> ");
//         io::stdout().flush().unwrap();
//         io::stdin().read_line(&mut input).unwrap();
//         let input = input.trim();
//         if input == "quit" {
//             break;
//         }

//         let mut lexer = Lexer::new(input.to_string());
//         while let token = lexer.next_token() {
//             println!("{}", token);
//         }
//     }
// }

use super::lexer::Lexer;
use super::token::TokenType;
use std::io::{self, BufRead, Write};

const PROMPT: &str = ">> ";

pub fn start<R: BufRead, W: Write>(mut input: R, mut output: W) {
    loop {
        write!(output, "{}", PROMPT).unwrap();
        output.flush().unwrap();

        let mut line = String::new();
        if input.read_line(&mut line).unwrap() == 0 {
            return;
        }

        let mut lexer = Lexer::new(line);

        loop {
            let token = lexer.next_token();
            if token.t == TokenType::EOF {
                break;
            }

            writeln!(output, "{:?}", token).unwrap();
        }
    }
}
