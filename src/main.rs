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
use self::nfa::helpers::nfa_dot;
use self::nfa::NFA;

//set up structopt derivation for flags of thegrep
#[derive(Debug, StructOpt)]
#[structopt(name = "thegrep", about = "Tar Heel egrep", author = "")]

//declring optionals for thegrep function
struct Opt {
    #[structopt(short = "p", long = "parse", help = "Show Parsed AST")]
    parse: bool,

    #[structopt(short = "t", long = "tokens", help = "Show Tokens")]
    tokens: bool,

    #[structopt(help = "Regular Expression Pattern")]
    pattern: String,

    #[structopt(short = "d", long = "dot")]
    dot: bool,

    #[structopt(help = "FILES")]
    path: Vec<String>,
}

//importing tokenizer and parser functionalities from the other files
pub mod tokenizer;
use self::tokenizer::Tokenizer;
pub mod parser;
use self::parser::Parser;
use std::fs::File;
use std::io::BufRead;

//main takes in opt from the args passed in on the command line, if it encounters the parse or
//tokens flag, it will carry out the helped functions for each respectively
fn main() {
    let opt = Opt::from_args();

    if opt.tokens {
        eval_show_tokens(&opt.pattern);
    }
    if opt.parse {
        eval_show_parse(&opt.pattern);
    }
    if opt.dot {
        eval_show_dot(&opt.pattern);
    }

    let input = &opt.pattern;
    let nfa = NFA::from(&input).unwrap();
    if opt.path.len() > 0 {
        read_files(&opt, &nfa);
    } else {
        print_stdin(&nfa);
    }
}

fn print_stdin(nfa: &NFA) {
    let stdin = io::stdin();
    let reader = stdin.lock();
    check(nfa, reader);
}

fn read_files(opt: &Opt, nfa: &NFA) -> io::Result<()> {
    for paths in opt.path.iter() {
        let file = File::open(paths)?;
        let reader = io::BufReader::new(file);
        check(nfa, reader);
    }
    Ok(())
}

fn check<R: BufRead>(nfa: &NFA, reader: R) {
    for line in reader.lines() {
        if let Ok(point) = line {
            if nfa.accepts(&point) {
                println!("{}", &point);
            }
        }
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
        }
        Err(msg) => eprintln!("thegrep: {}", msg),
    }
    println!("\n");
}

//helper method for when dot flag is used
fn eval_show_dot(input: &str) {
    let nfa = NFA::from(&input).unwrap();
    println!("{}", nfa_dot(&nfa));
    std::process::exit(0);
}
