use crate::common::*;
use behavior_tree::*;
mod common;

#[test]
fn test_simple_sequence() {
    let mut bt: Node<()> = Node::sequence(vec![
        Node::action("success", |_| Status::Success),
        YesTick::action(),
        Node::action("success", |_| Status::Success),
        Node::action("success", |_| Status::Success),
        YesTick::action(),
    ]);

    assert_eq!(bt.status, Status::Initialized);

    let (res, debug_repr) = bt.tick(1.0, &mut ());
    assert_eq!(res, Status::Success);
    assert_eq!(debug_repr.cursor.index(), 5);

    assert_eq!(bt.status, Status::Success);
    if let Behavior::Sequence(_, ref nodes) = bt.behavior {
        for node in nodes.iter() {
            assert_eq!(node.borrow().status, Status::Success);
        }
    } else {
        panic!("Expected sequence")
    }

    bt.reset();
    assert_eq!(bt.status, Status::Initialized);

    if let Behavior::Sequence(_, ref nodes) = bt.behavior {
        for node in nodes.iter() {
            assert_eq!(node.borrow().status, Status::Initialized);
        }
    } else {
        panic!("Expected sequence")
    }
}

#[test]
fn test_simple_sequence_inv() {
    let mut bt: Node<()> = Node::sequence(vec![
        Node::action("failure", |_| Status::Failure),
        NoTick::action(),
        NoTick::action(),
        NoTick::action(),
        NoTick::action(),
    ]);

    assert_eq!(bt.status, Status::Initialized);

    let (res, debug_repr) = bt.tick(1.0, &mut ());
    assert_eq!(res, Status::Failure);
    assert_eq!(debug_repr.cursor.index(), 0);

    assert_eq!(bt.status, Status::Failure);
}

// Sequence
// S S R ....... -> R
// S S S S F ... -> F
// S S S S S S S -> S

// Select
// F F R ....... -> R
// F F F F S ... -> S
// F F F F F F F -> F

#[test]
fn test_simple_running() {
    let mut bt: Node<()> = Node::sequence(vec![
        Node::action("success", |_| Status::Success),
        YesTick::action(),
        AlwaysRunning::action(),
        NoTick::action(),
    ]);

    // Check that sequence doesn't step over running tasks
    for _ in 0..10 {
        let (res, debug_repr) = bt.tick(1.0, &mut ());
        assert_eq!(res, Status::Running);
        assert_eq!(debug_repr.cursor.index(), 2);
    }
}

#[test]
fn test_simple_sequence_pingpong() {
    let mut bt = Node::sequence(vec![Node::action("inc_pingpong", inc_pingpong)]);

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

    let (status, debug_repr) = bt.tick(0.0, &mut data);
    assert_eq!(status, Status::Running);
    assert_eq!(data.value, 3);
    assert_eq!(debug_repr.cursor.index(), 0);
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
            data.x += 1;
            Status::Success
        }
    }

    fn inc_y(data: &mut DoubleCounter) -> Status {
        if data.y % 2 == 0 {
            data.y += 1;
            Status::Running
        } else {
            data.y += 1;
            Status::Success
        }
    }

    let mut bt = Node::sequence(vec![
        Node::action("inc_once_1", inc_x),
        Node::action("inc_once_2", inc_y),
    ]);

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
    assert_eq!(data.x, 2);
    assert_eq!(data.y, 1);
    assert_eq!(debug_repr.cursor.index(), 1);

    let (status, debug_repr) = bt.tick(0.0, &mut data);
    assert_eq!(data.x, 2);
    assert_eq!(data.y, 2);
    assert_eq!(status, Status::Success);
    assert_eq!(debug_repr.cursor.index(), 2);

    let (status, debug_repr) = bt.tick(0.0, &mut data);
    assert_eq!(data.x, 3);
    assert_eq!(data.y, 2);
    assert_eq!(status, Status::Running);
    assert_eq!(debug_repr.cursor.index(), 0);

    let (status, debug_repr) = bt.tick(0.0, &mut data);
    assert_eq!(data.x, 4);
    assert_eq!(data.y, 3);
    assert_eq!(status, Status::Running);
    assert_eq!(debug_repr.cursor.index(), 1);

    let (status, debug_repr) = bt.tick(0.0, &mut data);
    assert_eq!(data.x, 4);
    assert_eq!(data.y, 4);
    assert_eq!(status, Status::Success);
    assert_eq!(debug_repr.cursor.index(), 2);
}
