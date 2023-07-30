use cell::Cell;

mod cell;
mod macros;
#[cfg(test)]
mod sample_tests;
mod solve;

type GridArray = [[Cell; 9]; 9];

macro_rules! states {
    ($($state:ident),*) => {
        $(
            #[derive(Debug, Clone, Copy, PartialEq, Eq)]
            pub struct $state;
        )*
    };
}

states! {Solved, Unsolved}

#[derive(Debug, Clone, Eq)]
pub struct Grid<State>(Box<GridArray>, State);

impl<LhsState, RhsState> PartialEq<Grid<RhsState>> for Grid<LhsState> {
    fn eq(&self, other: &Grid<RhsState>) -> bool {
        self.0 == other.0
    }
}

impl Grid<Unsolved> {
    pub fn from_cells(grid: [[Cell; 9]; 9]) -> Self {
        Self(Box::new(grid), Unsolved)
    }
}
