#[macro_export]
macro_rules! duration_from {
    ( $m:expr ) => {
        Duration::new($m * 60, 0)
    };
    ( $m:expr, $s:expr ) => {
        Duration::new($m * 60 + $s, 0)
    };
    ( $m:expr, $s:expr, $n:expr ) => {
        Duration::new($m * 60 + $s, $n)
    };
}
