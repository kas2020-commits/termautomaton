# Termautomaton

`termautomaton` is a program to simulate [Cellular Automaton](https://en.wikipedia.org/wiki/Cellular_automaton) (CA) with a Terminal User Interface (TUI) in Rust.

[Demo](https://github.com/user-attachments/assets/9d3dd6e6-c52f-4d4d-95bf-a7427c61146b)

## Usage

By default, if you just run the binary with no other arguments, it will initialize a random cell grid and evolve the grid using [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life):

```sh
# Evolves a random grid.
termautomaton
```

You can also provide your own initialized grid as an ASCII formatted file:

```sh
termautomaton -a gosperglidergun.cells
```

The format of the ASCII file is very forgiving, and allows for top-of-line comments using either `#` or `!` and then afterwards converts the characters `*`, `#`, `O` and `X` as "alive" cells and every other character as a "dead" cell. There are plans to support more than 2-state cell grids in ASCII.
