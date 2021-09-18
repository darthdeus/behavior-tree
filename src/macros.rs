#[macro_export]
macro_rules! cond {
    ($cond:expr, $a:expr, $b:expr $(,)?) => {
        Node::cond(
            stringify!($cond),
            $cond,
            $a,
            $b,
        )
    };
}

#[macro_export]
macro_rules! sequence {
    ($($x:expr),+ $(,)?) => {
        Node::sequence(vec![$($x),+])
    }
}
