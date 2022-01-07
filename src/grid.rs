use std::collections::HashMap;
use std::fmt;
use std::ops;

use crate::error::GridError;

#[derive(Debug)]
pub struct Grid {
    cells: Vec<Vec<Option<Cell>>>,
    width: usize,
    height: usize,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Cell {
    Zero,
    One,
}

impl Grid {
    pub fn parse<I, S, E>(lines: I) -> Result<Grid, GridError>
    where
        I: Iterator<Item = Result<S, E>>,
        S: AsRef<str>,
        GridError: From<E>,
    {
        let mut cells: Vec<Vec<_>> = Vec::new();

        for line in lines {
            let mut vec = Vec::new();

            for c in line?.as_ref().chars() {
                match c {
                    ' ' | '\t' => {}
                    '0' => vec.push(Some(Cell::Zero)),
                    '1' => vec.push(Some(Cell::One)),
                    '-' => vec.push(None),
                    _ => {
                        return Err(GridError::InvalidChar(c));
                    }
                };
            }

            if !vec.is_empty() {
                if let Some(prev) = cells.last() {
                    if vec.len() != prev.len() {
                        return Err(GridError::WidthMismatch(prev.len(), vec.len()));
                    }
                }

                cells.push(vec);
            }
        }

        if cells.is_empty() {
            return Err(GridError::EmptyGrid);
        }

        let height = cells.len();
        let width = cells[0].len();

        Ok(Grid { cells, height, width })
    }

    pub fn solve(&mut self) -> Result<(), GridError> {
        // TODO

        self.is_valid()?;

        if self.is_filled() {
            Ok(())
        } else {
            unimplemented!()
        }
    }

    fn is_filled(&self) -> bool {
        (0..self.height).all(|i| (0..self.width).all(|j| self[(i, j)].is_some()))
    }

    fn is_valid(&self) -> Result<(), GridError> {
        // No more than 2 consecutive values in a line
        (0..self.height)
            .try_for_each(|i| self.check_cells((0..self.width - 2).map(|j| ((i, j), (i, j + 1), (i, j + 2)))))?;

        // No more than 2 consecutive values in a column
        (0..self.width)
            .try_for_each(|j| self.check_cells((0..self.height - 2).map(|i| ((i, j), (i + 1, j), (i + 2, j)))))?;

        // Check that full lines are balanced
        (0..self.height).try_for_each(|i| self.check_balance((0..self.width).map(|j| (i, j))))?;

        // Check that full columns are balanced
        (0..self.width).try_for_each(|j| self.check_balance((0..self.height).map(|i| (i, j))))?;

        // For each line pair
        for i0 in 0..self.height - 1 {
            // Lines are filled and different
            (i0 + 1..self.height).try_for_each(|i1| self.check_lanes((0..self.width).map(|j| ((i0, j), (i1, j)))))?;
        }

        // For each column pair
        for j0 in 0..self.width - 1 {
            // Columns are filled and different
            (j0 + 1..self.width).try_for_each(|j1| self.check_lanes((0..self.height).map(|i| ((i, j0), (i, j1)))))?;
        }

        Ok(())
    }

    fn check_cells<I>(&self, indices: I) -> Result<(), GridError>
    where
        I: Iterator<Item = ((usize, usize), (usize, usize), (usize, usize))>,
    {
        for (idx0, idx1, idx2) in indices {
            match (&self[idx0], &self[idx1], &self[idx2]) {
                (Some(Cell::Zero), Some(Cell::Zero), Some(Cell::Zero))
                | (Some(Cell::One), Some(Cell::One), Some(Cell::One)) => {
                    return Err(GridError::AdjacentCells);
                }
                _ => {}
            }
        }

        Ok(())
    }

    fn check_balance<I>(&self, indices: I) -> Result<(), GridError>
    where
        I: Iterator<Item = (usize, usize)>,
    {
        let mut balance = HashMap::new();

        for idx in indices {
            match &self[idx] {
                Some(x) => {
                    balance.entry(x).and_modify(|count| *count += 1).or_insert(1);
                }
                None => {
                    return Ok(());
                }
            }
        }

        if balance[&Cell::Zero] != balance[&Cell::One] {
            return Err(GridError::LaneUnbalanced);
        }

        Ok(())
    }

    fn check_lanes<I>(&self, indices: I) -> Result<(), GridError>
    where
        I: Iterator<Item = ((usize, usize), (usize, usize))>,
    {
        for (idx0, idx1) in indices {
            match (&self[idx0], &self[idx1]) {
                (Some(Cell::Zero), Some(Cell::Zero)) | (Some(Cell::One), Some(Cell::One)) => {}
                _ => return Ok(()),
            }
        }

        Err(GridError::SameLanes)
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                match &self.cells[i][j] {
                    Some(cell) => {
                        write!(fmt, "{}", cell)?;
                    }
                    None => {
                        write!(fmt, "-")?;
                    }
                }

                if j < self.width - 1 {
                    write!(fmt, " ")?;
                }
            }

            if i < self.height - 1 {
                writeln!(fmt)?;
            }
        }

        Ok(())
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Zero => write!(fmt, "0"),
            Self::One => write!(fmt, "1"),
        }
    }
}

impl ops::Index<(usize, usize)> for Grid {
    type Output = Option<Cell>;

    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        &self.cells[idx.0][idx.1]
    }
}

impl ops::Not for Cell {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Zero => Self::One,
            Self::One => Self::Zero,
        }
    }
}
