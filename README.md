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

## [Day 09: Mirage Maintenance](https://adventofcode.com/2023/day/9)

basic accumulation (maybe optimization on memory is quite overkill here)

[Code](./src/solutions/day09.rs)

## [Day 10: Pipe Maze](https://adventofcode.com/2023/day/10)

basic path for part 1.
inner tile scan using 3x3 ones, so bfs can go between adjacent pipes without crossing real pipes.

[Code](./src/solutions/day10.rs)

## [Day 11: Cosmic Expansion](https://adventofcode.com/2023/day/11)

no grid as this isn't required, simple coordinates basic stuffs.
tests done manually, as there is an expand scale parameter.

[Code](./src/solutions/day11.rs)

## [Day 12: Hot Springs](https://adventofcode.com/2023/day/12)


[Code](./src/solutions/day12.rs)

## [Day 13: Point of Incidence](https://adventofcode.com/2023/day/13)

simple compare string operations

[Code](./src/solutions/day13.rs)

## [Day 14: Parabolic Reflector Dish](https://adventofcode.com/2023/day/14)

part1: simple direct computation
part2: by design there will be cycles, so we do "some" turns and automatically scan for cycles in
order to deduce results.

[Code](./src/solutions/day14.rs)

## [Day 15: Lens Library](https://adventofcode.com/2023/day/15)

simple hashmap, as suggested by text.

[Code](./src/solutions/day15.rs)
