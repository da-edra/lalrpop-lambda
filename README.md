# λ-calculus Parser (using LALRPOP)

[![Build Status](https://travis-ci.org/nixpulvis/lalrpop-lambda.svg?branch=master)](https://travis-ci.org/nixpulvis/lalrpop-lambda)
[![Crates.io Version](https://img.shields.io/crates/v/lalrpop-lambda.svg?color=%238035b9)](https://crates.io/crates/lalrpop-lambda)
[![docs.rs](https://img.shields.io/badge/docs.rs-0.x.x-lightgrey.svg)](https://docs.rs/lalrpop-lambda)

Write lambda calculus with ease, and evaluate it. There are a number of ways to
use this library (each interchangeable with another):

- `Expression` AST variants `Abs`, `App`, and `Var`
- Macros `abs!`/`λ!`, `app!`/`γ!`, and `var!`
- Parsed λ-calculus strings `λx.x (\a.\b.a)`
- Native types: `u64`, `bool`, `fn` (WIP)

```rust
let id = λ!{x.x};
let one = λ!{f.λ!{x.γ!(f,x)}};

println!("{}", one.normalize(false));
assert_eq!(1u64, u64::from(app!({id},{one})));
```

![](extra/site-demo.gif)

### Usage (JS)

```js
import("./node_modules/lalrpop-lambda/lalrpop_lambda.js").then(lambda => {
    console.log([
        new lambda.Exp("x"),
        new lambda.Exp(5),
        new lambda.Exp(false),
        new lambda.Exp("(\\x.x) y").normalize(true)
    ])
});
```

### Usage (Rust)

```rust
use lalrpop_lambda::lambda::ExpressionParser;
let parser = ExpressionParser::new();

// Parse a single free variable.
let x = parser.parse("x");

// Parse the identity function.
let id = parser.parse(r"\x.x");

// f ∘ g
let compose = parser.parse(r"\f.\g.\x.(f (g x))"));

// Print the free variable in this expression.
let unbound_y = parser.parse(r"\x.x y");
println!("{}", unbound_y.free_variables());

// No need for parsing strings at all.
let id = λ!{x.x};
let one = λ!{f.λ!{x.γ!(f, x)}};

// Identity application.
let id = λ!{x.x};
println!("(id one): {} -> {}",
         app!({&id}, {&one}),
         app!({&id}, {&one}).normalize(false));

// Make the Y combinator.
let ω = parser.parse(r"λx.(x x)");
let Ω = parser.parse(r"(λx.(x x)) (λx.(x x))");
let W = parser.parse(r"λf.λx. f x x");
let Y = parser.parse(r"λf.(λx.f (x x)) (λx.f (x x))");
```

### Development

This assumes you have an updated and working copy of [`rustup`][rustup].

```sh
cargo +nightly [build | test | bench | doc | run --example <>]
```

##### WASM

First make sure you have `wasm-pack` installed. Then:

```
wasm-pack build
```
