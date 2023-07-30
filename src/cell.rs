use std::fmt;

use crate::solve::{SolutionError, SolutionResult};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Cell(u16);

impl Cell {
    pub fn empty() -> Self {
        Self(0b1111111110)
    }

    pub fn value(value: u8) -> Option<Self> {
        match value {
            1..=9 => Some(Self(1 << value)),
            _ => None,
        }
    }

    pub(crate) fn constrain(
        &mut self,
        row: usize,
        col: usize,
        value: u8,
    ) -> SolutionResult<Option<u8>> {
        // if the cell was already constrained, don't do it again and produce
        // another constraint
        if ((self.0 >> value) & 1) == 0 {
            return Ok(None);
        }

        self.0 &= !(1 << value);

        match self.0.count_ones() {
            0 => Err(SolutionError::CellConflict(row, col)),
            1 => {
                let known = 15 - self.0.leading_zeros() as u8;
                Ok(Some(known))
            }
            _ => Ok(None),
        }
    }

    pub(crate) fn is_known(&self) -> bool {
        self.0.count_ones() == 1
    }

    pub(crate) fn known(&self) -> Option<u8> {
        self.is_known().then(|| 15 - self.0.leading_zeros() as u8)
    }

    pub(crate) fn unknown(&self) -> IterUnknowns {
        IterUnknowns {
            value: 0,
            flags: self.0,
        }
    }
}

pub(crate) struct IterUnknowns {
    value: u8,
    flags: u16,
}

impl Iterator for IterUnknowns {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        while self.value < 9 {
            self.value += 1;

            if ((self.flags >> self.value) & 1) != 0 {
                return Some(self.value);
            }
        }

        None
    }
}

impl fmt::Debug for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.unknown()).finish()
    }
}

#[cfg(test)]
#[test_log::test]
fn test_unknowns() {    
    let cell = Cell::empty();
    let unknowns: Vec<_> = cell.unknown().collect();
    assert_eq!(unknowns, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
}
