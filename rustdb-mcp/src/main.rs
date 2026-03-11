#![allow(dead_code)]

mod parser;
mod engine;
mod storage;
mod catalog;
mod transaction;
mod mcp;

use std::io::{self, BufRead, Write};
use parser::parser::Parser;
use engine::executor::Executor;

fn main() {
    let stdin = io::stdin();
    let mut executor = Executor::new();

    println!("RustDB v0.1 - Custom Database Engine");
    println!("Type SQL commands or 'exit' to quit");
    println!();

    loop {
        print!("rustdb> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() { continue; }
        if input == "exit" || input == "quit" {
            println!("Bye!");
            break;
        }

        // ; 기준으로 여러 쿼리 분리
        let queries: Vec<&str> = input
            .split(';')
            .map(|q| q.trim())
            .filter(|q| !q.is_empty())
            .collect();

        for query in queries {
            let mut p = Parser::new(query);
            match p.parse() {
                Ok(stmt) => match executor.execute(stmt) {
                    Ok(result) => println!("{}", result),
                    Err(e)     => println!("Error: {}", e),
                },
                Err(e) => println!("Parse Error: {}", e),
            }
        }
    }
}