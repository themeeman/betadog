use std::collections::{HashMap};
use std::io;
use std::io::Write;

mod betadog;

use betadog::lexer::{lex};
use betadog::eval;
use betadog::parser::{parse};

fn main() {
    let ops = {
        let mut ops = HashMap::with_capacity(5);
        ops.insert(String::from("^"), 60);
        ops.insert(String::from("*"), 40);
        ops.insert(String::from("/"), 40);
        ops.insert(String::from("+"), 20);
        ops.insert(String::from("-"), 20);
        ops
    };

    loop {
        let s = {
            let mut s = String::new();
            print!(">>> ");
            io::stdout().flush()
                .expect("Failed to flush stdout");
            io::stdin().read_line(&mut s)
                .expect("Failed to read input");
            s
        };

        match lex(s) {
            Ok(toks) => { 
                println!("Lexer Output: {:?}", toks);
                match parse(toks, ops.clone()) {
                    Ok(ast) => {
                        println!("Parser Output: {}", ast);
                        //println!("Result: {}", eval(ast));
                    }
                    Err(err) => println!("{:?}", err),
                }
            },
            Err(err) => println!("{:?}", err),
        };
    }
}
