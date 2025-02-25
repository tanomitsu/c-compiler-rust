use std::env;

#[derive(PartialEq, Clone)]
enum TokenKind {
    Reserved(char),
    Number(i32),
}

#[derive(Clone)]
struct Token {
    kind: TokenKind,
}

struct Tokenizer<'a> {
    input: &'a str,
    tokens: Vec<Token>,
    current: usize,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a str) -> Self {
        let mut tokenizer = Tokenizer {
            input,
            tokens: Vec::new(),
            current: 0,
        };
        tokenizer.tokenize().expect("トークナイズに失敗しました");
        tokenizer
    }

    // 次のトークンが期待している記号のときには、トークンを1つ読み進めて
    // 真を返す。それ以外の場合には偽を返す。
    fn consume(&mut self, op: char) -> bool {
        let token = &self.tokens[self.current];
        if token.kind != TokenKind::Reserved(op) {
            return false;
        }
        self.current += 1;
        true
    }

    fn expect(&mut self, op: char) -> Result<(), String> {
        let token = &self.tokens[self.current];
        if token.kind != TokenKind::Reserved(op) {
            return Err(format!("予期せぬ文字です: {}", op));
        }
        self.current += 1;
        Ok(())
    }

    fn expect_number(&mut self) -> Result<i32, String> {
        let token = &self.tokens[self.current];
        if let TokenKind::Number(num) = token.kind {
            self.current += 1;
            Ok(num)
        } else {
            Err("数値ではありません".to_string())
        }
    }

    fn at_eof(&self) -> bool {
        self.current >= self.tokens.len()
    }

    // 入力文字をトークナイズする
    fn tokenize(&mut self) -> Result<(), String> {
        let p = self.input;
        let mut pos = 0;
        while pos < p.len() {
            let mut c = p.chars().nth(pos).expect("文字がありません");
            if c.is_whitespace() {
                pos += 1;
                continue;
            }
            if c == '+' || c == '-' {
                self.tokens.push(Token {
                    kind: TokenKind::Reserved(c),
                });
                pos += 1;
                continue;
            }
            if c.is_numeric() {
                let mut num = String::new();
                while c.is_numeric() {
                    num.push(c);
                    pos += 1;
                    if pos >= p.len() {
                        break;
                    }
                    c = p.chars().nth(pos).expect("文字がありません");
                }
                let num = num.parse::<i32>().expect("数値ではありません");
                self.tokens.push(Token {
                    kind: TokenKind::Number(num),
                });
                continue;
            }
            return Err(format!("予期せぬ文字です: {}", c));
        }
        Ok(())
    }
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
    let mut tokenizer = Tokenizer::new(expr);

    println!(
        "  mov rax, {}",
        tokenizer.expect_number().expect("数値ではありません")
    );
    while !tokenizer.at_eof() {
        if tokenizer.consume('+') {
            println!(
                "  add rax, {}",
                tokenizer.expect_number().expect("数値ではありません")
            );
            continue;
        }
        tokenizer.expect('-').expect("予期せぬ文字です");
        println!(
            "  sub rax, {}",
            tokenizer.expect_number().expect("数値ではありません")
        );
    }

    println!("  ret");
}
