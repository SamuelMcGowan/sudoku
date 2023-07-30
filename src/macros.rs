#[macro_export]
macro_rules! grid {
    ($([$($t:tt)*])*) => {
        $crate::Grid::from_cells([$( [$( $crate::cell!($t), )*], )*])
    };
}

#[macro_export]
macro_rules! cell {
    (_) => {
        $crate::Cell::empty()
    };
    ($n:tt) => {
        $crate::Cell::value($n).expect("invalid cell value")
    };
}
