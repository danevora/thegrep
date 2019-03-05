#![allow(unused)]

/**
 * thegrep - Tar Heel egrep
 *
 * Author(s): Daniel Evora, Peter Morrow 
 * ONYEN(s): devora, peterjm
 *
 * UNC Honor Pledge: I pledge I have received no unauthorized aid
 * on this assignment. I further pledge not to distribute my solution
 * to this code to anyone other than the course staff and partner.
 */

extern crate structopt;
use structopt::StructOpt;
use std::io;

const QUIT_STRING: &str = "quit\n";
const EXIT_OK: i32 = 0;
const EXIT_ERR: i32 = 1;

#[derive(Debug, StructOpt)]
#[structopt(name = "thegrep", about = "Tar Heel egrep", author = "")]
struct Opt {
    
    #[structopt(short = "p", long = "parse", help = "Show Parsed AST")]
    parse: bool,

    #[structopt(short = "t", long = "tokens", help = "Show Tokens")]
    tokens: bool,

    #[structopt(help = "Regular Expression Pattern")]
    pattern: String,

}

pub mod tokenizer;
use self::tokenizer::Tokenizer;

fn main() {
    let opt  = Opt::from_args();
    loop {
        eval(&read(), &opt);
    }
}

fn eval(input: &str, opt: &Opt) {
    if opt.tokens {
        eval_show_tokens(input);
    }
}

fn eval_show_tokens(input: &str) {
    let mut tokens = Tokenizer::new(input);
    while let Some(token) = tokens.next() {
        println!("{:?}", token);
    }
    print!("\n");
}

fn read() -> String {
    match read_line() {
        Ok(line) => {
            if line == QUIT_STRING {
                std::process::exit(EXIT_OK);
            } else {
                line
            }
        }
        Err(message) => {
            eprintln!("Err: {}", message);
            std::process::exit(EXIT_ERR);
        }
    }
}

fn read_line() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

