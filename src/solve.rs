use crate::{Cell, Grid, Solved, Unsolved};

#[derive(Debug)]
pub enum SolutionError {
    UnsolvableCells(Vec<(usize, usize)>),
    RecursionLimitReached,
}

pub type SolutionResult = Result<Grid<Solved>, SolutionError>;

#[derive(Debug)]
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

    fn known(&self) -> Option<u8> {
        let mut values = self.0.iter();

        let value = values.position(|&v| v)?;

        if values.any(|&v| v) {
            return None;
        }

        Some(value as u8 + 1)
    }

    fn iter_unknown(&self) -> impl Iterator<Item = u8> + '_ {
        (if self.known().is_none() {
            Some(
                self.0
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &v)| v.then_some(i as u8 + 1)),
            )
        } else {
            None
        })
        .into_iter()
        .flatten()
    }
}

impl Grid<Unsolved> {
    pub fn solve(self, max_depth: usize) -> SolutionResult {
        println!("gathering initial constraints");
        let constraints = self.initial_constraints();

        let result = self.solve_inner(constraints, 0, max_depth);
        if result.is_ok() {
            println!("FOUND A SOLUTION");
        } else {
            println!("DID NOT FIND A SOLUTION");
        }
        result
    }

    fn solve_inner(
        mut self,
        mut constraints: Vec<Constraint>,
        depth: usize,
        max_depth: usize,
    ) -> SolutionResult {
        println!("depth: {depth}, constraints: {}", constraints.len());

        while !constraints.is_empty() {
            println!("applying {} constraints", constraints.len());
            constraints = self.apply_constraints(&constraints);
        }

        if depth >= max_depth {
            println!("recursion limit reached, not attempting to solve ambiguities");
            return self.into_solved();
        }

        // recursively solve ambiguities
        for constraint in self.iter_possible_constraints() {
            println!("trying constraint: {constraint:?}");

            let mut grid = self.clone();

            grid.0[constraint.row][constraint.col] = Cell::value(constraint.value).unwrap();

            match grid.solve_inner(vec![constraint], depth + 1, max_depth) {
                Ok(grid) => {
                    return Ok(grid);
                }
                Err(_) => {
                    println!("constraint backtracked");
                    continue;
                }
            }
        }

        // if there no ambiguities, just check the solution and return it
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

    fn into_solved(self) -> SolutionResult {
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
            Err(SolutionError::UnsolvableCells(unsolved))
        }
    }

    fn iter_cells_with_positions(&self) -> impl Iterator<Item = (usize, usize, &Cell)> + '_ {
        self.0.iter().enumerate().flat_map(|(row_i, row)| {
            row.iter()
                .enumerate()
                .map(move |(col_i, cell)| (row_i, col_i, cell))
        })
    }

    fn iter_possible_constraints(&self) -> impl Iterator<Item = Constraint> + '_ {
        self.iter_cells_with_positions()
            .flat_map(|(row, col, cell)| {
                cell.iter_unknown()
                    .map(move |value| Constraint { row, col, value })
            })
    }
}
