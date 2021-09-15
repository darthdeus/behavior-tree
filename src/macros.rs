#[macro_export]
macro_rules! status {
    ($name:expr, $status:expr) => {
        ($status, DebugRepr::new($name, $status))
    };
}

#[macro_export]
macro_rules! iff {
    ($cond:expr, $a:expr, $b:expr) => {
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
macro_rules! action {
    ($name:expr, $a:expr) => {
        Behavior::Action($name.to_owned(), $a)
    };
}

#[macro_export]
macro_rules! action_success {
    ($name:expr, $a:expr) => {
        Behavior::ActionSuccess($name.to_owned(), $a)
    };
}

#[macro_export]
macro_rules! sequence {
    ($($x:expr),+) => {
        Behavior::Sequence(0, vec![$($x),+])
    }
}
