#![feature(non_ascii_idents)]
extern crate lalrpop_lambda;

use lalrpop_lambda::Expression;
use lalrpop_lambda::parse::ExpressionParser;

macro_rules! play {
    ($expr:expr $(, $func:expr)?) => {{
        let e = ExpressionParser::new().parse($expr).unwrap();
        print!("{} parse-> {}", $expr, e);
        $(
            let e = $func(&e, false);  // very funky.
            print!(" -> {}", e);
        )?
        println!("");
        e
    }}
}

fn main() {
    play!("x");
    play!(r"\x.x");
    play!(r"\x.y");
    play!("x x");
    play!("x y");

    println!();
    play!(r"(\x.x) x", Expression::normalize);
    play!(r"(\x.x) y", Expression::normalize);

    // Single β-reduction identity function.
    println!();
    play!(r"\x.x a", Expression::normalize);
    play!(r"(\x.x) a", Expression::normalize);

    // Partial application.
    println!();
    let norm = play!(r"(\x.\y.x y) a", Expression::normalize);
    play!(&format!("({}) b", norm), Expression::normalize);
    // Multiple (curried) β-reductions on an identity function.
    play!(r"(\x.\y.x y) a b", Expression::normalize);

    println!();
    play!(r"((\x.(\x.x x) a) b)", Expression::normalize);

    // Ω
    println!();
    play!(r"\x.(x x) (\x.(x x))");
    play!(r"(\x.(x x)) (\x.(x x))");
    // XXX: Blows the stack in our strategy.
    play!(r"(\x.(x x)) (\x.(x x))", Expression::normalize);
}
