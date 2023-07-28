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
                    // TODO: Check if token is a valid identifier.
                    Token::Id(token.to_string())
                }
            }
        };
        tokens.push(t);
    }

    tokens
}

#[derive(Debug)]
enum SExp {
    Nil,
    Int(usize),
    Id(String),
    Cons(Box<SExp>, Box<SExp>),
}

// TODO: Return Result instead of panicking.
fn parse_sexp(tokens: &mut Vec<Token>) -> SExp {
    // TODO: Either reverse vec or use VecDeque to avoid O(n^2)
    let t = tokens.remove(0);

    match t {
        Token::Int(i) => SExp::Int(i),
        Token::Id(id) => SExp::Id(id.clone()),
        Token::LParen => {
            let exp = parse_sexp_list(tokens);
            assert!(matches!(tokens[0], Token::RParen));
            tokens.remove(0); // Discard Token::RPAREN.
            exp
        }
        _ => panic!(
            "parse_sexp: unexpected token, t = {:?}, tokens = {:?}",
            t, tokens
        ),
    }
}

fn parse_sexp_list(tokens: &mut Vec<Token>) -> SExp {
    match tokens[0] {
        Token::RParen => SExp::Nil,
        _ => {
            let head = parse_sexp(tokens);
            let tail = parse_sexp_list(tokens);
            SExp::Cons(Box::new(head), Box::new(tail))
        }
    }
}

#[derive(Debug)]
enum Exp {
    Int(usize),
    Id(String),
    List(Vec<Exp>),
    Plus(Box<Exp>, Box<Exp>),
    First(Box<Exp>),
}

// TODO: Return Result instead of panicking.
fn parse_ast(sexp: SExp) -> Exp {
    match sexp {
        SExp::Int(i) => Exp::Int(i),
        SExp::Id(id) => Exp::Id(id.clone()),
        SExp::Cons(sexp1, sexp2) if matches!(*sexp2, SExp::Nil) => parse_ast(*sexp1),
        SExp::Cons(sexp1, sexp2) => match *sexp1 {
            SExp::Id(id) => match id.as_str() {
                "+" => match *sexp2 {
                    SExp::Cons(sexp3, sexp4) => {
                        Exp::Plus(Box::new(parse_ast(*sexp3)), Box::new(parse_ast(*sexp4)))
                    }
                    _ => panic!("parse_ast: + missing operands"),
                },
                "first" => Exp::First(Box::new(parse_ast(*sexp2))),
                "list" => {
                    let mut items = Vec::new();
                    let mut next_sexp = *sexp2;
                    while let SExp::Cons(sexp3, sexp4) = next_sexp {
                        items.push(parse_ast(*sexp3));
                        next_sexp = *sexp4;
                    }
                    Exp::List(items)
                }
                _ => panic!("parse_ast: unsupported id: {:?}, {:?}", id, sexp2),
            },
            _ => panic!("parse_ast: {:?}, {:?}", sexp1, sexp2),
        },
        SExp::Nil => panic!("parse_ast: unexpected nil"),
    }
}

// TODO: Add tests for each step of transformation.
fn main() {
    // TODO: write out assert_eq! here to make testing easier.
    loop {
        let mut input = String::new();
        // TODO: Add prompt when reading lines.
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let mut tokens = tokenize(&input);
        println!("main: tokens = {:?}", tokens);
        let sexp = parse_sexp(&mut tokens);
        let ast = parse_ast(sexp);
        println!("main: ast = {:#?}", ast);
    }
}
