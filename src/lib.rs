#[cfg(test)]
mod sample_tests;

pub fn sudoku(bytes: [[u8; 9]; 9]) -> Result<[[u8; 9]; 9], NoSolution> {
    let grid = Grid::from_u8s(bytes);
    grid.solve().map(|grid| grid.into_u8s())
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("No possible values for cell ({}, {})", self.col, self.row)]
pub struct NoSolution {
    pub row: usize,
    pub col: usize,
}

struct Constraint {
    row: usize,
    col: usize,
    value: u8,
}

struct Cell([bool; 9]);

impl Cell {
    fn from_u8(value: u8) -> Self {
        match value {
            0 => Self([true; 9]),
            n => {
                let mut cell = [false; 9];
                cell[n as usize - 1] = true;
                Self(cell)
            }
        }
    }

    fn into_u8(self) -> u8 {
        self.known().unwrap_or(0)
    }

    fn constrain(&mut self, value: u8) -> Option<u8> {
        if !self.0[value as usize - 1] {
            return None;
        }

        self.0[value as usize - 1] = false;
        self.known()
    }

    fn known(&self) -> Option<u8> {
        let mut values = self.0.iter();

        let value = values.position(|&v| v)?;

        if values.any(|&v| v) {
            return None;
        }

        Some(value as u8 + 1)
    }

    fn no_value(&self) -> bool {
        !self.0.iter().any(|&v| v)
    }
}

pub struct Grid([[Cell; 9]; 9]);

impl Grid {
    pub fn from_u8s(grid: [[u8; 9]; 9]) -> Self {
        Self(grid.map(|row| row.map(Cell::from_u8)))
    }

    pub fn into_u8s(self) -> [[u8; 9]; 9] {
        self.0.map(|row| row.map(Cell::into_u8))
    }

    pub fn solve(mut self) -> Result<Self, NoSolution> {
        let mut constraints = self.initial_constraints();
        while !constraints.is_empty() {
            constraints = self.apply_constraints(&constraints);
        }

        self.check_solution()?;

        Ok(self)
    }

    fn initial_constraints(&self) -> Vec<Constraint> {
        let mut constraints = vec![];
        for row in 0..9 {
            for col in 0..9 {
                if let Some(known) = self.0[row][col].known() {
                    constraints.push(Constraint {
                        row,
                        col,
                        value: known,
                    });
                }
            }
        }
        constraints
    }

    fn apply_constraints(&mut self, constraints: &[Constraint]) -> Vec<Constraint> {
        let mut new_constraints = vec![];

        for constraint in constraints {
            for (col, cell) in self.0[constraint.row].iter_mut().enumerate() {
                if col == constraint.col {
                    continue;
                }

                if let Some(known) = cell.constrain(constraint.value) {
                    new_constraints.push(Constraint {
                        row: constraint.row,
                        col,
                        value: known,
                    })
                }
            }

            for (row, cell) in self
                .0
                .iter_mut()
                .map(|row| &mut row[constraint.col])
                .enumerate()
            {
                if row == constraint.row {
                    continue;
                }

                if let Some(known) = cell.constrain(constraint.value) {
                    new_constraints.push(Constraint {
                        row,
                        col: constraint.col,
                        value: known,
                    })
                }
            }

            let row_start = constraint.row - constraint.row % 3;
            let col_start = constraint.col - constraint.col % 3;

            for row in row_start..(row_start + 3) {
                for col in col_start..(col_start + 3) {
                    if row == constraint.row && col == constraint.col {
                        continue;
                    }

                    let cell = &mut self.0[row][col];

                    if let Some(known) = cell.constrain(constraint.value) {
                        new_constraints.push(Constraint {
                            row,
                            col,
                            value: known,
                        })
                    }
                }
            }
        }

        new_constraints
    }

    fn check_solution(&self) -> Result<(), NoSolution> {
        for row in 0..9 {
            for col in 0..9 {
                if self.0[row][col].no_value() {
                    return Err(NoSolution { row, col });
                }
            }
        }
        Ok(())
    }
}
