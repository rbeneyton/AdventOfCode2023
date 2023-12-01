# Advent Of Code 2023

https://adventofcode.com/2023 solutions in Rust.

Retrieve your daily input using your session cookie via:
```sh
cargo run --release -- --day <day> download --session <session>
```
The data is put in data/ and used directly at compile time.

To compute the  execution time, use:
```sh
cargo run --release -- --day <day> execute --part <part>
```

To measure execution time for a particular day, use:
```sh
cargo run --release -- --day <day> benchmark --number <number> --current
```

## [Day 01: Trebuchet?!](https://adventofcode.com/2023/day/1)

basic line parsing + minimization

[Code](./src/solutions/day01.rs)
