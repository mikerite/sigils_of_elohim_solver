# Sigils of Elohim Solver

A console application that solves tetromino tiling puzzles from the game
(Sigils of Elohim)[1].

```
$soe_solver 4 4 LLZZ --pretty
┌─────┬─┐
│ ┌─┬─┘ │
├─┘ │ ┌─┤
│ ┌─┴─┘ │
└─┴─────┘
```

## Algorithm

The program uses a simple back tracking algorithm. It tries to fill the
top-left most open square any of the remaining tetrominoes. If none fit, it
back tracks to the previous tetromino and resumes the search there with the
next available tetromino. The program uses bitboards in its representation
of the puzzle state during search.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

[1](https://store.steampowered.com/app/321480/Sigils_of_Elohim/)
