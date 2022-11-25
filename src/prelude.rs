#[macro_export]
macro_rules! commands {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec: Vec<Box<dyn CommandExt>> = Vec::new();
            $(
                temp_vec.push(Box::new($x));
            )*
            temp_vec
        }
    };
}
