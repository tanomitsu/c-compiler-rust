use std::env;

fn read_number(expr: &String, pos: &mut usize) -> Result<i32, std::num::ParseIntError> {
    let mut num = String::new();
    assert!(*pos < expr.len());
    loop {
        if *pos >= expr.len() {
            break;
        }
        let c = expr.chars().nth(*pos).unwrap_or_else(|| {
            eprintln!("予期せぬ文字列の終わりです");
            std::process::exit(1);
        });
        if c.is_numeric() {
            num.push(c);
            *pos += 1;
        } else {
            break;
        }
    }

    num.parse::<i32>()
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() != 1 {
        eprintln!("引数の個数が正しくありません");
        std::process::exit(1);
    }

    println!(".intel_syntax noprefix");
    println!(".globl main");
    println!("main:");

    let expr = &args[0];
    let mut pos = 0;

    let num = read_number(expr, &mut pos).expect("Failed to parse number");
    println!("  mov rax, {}", num);

    while pos < expr.len() {
        let c = expr.chars().nth(pos).unwrap();
        if c == '+' {
            pos += 1;
            let num = read_number(expr, &mut pos).expect("Failed to parse number");
            println!("  add rax, {}", num);
            continue;
        }
        if c == '-' {
            pos += 1;
            let num = read_number(expr, &mut pos).expect("Failed to parse number");
            println!("  sub rax, {}", num);
            continue;
        }
        eprintln!("予期せぬ文字です: {}", c);
        std::process::exit(1);
    }

    println!("  ret");
}
