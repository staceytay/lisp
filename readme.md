# (Lisp (in Rust))

A simple lisp parser written in Rust. The parser takes in a single line input and returns an AST.

``` text
Transformations: string (input) -> tokens -> S-expression -> AST
```

## Installation

``` sh
 > git clone git@github.com:staceytay/lisp.git
 > cd lisp
 > cargo run
   Compiling lisp v0.1.0 (/Users/st/Repositories/rust/lisp)
    Finished dev [unoptimized + debuginfo] target(s) in 0.66s
     Running `target/debug/lisp`
(first (list 1 (+ 2 3) 9))
main: tokens = [LParen, Id("first"), LParen, Id("list"), Int(1), LParen, Id("+"), Int(2), Int(3), RParen, Int(9), RParen, RParen]
main: ast = First(
    List(
        [
            Int(
                1,
            ),
            Plus(
                Int(
                    2,
                ),
                Int(
                    3,
                ),
            ),
            Int(
                9,
            ),
        ],
    ),
)
```


## References

- [Gradual Scheme](https://github.com/staceytay/gradual-scheme)
- [(How to Write a (Lisp) Interpreter (in Python))](https://norvig.com/lispy.html)
- [Learn X in Y minutes: Where X=racket](https://learnxinyminutes.com/docs/racket/)
- [Parsing S-Expressions in Scala](https://matt.might.net/articles/parsing-s-expressions-scala/)
