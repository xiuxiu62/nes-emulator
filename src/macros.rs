/// Converts KiloBytes to Bytes
#[macro_export]
macro_rules! kb {
    ($kb: expr) => {
        $kb * 1024
    };
}

/// Creates a rom from a sequence of bytes
#[macro_export]
macro_rules! rom {
    [$($byte:expr),*] => {
        Rom::new(vec![$($byte),*])
    };
    [$value:expr; $count:expr] => {
        Rom::new(vec![$value; $count])
    };
}
