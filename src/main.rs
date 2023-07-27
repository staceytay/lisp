use std::io;

#[derive(Clone, Debug)]
enum Token {
    LParen,
    RParen,
    Int(usize),
    Id(String),
}

fn tokenize(buf: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let replaced = buf.replace("(", " ( ").replace(")", " ) ");
    let mut iter = replaced.split_whitespace();
    while let Some(token) = iter.next() {
        let t = match token {
            "(" => Token::LParen,
            ")" => Token::RParen,
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

#[derive(Debug)]
enum Exp {
    Nil,
    Int(usize),
    Var(String),
    List(Box<Exp>, Box<Exp>),
    Plus(Box<Exp>, Box<Exp>),
    First(Box<Exp>),
}

fn parse_list(tokens: &mut Vec<Token>) -> Exp {
    match tokens[0] {
        Token::RParen => Exp::Nil,
        _ => {
            let head = parse(tokens);
            let tail = parse_list(tokens);

            match head {
                Exp::Var(ref v) => match v.as_str() {
                    "+" => match tail {
                        Exp::List(b1, b2) => match *b2 {
                            Exp::List(b3, b4) if matches!(*b4, Exp::Nil) => Exp::Plus(b1, b3),
                            _ => panic!("parse_list: + wrong number of operands"),
                        },
                        _ => panic!("parse_list: + unexpected operands"),
                    },
                    "first" => Exp::First(Box::new(tail)),
                    _ => Exp::List(Box::new(head), Box::new(tail)),
                },
                _ => Exp::List(Box::new(head), Box::new(tail)),
            }
        }
    }
}

fn parse(tokens: &mut Vec<Token>) -> Exp {
    let t = tokens.remove(0);

    match t {
        Token::Int(i) => Exp::Int(i),
        Token::Id(id) => Exp::Var(id.clone()),
        Token::LParen => {
            let exp = parse_list(tokens);
            tokens.remove(0); // Discard Token::RPAREN.
            exp
        }
        _ => panic!(
            "parse: unexpected token, t = {:?}, tokens = {:?}",
            t, tokens
        ),
    }
}

fn main() {
    // TODO: write out assert_eq! here to make testing easier.
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let mut tokens = tokenize(&input);
        println!("main: tokens = {:?}", tokens);
        let ast = parse(&mut tokens);
        println!("main: ast = {:?}", ast);
    }
}
