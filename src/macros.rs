#[macro_export]
macro_rules! times {
    ($start:expr, $end:expr; $i: ident => $e: block) => {{
        let mut $i = $start;
        while $i < $end {
            $e;
            $i += 1;
        }
    }};
}
