#![no_std]
#![doc = include_str!("../README.md")]

extern crate alloc;

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use core::fmt;

use unicode_width::UnicodeWidthStr;

#[cfg(test)]
mod tests;

/// Indicates alignment of contents when padding is required
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Alignment {
    /// Padding is added to right side of text
    ///
    /// ## Example
    ///
    /// ```text
    /// file
    /// file1
    /// file12
    /// ```
    Left,
    /// Padding is added to left side of text
    ///
    /// ## Example
    ///
    /// ```text
    ///   file
    ///  file1
    /// file12
    /// ```
    Right,
}

impl Default for Alignment {
    #[inline]
    fn default() -> Self {
        Self::Left
    }
}

/// A textual string containing its display width and alignment
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct GridCell<D: fmt::Display> {
    /// The textual string displayed when written
    pub contents: D,
    /// The display width of contents in columns
    pub width: usize,
    /// Whether contents is (left/right) aligned when padding is required
    pub alignment: Alignment,
}

impl<D: fmt::Display> GridCell<D> {
    pub(crate) fn write<F: fmt::Write>(&self, f: &mut F, width: usize) -> fmt::Result {
        let pad_width: usize = if width <= self.width {
            0
        } else {
            width - self.width
        };

        // Check if pad width is 0
        if pad_width == 0 {
            // if pad width is 0, we do not need to do padding
            write!(f, "{}", self.contents)
        } else if self.alignment == Alignment::Left {
            write!(f, "{}{}", self.contents, " ".repeat(pad_width))
        } else {
            write!(f, "{}{}", " ".repeat(pad_width), self.contents)
        }
    }
}

impl From<String> for GridCell<String> {
    fn from(value: String) -> Self {
        let width = UnicodeWidthStr::width(&*value);

        Self {
            contents: value,
            width: width,
            alignment: Alignment::Left,
        }
    }
}

/// The main struct used to format GridCells in a grid like format similar to `ls`
#[derive(Debug, Default)]
pub struct Grid<'cells, 'seperator, D: fmt::Display> {
    cells: &'cells [GridCell<D>],
    seperator: Cow<'seperator, str>,
    seperator_width: usize,
    direction: Direction,
}

impl<'cells, 'seperator, D: fmt::Display> Grid<'cells, 'seperator, D> {
    /// Create a new Grid
    pub fn new<S>(seperator: S, direction: Direction, cells: &'cells [GridCell<D>]) -> Self
    where
        S: Into<Cow<'seperator, str>>,
    {
        let seperator: Cow<'_, str> = seperator.into();
        let seperator_width = UnicodeWidthStr::width(&*seperator);

        Self {
            cells,
            seperator,
            seperator_width,
            direction,
        }
    }

    #[inline]
    pub(crate) fn total_cell_count(&self) -> usize {
        self.cells.len()
    }

    /// Returns a displayable containing the specified number of columns
    pub fn fit_into_columns(&self, num_columns: usize) -> Display<'_, D> {
        let dimentions = self.calculate_dimentions(num_columns);

        Display {
            dimentions: dimentions,
            grid: self,
        }
    }

    /// Returns a well packed displayable grid fitted within display width
    ///
    /// Returns `None` if one of the GridCell contains a width greator than the display width
    pub fn fit_into_width(&self, display_width: usize) -> Option<Display<'_, D>> {
        if self.cells.is_empty() {
            return Some(Display {
                dimentions: Dimentions::one_row(0),
                grid: self,
            });
        }
        let max_cell_width: usize = self.cells.iter().map(|cell| cell.width).max().unwrap_or(0);

        // return `None` if there is a `DisplayCell` whose width is
        // greator than or equal than display_width
        if max_cell_width >= display_width {
            None
        } else {
            let total_width: usize = (self.cells.iter().map(|cell| cell.width).sum::<usize>())
                + (self.total_cell_count() - 1) * self.seperator_width;

            // if total width width is <= display_width, display all `DisplayCell` in one row
            if total_width <= display_width {
                Some(Display {
                    dimentions: Dimentions::one_row(self.total_cell_count()),
                    grid: self,
                })
            } else {
                Some(self.internal_fit_into_width(max_cell_width, display_width))
            }
        }
    }

    fn internal_fit_into_width(
        &self,
        max_cell_width: usize,
        display_width: usize,
    ) -> Display<'_, D> {
        let total_cell_count = self.total_cell_count();
        // choose the starting num_columns by using the max DisplayCell width
        // with seperator spaces
        let mut num_columns = display_width / (max_cell_width + self.seperator_width);
        let mut dimentions = self.calculate_dimentions(num_columns);

        // increase the num_columns to find the dimentions where grid is most well packed
        loop {
            num_columns += 1;
            let new_dimentions = self.calculate_dimentions(num_columns);

            // stop increasing num_columns if total width is greator than display_width
            if new_dimentions.total_width(self.seperator_width) > display_width {
                break;
            }
            // use new_dimentions as dimentions if it is well packed
            else if new_dimentions.is_well_packed(total_cell_count, dimentions.num_rows) {
                dimentions = new_dimentions;
            }
        }

        Display {
            dimentions: dimentions,
            grid: self,
        }
    }

    fn calculate_dimentions(&self, num_columns: usize) -> Dimentions {
        let num_rows = usize_div_ceil(self.total_cell_count(), num_columns);
        let mut column_widths: Vec<usize> = vec![0; num_columns];

        for (cell_index, cell) in self.cells.iter().enumerate() {
            let column_index = match self.direction {
                Direction::LeftToRight => cell_index % num_columns,
                Direction::TopToBottom => cell_index / num_rows,
            };

            column_widths[column_index] = column_widths[column_index].max(cell.width);
        }

        Dimentions {
            num_rows: num_rows,
            column_widths: column_widths,
        }
    }
}

/// The displayable represntation of [`Grid`](struct.Grid.html)
#[derive(Debug)]
pub struct Display<'grid, D: fmt::Display> {
    dimentions: Dimentions,
    grid: &'grid Grid<'grid, 'grid, D>,
}

impl<D: fmt::Display> fmt::Display for Display<'_, D> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total_cell_count = self.grid.total_cell_count();
        if total_cell_count == 0 {
            return writeln!(f);
        }
        let mut cell_count: usize = 0;
        let last_cell_index = total_cell_count - 1;
        let num_columns = self.dimentions.column_widths.len();
        let last_column_index = num_columns - 1;

        for row_index in 0..self.dimentions.num_rows {
            for column_index in 0..num_columns {
                let cell_index = match self.grid.direction {
                    Direction::LeftToRight => row_index * num_columns + column_index,
                    Direction::TopToBottom => row_index + self.dimentions.num_rows * column_index,
                };

                // if the cell_index is greator than last_cell_index,
                // continue to next loop iteration
                if cell_index > last_cell_index {
                    continue;
                }

                cell_count += 1;
                let cell = &self.grid.cells[cell_index];

                // if (the current column is the last column or is the last cell)
                // and the cell is left aligned, the cell does not need to be
                // written with padding and does not need be written with seperator spaces
                if ((column_index == last_column_index) || (cell_count == total_cell_count))
                    && cell.alignment == Alignment::Left
                {
                    write!(f, "{}", cell.contents)?;
                } else {
                    cell.write(f, self.dimentions.column_widths[column_index])?;
                    write!(f, "{}", self.grid.seperator)?;
                }
            }
            // write a '\n' after the last column in row
            writeln!(f)?;
        }
        Ok(())
    }
}

/// Indicates direction GridCells should be written in
#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    /// Writes GridCells from left to right, like a typewriter
    LeftToRight,
    /// Writes GridCells from top to bottom, like how `ls` lists files by default
    TopToBottom,
}

impl Default for Direction {
    #[inline]
    fn default() -> Self {
        Self::LeftToRight
    }
}

#[derive(Debug, Default)]
struct Dimentions {
    num_rows: usize,
    column_widths: Vec<usize>,
}

impl Dimentions {
    pub fn total_width(&self, spaces: usize) -> usize {
        self.column_widths.iter().sum::<usize>() + ((self.column_widths.len() - 1) * spaces)
    }

    /// For dimentions to be well packed, the following must occur:
    /// 1. the last column must have less than or equal to the number of rows
    /// 2. there should be as few columns as possible, this is done by checking if
    ///     the current number of rows chosen to be used is the same as the previous
    ///     well packed dimentions. If it is the same, the previous well packed dimentions
    ///     is more well packed due to it having fewer columns
    #[inline]
    pub fn is_well_packed(&self, cell_count: usize, previous_num_rows: usize) -> bool {
        let last_col_cell_count = cell_count % (self.column_widths.len() - 1);

        (last_col_cell_count <= self.num_rows) && (self.num_rows != previous_num_rows)
    }

    pub fn one_row(cell_count: usize) -> Self {
        Self {
            num_rows: 1,
            column_widths: vec![0; cell_count],
        }
    }
}

/// Calculate the quotient of `lhs` and `rhs`, rounding the result towards positive infinity
///
/// div_ceil implementation is taken from Rust Core 1.73.0 stable
#[inline]
fn usize_div_ceil(lhs: usize, rhs: usize) -> usize {
    let d = lhs / rhs;
    let r = lhs % rhs;
    if r > 0 && rhs > 0 {
        d + 1
    } else {
        d
    }
}
