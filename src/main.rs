use std::io;

#[derive(Debug)]
enum Token {
    LParen,
    RParen,
    Int(usize),
    Id(String),
    Op(Operator),
}

#[derive(Debug)]
enum Operator {
    Plus,
    List,
}

fn tokenize(buf: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let replaced = buf.replace("(", " ( ").replace(")", " ) ");
    let mut iter = replaced.split_whitespace();
    while let Some(token) = iter.next() {
        let t = match token {
            "(" => Token::LParen,
            ")" => Token::RParen,
            "+" => Token::Op(Operator::Plus),
            "list" => Token::Op(Operator::List),
            _ => {
                if token.chars().all(char::is_numeric) {
                    Token::Int(token.parse::<usize>().unwrap())
                } else {
                    Token::Id(token.to_string())
                }
            }
        };
        tokens.push(t);
    }

    tokens
}

fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let tokens = tokenize(&input);
        println!("main: tokens = {:?}", tokens);
    }
}
