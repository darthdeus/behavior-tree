use std::{cell::RefCell, rc::Rc};

use behavior_tree::*;

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

    let mut bt = Node::sequence(vec![
        Node::stateful_action("inc_x", c1),
        Node::stateful_action("inc_y", c2)
    ]);

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
