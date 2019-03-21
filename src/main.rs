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

//importing the library for structopt
extern crate structopt;
use structopt::StructOpt;
//importing library fot std in/out
use std::io;

pub mod nfa;
use self::nfa::NFA;
use self::nfa::helpers::nfa_dot;

//initializing constants for quitting program
const QUIT_STRING: &str = "quit\n"; 
const EXIT_OK: i32 = 0;
const EXIT_ERR: i32 = 1;

//set up structopt derivation for flags of thegrep
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

//importing tokenizer and parser functionalities from the other files
pub mod tokenizer;
use self::tokenizer::Tokenizer;
pub mod parser;
use self::parser::Parser;

//main takes in opt from the args passed in on the command line, if it encounters the parse or
//tokens flag, it will carry out the helped functions for each respectively
fn main() {
    let opt  = Opt::from_args();
    if opt.tokens {
        eval_show_tokens(&opt.pattern);
    }
    if opt.parse {
        eval_show_parse(&opt.pattern);
    }
}

//declares a mutable tokenizer for input. Then loops through this input and tokenizes the
//individual elements of input and prtins them to stdout
fn eval_show_tokens(input: &str) {
    let mut tokens = Tokenizer::new(input);
    while let Some(token) = tokens.next() {
        println!("{:?}", token);
    }
    print!("\n");
}

//declares a parser to parse a tokenizer of input. If everything is parsed, the returnes statement
//from parser.rs is printed, otherwise an error is printed to stderr 
fn eval_show_parse(input: &str) {
    match Parser::parse(Tokenizer::new(input)) {
        Ok(statement) => {
            println!("{:?}", statement);
        },
        Err(msg) => eprintln!("thegrep: {}", msg),
    }
    println!("\n");
}

