My Advent of Code solutions for 2025.
Solutions are in Rust and each day is its own crate. So you need to `cd` in to the day to run it.
Each day is based on a simple template crate that I just `cp template day_xx` at the start of each day.
Each day also uses `clap` so I can plug in whatever input file I want and dynamically run either or both parts.
The format for that is:

`cargo run -- -i "path/to/input" -p 1`

`p` can be `1`, `2`, or `both` (which is default).

Most solutions are test driven, but not all if the task is simple.
