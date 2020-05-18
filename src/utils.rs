macro_rules! arr32 {
    ( $( $item:expr ),* $(,)? ) => {
        [
            $( Word32($item), )*
        ]
    }
}
