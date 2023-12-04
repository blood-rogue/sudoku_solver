#[macro_export]
macro_rules! unroll {
    (for $v:ident in 0..$end:tt {$($c:tt)*}) => {
        unroll!(@$v, 0, $end, $($c)*);
    };

    (@$v:ident, $a:expr, 10, $($c:tt)*) => {
        { const $v: usize = $a; $($c)* }
        { const $v: usize = $a + 1; $($c)* }
        { const $v: usize = $a + 2; $($c)* }
        { const $v: usize = $a + 3; $($c)* }
        { const $v: usize = $a + 4; $($c)* }
        { const $v: usize = $a + 5; $($c)* }
        { const $v: usize = $a + 6; $($c)* }
        { const $v: usize = $a + 7; $($c)* }
        { const $v: usize = $a + 8; $($c)* }
        { const $v: usize = $a + 9; $($c)* }
    };

    (@$v:ident, $a:expr, 20, $($c:tt)*) => {
        unroll!(@$v, $a, 10, $($c)*);
        unroll!(@$v, $a + 10, 10, $($c)*);
    };

    (@$v:ident, $a:expr, 40, $($c:tt)*) => {
        unroll!(@$v, $a, 20, $($c)*);
        unroll!(@$v, $a + 20, 20, $($c)*);
    };

    (@$v:ident, $a:expr, 80, $($c:tt)*) => {
        unroll!(@$v, $a, 40, $($c)*);
        unroll!(@$v, $a + 40, 40, $($c)*);
    };

    (@$v:ident, $a:expr, 81, $($c:tt)*) => {
        unroll!(@$v, $a, 80, $($c)*);
        { const $v: usize = $a + 80; $($c)* }
    };
}
