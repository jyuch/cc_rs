use cc_rs::parse::parse;
use cc_rs::tokenize::tokenize;
use cc_rs::Node;
use std::env;

fn gen(node: Box<Node>) {
    match *node {
        Node::Num(i) => println!("  push {}", i),
        Node::Add { lhs, rhs } => {
            gen(lhs);
            gen(rhs);

            println!("  pop rdi");
            println!("  pop rax");
            println!("  add rax, rdi");
            println!("  push rax");
        }
        Node::Sub { lhs, rhs } => {
            gen(lhs);
            gen(rhs);

            println!("  pop rdi");
            println!("  pop rax");
            println!("  sub rax, rdi");
            println!("  push rax");
        }
        Node::Mul { lhs, rhs } => {
            gen(lhs);
            gen(rhs);

            println!("  pop rdi");
            println!("  pop rax");
            println!("  imul rax, rdi");
            println!("  push rax");
        }
        Node::Div { lhs, rhs } => {
            gen(lhs);
            gen(rhs);

            println!("  pop rdi");
            println!("  pop rax");
            println!("  cqo");
            println!("  idiv rdi");
            println!("  push rax");
        }
    }
}

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
