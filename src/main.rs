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

fn main() {
    let opt  = Opt::from_args();
    println!("{:?}", &opt);
}
