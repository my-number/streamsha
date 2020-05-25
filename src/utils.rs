macro_rules! arr32 {
    ( $( $item:expr ),* $(,)? ) => {
        [
            $( Word32($item), )*
        ]
    }
}

macro_rules! arr64 {
    ( $( $item:expr ),* $(,)? ) => {
        [
            $( Word64($item), )*
        ]
    }
}
