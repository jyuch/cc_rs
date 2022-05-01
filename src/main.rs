mod ctype;
mod stdlib;

use crate::{
    ctype::{is_digit, is_space},
    stdlib::strtol,
};
use std::env;

enum Token {
    Reserved(char),
    Num(i32),
    //EOF,
}

#[macro_export]
macro_rules! error {
    () => ({
        std::process::exit(1);
    });
    ($($arg:tt)*) => ({
        eprintln!($($arg)*);
        std::process::exit(1);
    })
}

fn consume(op: char, token: &[Token]) -> (bool, &[Token]) {
    match token {
        &[Token::Reserved(o), ..] if o == op => (true, &token[1..]),
        _ => (false, &token),
    }
}

fn expect(op: char, token: &[Token]) -> &[Token] {
    match token {
        &[Token::Reserved(o), ..] if o == op => &token[1..],
        _ => error!("{}ではありません", op),
    }
}

fn expect_number(token: &[Token]) -> (i32, &[Token]) {
    match token {
        &[Token::Num(i), ..] => (i, &token[1..]),
        _ => error!("数ではありません"),
    }
}

fn tokenize(code: &str) -> Vec<Token> {
    let mut remain = code.to_string();
    let mut token = vec![];

    loop {
        if remain.is_empty() {
            break;
        }

        if is_space(&remain) {
            remain = remain[1..remain.len()].to_string();
            continue;
        }

        let n = &remain[0..1].to_string();
        if n.eq("+") || n.eq("-") {
            remain = remain[1..remain.len()].to_string();
            token.push(Token::Reserved(n.chars().next().unwrap()));
            continue;
        }

        if is_digit(&remain) {
            let (n, r) = strtol(&remain);
            remain = r;
            token.push(Token::Num(n));
            continue;
        }

        error!("トークナイズできません");
    }

    token
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        error!("引数の個数が正しくありません");
    }

    let t = tokenize(&args[1]);
    let mut token = t.as_slice();

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    let (n, r) = expect_number(token);
    token = r;
    println!("  mov rax, {}", n);

    loop {
        if token.is_empty() {
            break;
        }

        if let (true, r) = consume('+', token) {
            let (n, r) = expect_number(r);
            token = r;
            println!("  add rax, {}", n);
            continue;
        } else {
            let r = expect('-', token);
            let (n, r) = expect_number(r);
            token = r;
            println!("  sub rax, {}", n);
            continue;
        }
    }

    println!("  ret");
}
