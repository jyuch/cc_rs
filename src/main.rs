extern crate core;

use cc_rs::generator::gen;
use cc_rs::parse::parse;
use cc_rs::tokenize::tokenize;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("引数の個数が正しくありません");
    }

    let token = tokenize(args[1].chars().collect());
    let node = parse(token);

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    gen(node);

    println!("  pop rax");
    println!("  ret");
}
