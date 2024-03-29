# nls_term_grid

[![CICD](https://github.com/bydlw98/nls_term_grid/actions/workflows/CICD.yml/badge.svg)](https://github.com/bydlw98/nls_term_grid/actions/workflows/CICD.yml)
[![Crates.io](https://img.shields.io/crates/v/nls_term_grid)](https://crates.io/crates/nls_term_grid)
[![Docs.rs](https://img.shields.io/docsrs/nls_term_grid)](https://docs.rs/nls_term_grid)
![License](https://img.shields.io/crates/l/nls_term_grid)

This library arranges textual data in a grid format similar to `ls`

---

This library is extracted from [nls-ls v0.4.0]("https://github.com/bydlw98/nls-ls/tree/v0.4.0").

The Grid API and Implementation is inspired by [ogham/term_grid](https://crates.io/crates/term-grid)

---

## Installation

```toml
[dependencies]
nls_term_grid = "0.1.0"
```

## Example

```rust
use nls_term_grid::{Grid, GridCell, Direction, Alignment};

let cells_slice: [GridCell; 15] = [
    GridCell::from(String::from("file10")),
    GridCell::from(String::from("file20")),
    GridCell::from(String::from("file3")),
    GridCell::from(String::from("file400")),
    GridCell::from(String::from("file5")),

    GridCell::from(String::from("file100")),
    GridCell::from(String::from("file2")),
    GridCell::from(String::from("file30")),
    GridCell::from(String::from("file4")),
    GridCell::from(String::from("file500")),

    GridCell::from(String::from("file1")),
    GridCell::from(String::from("file200")),
    GridCell::from(String::from("file300")),
    GridCell::from(String::from("file40")),
    GridCell::from(String::from("file50")),
];

let grid = Grid::new(2, Direction::LeftToRight, &cells_slice);
let display = grid.fit_into_width(35).unwrap();

assert_eq!(
    display.to_string(),
    "file10   file20   file3   file400\n\
     file5    file100  file2   file30\n\
     file4    file500  file1   file200\n\
     file300  file40   file50\n"
);
```
