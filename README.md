# Advent of Code 2023

Advent of Code 2023 done with Rust.

Uses `clap.rs` to expose a command line interface. Usage:

```
# String input
$ advent_of_code_2023 --day 1 --part 1 "123four"
Answer for day 1, part 1
14

# Reading from file
$ advent_of_code_2023 --day 1 --part 2 -f ./path/to/input
Answer for day 1, part 2
55555
```

Tests are (at minimum) the provided examples from each day. Extra tests may be added if I found them helpful for development. 

Individual days can be tested as

```
$ cargo test day1

     Finished test [unoptimized + debuginfo] target(s) in 0.01s
     Running unittests src/lib.rs (target/debug/deps/advent_of_code_23-ef15f2841bade115)

running 2 tests
test day1::tests::part_1 ... ok
test day1::tests::part_2 ... ok

...
```

## Useful Tools

Specific projects I found useful in tackling these problems:

- [`nom.rs`](https://github.com/rust-bakery/nom) for parsing input
