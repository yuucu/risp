mod env;
mod eval;
mod parse;
mod types;

use env::default_env;
use eval::eval;
use parse::{parse, tokenize};
use std::io::{self, Write};
use types::{RispEnv, RispErr, RispExp};

pub fn parse_eval(expr: &str, env: &mut RispEnv) -> Result<RispExp, RispErr> {
    let (parsed_exp, _) = parse(&tokenize(expr))?;
    let evaled_exp = eval(&parsed_exp, env)?;

    Ok(evaled_exp)
}

fn main() {
    let env = &mut default_env();
    const PROMPT: &str = "risp > ";
    println!("risp, by yuucu 2022");
    println!("Ctrl+D to exit");

    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        let bytes_read = io::stdin().read_line(&mut input).unwrap();
        if bytes_read == 0 {
            // EOF or Ctrl-D
            println!("\nBye!");
            break;
        }
        match parse_eval(&input, env) {
            Ok(res) => println!("{}", res),
            Err(e) => match e {
                RispErr::Reason(msg) => println!("Error: {}", msg),
            },
        }
    }
}
