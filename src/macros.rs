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
        Node::named_sequence(stringify!($($x),+), vec![$($x),+])
    }
}

#[macro_export]
macro_rules! select {
    ($($x:expr),+ $(,)?) => {
        Node::named_select(stringify!($($x),+), vec![$($x),+])
    }
}
