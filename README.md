# Advent of Code 2018 (Rust)

Problems from [Advent of Code 2018](https://adventofcode.com/2018), written in [Rust](https://www.rust-lang.org/).



## Day preparation

To prepare a subproject for each new day:

```sh
./new-day.sh <DayNumber>
```

For example: `./new-day.sh 12` initializes all files in a directory named `day-12`.

And then add the new `day-NN` directory to the `workspace.members` array in [Cargo.toml](Cargo.toml).



## Run

To build and run the code for any of the days:

```sh
./run-day.sh <DayNumber>
```

Of course, `rustc` and `cargo` must be installed; if they are not, check the [Install Rust](https://www.rust-lang.org/tools/install) page.
