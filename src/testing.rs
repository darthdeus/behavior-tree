//! Nodes in this module are intended for testing. While they're being
//! used to internally test this library they could also prove to be useful
//! for testing custom node types in user code.
use std::{cell::RefCell, rc::Rc};

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
        // no-op to allow testing reset on bigger trees
    }
}

/// Node that always runs.
pub struct AlwaysRunning;

impl AlwaysRunning {
    pub fn action<T>() -> Node<T> {
        Node::action("running", |_| Status::Running)
    }
}

/// Node that always returns a given status when ticked, but exposes the status
/// to be modified from the outside.
pub struct ConstAction {
    pub return_status: Rc<RefCell<Status>>,
}

impl ConstAction {
    pub fn new(status: Status) -> Self {
        Self {
            return_status: Rc::new(RefCell::new(status)),
        }
    }
}

impl<T> StatefulAction<T> for ConstAction {
    fn tick(&mut self, _data: &mut T) -> Status {
        *self.return_status.borrow()
    }

    fn reset(&mut self) {}
}
// impl<T> StatefulAction<T> for AlwaysRunning {
//     fn tick(&mut self, _data: &mut T) -> Status {
//         Status::Running
//     }
//
//     fn reset(&mut self) {
//         panic!("AlwaysRunning should never be reset");
//     }
// }

#[derive(Default)]
pub struct Counter {
    pub value: Rc<RefCell<i32>>,
    resettable: bool,
}

impl Counter {
    pub fn action<T>(resettable: bool) -> (Rc<RefCell<Node<T>>>, Rc<RefCell<i32>>) {
        let value = Rc::new(RefCell::new(0));
        (
            Rc::new(RefCell::new(Node::stateful_action(
                "counter",
                Box::new(Self {
                    value: value.clone(),
                    resettable,
                }),
            ))),
            value,
        )
    }
}

impl<T> StatefulAction<T> for Counter {
    fn tick(&mut self, _data: &mut T) -> Status {
        *self.value.borrow_mut() += 1;
        Status::Success
    }

    fn reset(&mut self) {
        if self.resettable {
            *self.value.borrow_mut() = 0;
        }
    }
}
