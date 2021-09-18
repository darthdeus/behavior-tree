//! Nodes in this module are intended for testing. While they're being
//! used to internally test this library they could also prove to be useful
//! for testing custom node types in user code.
use crate::prelude::*;

/// Node that will panic upon being dropped if it hasn't ticked
/// at least once.
///
/// Inspired by https://github.com/danieleades/aspen/blob/master/src/std_nodes/testing.rs
#[derive(Default)]
pub struct YesTick {
    pub ticked: bool,
}

impl YesTick {
    pub fn action<T>() -> Node<T> {
        Node::stateful_action("yes", Box::new(YesTick::default()))
    }
}

impl<T> StatefulAction<T> for YesTick {
    fn tick(&mut self, _data: &mut T) -> Status {
        self.ticked = true;
        Status::Success
    }

    fn reset(&mut self) {
        // no-op
    }
}

impl Drop for YesTick {
    fn drop(&mut self) {
        if !self.ticked {
            panic!("YesTick dropped without being ticked");
        }
    }
}

/// Node that will panic when it ticks.
///
/// Inspired by https://github.com/danieleades/aspen/blob/master/src/std_nodes/testing.rs
#[derive(Default)]
pub struct NoTick;

impl NoTick {
    pub fn action<T>() -> Node<T> {
        Node::stateful_action("no", Box::new(NoTick::default()))
    }
}

impl<T> StatefulAction<T> for NoTick {
    fn tick(&mut self, _data: &mut T) -> Status {
        panic!("NoTick node should never be ticked");
    }

    fn reset(&mut self) {
        panic!("Since NoTick should never be ticked, it should also never be reset");
    }
}
