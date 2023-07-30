use crate::{Cell, Grid, Solved, Unsolved};

struct Constraint {
    row: usize,
    col: usize,
    value: u8,
}

impl Cell {
    fn constrain(&mut self, value: u8) -> Option<u8> {
        if !self.0[value as usize - 1] {
            return None;
        }

        self.0[value as usize - 1] = false;
        self.known()
    }
}

impl Grid<Unsolved> {
    pub fn solve(mut self) -> Result<Grid<Solved>, Vec<(usize, usize)>> {
        let mut constraints = self.initial_constraints();
        while !constraints.is_empty() {
            constraints = self.apply_constraints(&constraints);
        }

        self.into_solved()
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

    fn into_solved(self) -> Result<Grid<Solved>, Vec<(usize, usize)>> {
        let mut unsolved = vec![];

        for row in 0..9 {
            for col in 0..9 {
                if self.0[row][col].known().is_none() {
                    unsolved.push((row, col));
                }
            }
        }

        if unsolved.is_empty() {
            Ok(Grid(self.0, Solved))
        } else {
            Err(unsolved)
        }
    }
}
