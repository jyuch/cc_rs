use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("引数の個数が正しくありません");
        process::exit(1);
    }

    let (n, mut remain) = strton(&args[1]);
    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");
    println!("  mov rax, {}", n);

    loop {
        if remain.is_empty() {
            break;
        }

        if remain[0..1].eq("+") {
            let r = &remain[1..remain.len()];
            let (n, r) = strton(r);
            println!("  add rax, {}", n);
            remain = r;
            continue;
        }

        if remain[0..1].eq("-") {
            let r = &remain[1..remain.len()];
            let (n, r) = strton(r);
            println!("  sub rax, {}", n);
            remain = r;
            continue;
        }

        eprintln!("予期しない文字です:{}", &remain[0..remain.len()]);
        process::exit(1);
    }

    println!("  ret");
    process::exit(0);
}

fn strton(value: &str) -> (i32, String) {
    let position = value
        .find(|it: char| !it.is_numeric())
        .unwrap_or(value.len());

    let n: i32 = value[0..position].parse().unwrap();
    (n, value[position..].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strton_test() {
        let (n, remain) = strton("123+");
        assert_eq!(n, 123);
        assert_eq!(remain, "+");
    }
}
