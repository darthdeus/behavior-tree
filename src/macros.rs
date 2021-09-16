#[macro_export]
macro_rules! iff {
    ($cond:expr, $a:expr, $b:expr $(,)?) => {
        Behavior::If(
            stringify!($cond).to_string(),
            // Box::new($cond),
            $cond,
            Box::new($a),
            Box::new($b),
        )
    };
}

#[macro_export]
macro_rules! sequence {
    ($($x:expr),+ $(,)?) => {
        Behavior::Sequence(0, vec![$($x),+])
    }
}
