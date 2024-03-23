use std::fmt;

use unicode_width::UnicodeWidthStr;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Alignment {
    Left,
    Right,
}

impl Default for Alignment {
    fn default() -> Self {
        Self::Left
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct GridCell {
    pub contents: String,
    pub width: usize,
    pub alignment: Alignment,
}

impl GridCell {
    pub fn write<F: fmt::Write>(&self, f: &mut F, width: usize) -> fmt::Result {
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

impl From<String> for GridCell {
    fn from(value: String) -> Self {
        let width = UnicodeWidthStr::width(&*value);

        Self {
            contents: value,
            width: width,
            alignment: Alignment::Left,
        }
    }
}

#[derive(Debug, Default)]
pub struct Grid<'a> {
    cells_slice: &'a [GridCell],
    num_spaces: usize,
    direction: Direction,
}

impl<'a> Grid<'a> {
    pub fn new(num_spaces: usize, direction: Direction, cells_slice: &[GridCell]) -> Grid {
        Grid {
            cells_slice,
            num_spaces,
            direction,
        }
    }

    pub fn total_cell_count(&self) -> usize {
        self.cells_slice.len()
    }

    pub fn fit_into_columns(&self, num_columns: usize) -> Display<'_> {
        let dimentions = self.calculate_dimentions(num_columns);

        Display {
            dimentions: dimentions,
            grid: self,
        }
    }

    pub fn fit_into_width(&self, display_width: usize) -> Option<Display<'_>> {
        if self.cells_slice.is_empty() {
            return Some(Display {
                dimentions: Dimentions::one_row(0),
                grid: self,
            });
        }
        let max_cell_width: usize = self
            .cells_slice
            .iter()
            .map(|cell| cell.width)
            .max()
            .unwrap_or(0);

        // return `None` if there is a `DisplayCell` whose width is
        // greator than or equal than display_width
        if max_cell_width >= display_width {
            None
        } else {
            let total_width: usize = (self
                .cells_slice
                .iter()
                .map(|cell| cell.width)
                .sum::<usize>())
                + (self.total_cell_count() - 1) * self.num_spaces;

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

    fn internal_fit_into_width(&self, max_cell_width: usize, display_width: usize) -> Display<'_> {
        let total_cell_count = self.total_cell_count();
        // choose the starting num_columns by using the max DisplayCell width
        // with seperator spaces
        let mut num_columns = display_width / (max_cell_width + self.num_spaces);
        let mut dimentions = self.calculate_dimentions(num_columns);

        // increase the num_columns to find the dimentions where grid is most well packed
        loop {
            num_columns += 1;
            let new_dimentions = self.calculate_dimentions(num_columns);

            // stop increasing num_columns if total width is greator than display_width
            if new_dimentions.total_width(self.num_spaces) > display_width {
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

        for (cell_index, cell) in self.cells_slice.iter().enumerate() {
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

#[derive(Debug)]
pub struct Display<'a> {
    dimentions: Dimentions,
    grid: &'a Grid<'a>,
}

impl fmt::Display for Display<'_> {
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
                let cell = &self.grid.cells_slice[cell_index];

                // if (the current column is the last column or is the last cell)
                // and the cell is left aligned, the cell does not need to be
                // written with padding and does not need be written with seperator spaces
                if ((column_index == last_column_index) || (cell_count == total_cell_count))
                    && cell.alignment == Alignment::Left
                {
                    write!(f, "{}", cell.contents)?;
                } else {
                    cell.write(f, self.dimentions.column_widths[column_index])?;
                    write!(f, "{}", " ".repeat(self.grid.num_spaces))?;
                }
            }
            // write a '\n' after the last column in row
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Direction {
    LeftToRight,
    TopToBottom,
}

impl Default for Direction {
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

    /// for dimentions to be well packed, the following must occur:
    /// 1. the last column must have less than or equal to the number of rows
    /// 2. there should be as few columns as possible, this is done by checking if
    ///     the current number of rows chosen to be used is the same as the previous
    ///     well packed dimentions. If it is the same, the previous well packed dimentions
    ///     is more well packed due to it having fewer columns
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

/// div_ceil implementation is taken from Rust Core 1.73.0 stable
fn usize_div_ceil(lhs: usize, rhs: usize) -> usize {
    let d = lhs / rhs;
    let r = lhs % rhs;
    if r > 0 && rhs > 0 {
        d + 1
    } else {
        d
    }
}
