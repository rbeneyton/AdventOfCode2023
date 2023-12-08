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

## [Day 02: Cube Conundrum](https://adventofcode.com/2023/day/2)

parsing as usual

[Code](./src/solutions/day02.rs)

## [Day 03: Gear Ratios](https://adventofcode.com/2023/day/3)

simple loops

[Code](./src/solutions/day03.rs)

## [Day 04: Scratchcards](https://adventofcode.com/2023/day/4)

propagate counters

[Code](./src/solutions/day04.rs)

## [Day 05: If You Give A Seed A Fertilizer](https://adventofcode.com/2023/day/5)

ranges…

[Code](./src/solutions/day05.rs)

## [Day 06: Wait For It](https://adventofcode.com/2023/day/6)

trivial

[Code](./src/solutions/day06.rs)

## [Day 07: Camel Cards](https://adventofcode.com/2023/day/7)

ordering

[Code](./src/solutions/day07.rs)

## [Day 08: Haunted Wasteland](https://adventofcode.com/2023/day/8)

hidden cycles in the data, hard to spot

[Code](./src/solutions/day08.rs)
