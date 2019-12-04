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

### Day 3
A bit more complex, just because in Rust ignoring safety requires a lot of explicitly telling the
compiler that you are really fine if this code panics. On the other hand, the functional stuff is
really nice for programming in this style, and although it's a bit overkill Rust has nice ways of
encoding data representations compared to, say, JS or Python. Still very much just "implement the
basic steps you would use to solve the problem by hand". 

I wonder if using two lists is actually inefficient enough to be an issue: I doubt it, but that
might be a slightly nontrivial choice on my part.

### Day 4
Slightly harder! Doing the second part clearly took me a couple minutes, but still pretty much just
implementing exactly what the problem tells us to do. Luckily, no file I/O, so this probably took me
less time than day 3.
