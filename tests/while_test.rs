use std::{cell::RefCell, rc::Rc};

use behavior_tree::*;

struct Blackboard {
    cond: bool,
}

#[test]
fn test_simple_while_positive() {
    let mut bb = Blackboard { cond: false };
    let mut bt: Node<Blackboard> =
        Node::named_while_single("test", Box::new(|data| data.cond), NoTick::action());

    let (status, _) = bt.tick(1.0, &mut bb);
    assert_eq!(status, Status::Failure);
    let (status, _) = bt.tick(1.0, &mut bb);
    assert_eq!(status, Status::Failure);
    let (status, _) = bt.tick(1.0, &mut bb);
    assert_eq!(status, Status::Failure);
}

#[test]
fn test_simple_while_negative() {
    let mut bb = Blackboard { cond: true };
    let mut bt: Node<Blackboard> =
        Node::named_while_single("test", Box::new(|data| data.cond), YesTick::action());

    let (status, _) = bt.tick(1.0, &mut bb);
    assert_eq!(status, Status::Success);
    let (status, _) = bt.tick(1.0, &mut bb);
    assert_eq!(status, Status::Success);
    let (status, _) = bt.tick(1.0, &mut bb);
    assert_eq!(status, Status::Success);
}

#[test]
fn test_simple_while_running() {
    let mut bb = Blackboard { cond: true };
    let mut bt: Node<Blackboard> =
        Node::named_while_single("test", Box::new(|data| data.cond), AlwaysRunning::action());

    let (status, _) = bt.tick(1.0, &mut bb);
    assert_eq!(status, Status::Running);
    let (status, _) = bt.tick(1.0, &mut bb);
    assert_eq!(status, Status::Running);
    let (status, _) = bt.tick(1.0, &mut bb);
    assert_eq!(status, Status::Running);
}

#[test]
fn test_while_sequence() {
    let mut bb = Blackboard { cond: true };
    let mut bt: Node<Blackboard> = Node::sequence(vec![
        Node::named_while_single("test", Box::new(|data| data.cond), AlwaysRunning::action()),
        NoTick::action(),
    ]);

    let (status, _) = bt.tick(1.0, &mut bb);
    assert_eq!(status, Status::Running);
    let (status, _) = bt.tick(1.0, &mut bb);
    assert_eq!(status, Status::Running);
    let (status, _) = bt.tick(1.0, &mut bb);
    assert_eq!(status, Status::Running);
}

#[test]
fn test_while_select() {
    let mut bb = Blackboard { cond: false };
    let mut bt: Node<Blackboard> = Node::select(vec![
        Node::named_while_single("test", Box::new(|data| data.cond), NoTick::action()),
        YesTick::action(),
    ]);

    let (status, _) = bt.tick(1.0, &mut bb);
    assert_eq!(status, Status::Success);
    let (status, _) = bt.tick(1.0, &mut bb);
    assert_eq!(status, Status::Success);
    let (status, _) = bt.tick(1.0, &mut bb);
    assert_eq!(status, Status::Success);
}

#[test]
fn test_while_select_recheck() {
    let value = Rc::new(RefCell::new(true));
    let (counter_action, counter) = Counter::action(false);

    let v2 = value.clone();

    let mut bt: Node<()> = Node::select(vec![
        Node::named_while_single_child(
            "test",
            Box::new(move |_data| *v2.borrow()),
            counter_action,
        ),
        // Node::stateful_action(
        //     "const",
        //     Box::new(ConstAction {
        //         return_status: const_status.clone(),
        //     }),
        // ),
        // Node::stateful_action("counter", Box::new(Counter::default())),
        AlwaysRunning::action(),
    ]);

    let (status, debug_repr) = bt.tick(1.0, &mut ());
    assert_eq!(status, Status::Success);
    assert_eq!(debug_repr.cursor.index(), 0);
    assert_eq!(*counter.borrow(), 1);

    let (status, debug_repr) = bt.tick(1.0, &mut ());
    assert_eq!(status, Status::Success);
    assert_eq!(debug_repr.cursor.index(), 0);
    assert_eq!(*counter.borrow(), 2);

    *value.borrow_mut() = false;

    let (status, debug_repr) = bt.tick(1.0, &mut ());
    assert_eq!(status, Status::Running);
    assert_eq!(debug_repr.cursor.index(), 1);
    assert_eq!(*counter.borrow(), 2);

    let (status, debug_repr) = bt.tick(1.0, &mut ());
    assert_eq!(status, Status::Running);
    assert_eq!(debug_repr.cursor.index(), 1);
    assert_eq!(*counter.borrow(), 2);

    *value.borrow_mut() = false;

    let (status, debug_repr) = bt.tick(1.0, &mut ());
    assert_eq!(status, Status::Success);
    assert_eq!(debug_repr.cursor.index(), 0);
    assert_eq!(*counter.borrow(), 3);

    //     *const_status.borrow_mut() = Status::Success;
    //
    //     let (status, debug_repr) = bt.tick(1.0, &mut ());
    //     assert_eq!(status, Status::Success);
    //     assert_eq!(debug_repr.cursor.index(), 0);
    //
    //     *const_status.borrow_mut() = Status::Running;
    //
    //     let (status, debug_repr) = bt.tick(1.0, &mut ());
    //     assert_eq!(status, Status::Running);
    //     assert_eq!(debug_repr.cursor.index(), 0);
}
