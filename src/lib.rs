mod macros;
#[cfg(test)]
mod sample_tests;
mod solve;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell([bool; 9]);

impl Cell {
    pub fn empty() -> Self {
        Self([true; 9])
    }

    pub fn value(value: u8) -> Option<Self> {
        match value {
            1..=9 => {
                let mut cell = [false; 9];
                cell[value as usize - 1] = true;
                Some(Self(cell))
            }
            _ => None,
        }
    }

    fn known(&self) -> Option<u8> {
        let mut values = self.0.iter();

        let value = values.position(|&v| v)?;

        if values.any(|&v| v) {
            return None;
        }

        Some(value as u8 + 1)
    }
}

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
pub struct Grid<State>([[Cell; 9]; 9], State);

impl<LhsState, RhsState> PartialEq<Grid<RhsState>> for Grid<LhsState> {
    fn eq(&self, other: &Grid<RhsState>) -> bool {
        self.0 == other.0
    }
}

impl Grid<Unsolved> {
    pub fn from_cells(grid: [[Cell; 9]; 9]) -> Self {
        Self(grid, Unsolved)
    }
}
