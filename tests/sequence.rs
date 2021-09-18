use std::{cell::RefCell, rc::Rc};

use behavior_tree::*;
use crate::common::*;
mod common;

#[test]
fn test_simple_sequence() {
    let mut bt: Behavior<Counter> = sequence![Behavior::Action("inc_once", inc_once)];

    // S
    // |
    // A

    let mut data = Counter { value: 0 };

    let (status, debug_repr) = bt.tick(0.0, &mut data);
    assert_eq!(status, Status::Running);
    assert_eq!(data.value, 1);
    assert_eq!(debug_repr.cursor.index(), 0);

    let (status, debug_repr) = bt.tick(0.0, &mut data);
    assert_eq!(status, Status::Success);
    assert_eq!(data.value, 2);
    assert_eq!(debug_repr.cursor.index(), 1);
}

#[test]
fn test_stateful_action() {
    struct CounterWrap {
        value: Rc<RefCell<i32>>,
    }

    impl StatefulAction<()> for CounterWrap {
        fn tick(&mut self, _data: &mut ()) -> Status {
            let mut value = self.value.borrow_mut();

            if *value == 0 {
                *value += 1;
                Status::Running
            } else {
                Status::Success
            }
        }
    }

    let v1 = Rc::new(RefCell::new(0));
    let v2 = Rc::new(RefCell::new(0));

    let c1 = Box::new(CounterWrap { value: v1.clone() });
    let c2 = Box::new(CounterWrap { value: v2.clone() });

    let mut bt: Behavior<()> = sequence![
        Behavior::StatefulAction("inc_x".to_owned(), c1),
        Behavior::StatefulAction("inc_y".to_owned(), c2)
    ];

    //   Player visible?
    //  / \
    // S   Wait
    // |
    // X -> Y
    //
    // sequence(start_attack, wait, stop_attack)

    let mut data = ();
    assert_eq!(*v1.borrow(), 0);
    assert_eq!(*v2.borrow(), 0);

    let (status, debug_repr) = bt.tick(0.0, &mut data);
    assert_eq!(status, Status::Running);
    assert_eq!(debug_repr.cursor.index(), 0);
    assert_eq!(*v1.borrow(), 1);
    assert_eq!(*v2.borrow(), 0);

    let (status, debug_repr) = bt.tick(0.0, &mut data);
    assert_eq!(status, Status::Running);
    assert_eq!(debug_repr.cursor.index(), 1);
    assert_eq!(*v1.borrow(), 1);
    assert_eq!(*v2.borrow(), 1);

    let (status, debug_repr) = bt.tick(0.0, &mut data);
    assert_eq!(status, Status::Success);
    assert_eq!(debug_repr.cursor.index(), 2);
    assert_eq!(*v1.borrow(), 1);
    assert_eq!(*v2.borrow(), 1);

    // let (status, debug_repr) = bt.tick(0.0, &mut data);
    // assert_eq!(debug_repr.cursor.index(), 0);
    // assert_eq!(status, Status::Running);
}

#[test]
fn test_nested_sequence() {
    struct DoubleCounter {
        x: i32,
        y: i32,
    }

    fn inc_x(data: &mut DoubleCounter) -> Status {
        if data.x % 2 == 0 {
            data.x += 1;
            Status::Running
        } else {
            Status::Success
        }
    }

    fn inc_y(data: &mut DoubleCounter) -> Status {
        if data.y % 2 == 0 {
            data.y += 1;
            Status::Running
        } else {
            Status::Success
        }
    }

    let mut bt: Behavior<DoubleCounter> = sequence![
        Behavior::Action("inc_once_1", inc_x),
        Behavior::Action("inc_once_2", inc_y)
    ];

    // S
    // |
    // X -> Y
    //
    // sequence(start_attack, wait, stop_attack)

    let mut data = DoubleCounter { x: 0, y: 0 };

    let (status, debug_repr) = bt.tick(0.0, &mut data);
    assert_eq!(status, Status::Running);
    assert_eq!(data.x, 1);
    assert_eq!(data.y, 0);
    assert_eq!(debug_repr.cursor.index(), 0);

    let (status, debug_repr) = bt.tick(0.0, &mut data);
    assert_eq!(status, Status::Running);
    assert_eq!(data.x, 1);
    assert_eq!(data.y, 1);
    assert_eq!(debug_repr.cursor.index(), 1);

    let (status, debug_repr) = bt.tick(0.0, &mut data);
    assert_eq!(data.x, 1);
    assert_eq!(data.y, 1);
    assert_eq!(status, Status::Success);
    assert_eq!(debug_repr.cursor.index(), 2);

    // let (status, debug_repr) = bt.tick(0.0, &mut data);
    // dbg!(&debug_repr);
    // assert_eq!(data.value, 4);
    // assert_eq!(status, Status::Success);
    // assert_eq!(debug_repr.cursor.index(), 1);
}
