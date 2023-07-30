use crate::grid;
use crate::solve::SolutionError;

#[test]
fn puzzle_1() {
    let puzzle = grid![
        [6 _ 5 7 2 _ _ 3 9]
        [4 _ _ _ _ 5 1 _ _]
        [_ 2 _ 1 _ _ _ _ 4]
        [_ 9 _ _ 3 _ 7 _ 6]
        [1 _ _ 8 _ 9 _ _ 5]
        [2 _ 4 _ 5 _ _ 8 _]
        [8 _ _ _ _ 3 _ 2 _]
        [_ _ 2 9 _ _ _ _ 1]
        [3 5 _ _ 6 7 4 _ 8]
    ];
    let solution = grid![
        [6 1 5 7 2 4 8 3 9]
        [4 8 7 3 9 5 1 6 2]
        [9 2 3 1 8 6 5 7 4]
        [5 9 8 4 3 2 7 1 6]
        [1 3 6 8 7 9 2 4 5]
        [2 7 4 6 5 1 9 8 3]
        [8 4 9 5 1 3 6 2 7]
        [7 6 2 9 4 8 3 5 1]
        [3 5 1 2 6 7 4 9 8]
    ];

    let output = puzzle.solve(0).unwrap();
    assert_eq!(
        output, solution,
        "\nYour solution (left) did not match the correct solution (right)"
    );
}

#[test]
fn puzzle_2() {
    let puzzle = grid![
        [_ _ 8 _ 3 _ 5 4 _]
        [3 _ _ 4 _ 7 9 _ _]
        [4 1 _ _ _ 8 _ _ 2]
        [_ 4 3 5 _ 2 _ 6 _]
        [5 _ _ _ _ _ _ _ 8]
        [_ 6 _ 3 _ 9 4 1 _]
        [1 _ _ 8 _ _ _ 2 7]
        [_ _ 5 6 _ 3 _ _ 4]
        [_ 2 9 _ 7 _ 8 _ _]
    ];

    let solution = grid![
        [9 7 8 2 3 1 5 4 6]
        [3 5 2 4 6 7 9 8 1]
        [4 1 6 9 5 8 3 7 2]
        [8 4 3 5 1 2 7 6 9]
        [5 9 1 7 4 6 2 3 8]
        [2 6 7 3 8 9 4 1 5]
        [1 3 4 8 9 5 6 2 7]
        [7 8 5 6 2 3 1 9 4]
        [6 2 9 1 7 4 8 5 3]
    ];

    let output = puzzle.solve(0).unwrap();
    assert_eq!(
        output, solution,
        "\nYour solution (left) did not match the correct solution (right)"
    );
}

#[test]
fn puzzle_3() {
    let puzzle = grid![
        [2 9 5 7 4 3 8 6 1]
        [4 3 1 8 6 5 9 _ _]
        [8 7 6 1 9 2 5 4 3]
        [3 8 7 4 5 9 2 1 6]
        [6 1 2 3 8 7 4 9 5]
        [5 4 9 2 1 6 7 3 8]
        [7 6 3 5 2 4 1 8 9]
        [9 2 8 6 7 1 3 5 4]
        [1 5 4 9 3 8 6 _ _]
    ];

    // In marked rows, the 2 and 7 can be swapped and be valid.
    // The solver will get this solution though, so that's what we check against.
    let solution = grid![
        [2 9 5 7 4 3 8 6 1]
        [4 3 1 8 6 5 9 2 7] // <--
        [8 7 6 1 9 2 5 4 3]
        [3 8 7 4 5 9 2 1 6]
        [6 1 2 3 8 7 4 9 5]
        [5 4 9 2 1 6 7 3 8]
        [7 6 3 5 2 4 1 8 9]
        [9 2 8 6 7 1 3 5 4]
        [1 5 4 9 3 8 6 7 2] // <--
    ];

    let output = puzzle.solve(1).unwrap();

    assert_eq!(
        output, solution,
        "\nYour solution (left) did not match the correct solution (right)"
    );
}

#[test]
fn check_no_solution() {
    let puzzle = grid![
        [_ _ _ _ _ _ _ _ _]
        [_ _ _ _ _ _ _ _ _]
        [_ _ _ _ _ _ _ _ _]
        [_ _ _ _ _ _ _ _ _]
        [_ _ _ _ _ _ _ _ _]
        [_ _ _ _ _ _ _ _ _]
        [_ _ _ _ _ _ _ _ _]
        [_ 2 _ _ _ _ _ _ _]
        [1 _ 3 4 5 6 7 8 9]
    ];

    assert!(matches!(
        puzzle.solve(1),
        Err(SolutionError::CellConflict(8, 1))
    ));
}
