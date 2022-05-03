use cc_rs::{tokenize::tokenize, Token, TokenKind};
use std::env;

fn consume(op: char, token: &[Token]) -> (bool, &[Token]) {
    match &token {
        &[t, ..] => match t.kind {
            TokenKind::Reserved(c) if op.eq(&c) => (true, &token[1..]),
            _ => (false, &token),
        },
        _ => (false, &token),
    }
}

fn expect(op: char, token: &[Token]) -> &[Token] {
    match &token {
        &[t, ..] => match t.kind {
            TokenKind::Reserved(c) if op.eq(&c) => &token[1..],
            _ => panic!("{}ではありません。", op),
        },
        _ => panic!("{}ではありません", op),
    }
}

fn expect_number(token: &[Token]) -> (u32, &[Token]) {
    match &token {
        &[t, ..] => match t.kind {
            TokenKind::Num(i) => (i, &token[1..]),
            _ => panic!("数ではありません"),
        },
        _ => panic!("数ではありません"),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("引数の個数が正しくありません");
    }

    let t = tokenize(args[1].chars().collect());
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
