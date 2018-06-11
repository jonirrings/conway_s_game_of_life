extern crate board;

use board::config_loader::load_config;
use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2{
        println!("missing params!!!");
        println!("usage: {} [file_path]",args[0]);
        return;
    }
    let filename = &args[1];
    let board = load_config(filename);
    match board {
        Some(mut b) => {
            b.iterate();
            println!("{}", b);
        }
        None => println!("failed to parse configuration file"),
    }
}
