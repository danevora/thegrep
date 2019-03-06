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
pub mod parser;
use self::parser::Parser;

fn main() {
    let opt  = Opt::from_args();
    if opt.tokens {
        eval_show_tokens(&opt.pattern);
    }
    if opt.parse {
        eval_show_parse(&opt.pattern);
    }
}

fn eval_show_tokens(input: &str) {
    let mut tokens = Tokenizer::new(input);
    while let Some(token) = tokens.next() {
        println!("{:?}", token);
    }
    print!("\n");
}

fn eval_show_parse(input: &str) {
    match Parser::parse(Tokenizer::new(input)) {
        Ok(statement) => {
            println!("{:?}", statement);
        },
        Err(msg) => eprintln!("thegrep: {}", msg),
    }
    println!("\n");
}

