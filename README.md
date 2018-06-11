# Conway's Game of Life
## Game Rule
according to [wiki](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life),
the game is a zero-player game, which start from its initial state and evaluate
to its death or infinitely.

the game is on an infinite, two-dimensional orthogonal grid of square cells,
but here we are running it on a finite grid. 
A cell is in one of two possible states, alive or dead.
Every cell interacts with its eight neighbours,
which are the cells that are horizontally, vertically, or diagonally adjacent.
At each step in time, the following transitions occur:

1. Any live cell with fewer than two live neighbors dies, as if by under population.
1. Any live cell with two or three live neighbors lives on to the next generation.
1. Any live cell with more than three live neighbors dies, as if by overpopulation.
1. Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.

## Project Layout

```
.
├── Cargo.lock                  // depencies lock
├── Cargo.toml                  // project config
├── README.md
├── src                         // source diretory, source code goes here
│   ├── lib.rs                  // the game its self, as a library, 
│   └── main.rs                 // the binary to load the game from a configuration file
└── tests
    ├── error_state.txt
    ├── initial_state.txt
    └── integration-tests.rs    // integration tests go here (unit tests go in each file they're testing)
```

## Get Started
make sure your `git` and `rustup` is setup correctly

```shell
git clone https://github.com/jonirrings/conway_s_game_of_life.git
cd conway_s_game_of_life
cargo run --release

```