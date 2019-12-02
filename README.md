# Advent of Code 2019 Solutions In Rust

This is where I'll put my AOC 2019 solutions. I'm going to limit myself to Rust this time around,
because it's good practice and fast.

## Disclaimer
The code in this repository obviously spoils the problems. If you're competing and want to solve
them yourself, then why are you here? Go! 

### Day 1
Simple math: not too bad.

### Day 2
Pretty simple: you have to be a little careful in Rust, because you can't change the code as you
loop over it due to the borrow checker, but that can easily be remedied. Rust's functional approach
to handling iterators is a godsend for quick input parsing.
