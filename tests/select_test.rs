use std::{cell::RefCell, rc::Rc};

use behavior_tree::*;

#[test]
fn test_simple_select() {
    let mut bt: Node<()> = Node::select(vec![
        Node::action("fail", |_| Status::Failure),
        Node::action("fail", |_| Status::Failure),
        YesTick::action(),
        NoTick::action(),
        NoTick::action(),
    ]);

    let res = bt.tick(1.0, &mut ());
    assert_eq!(res, Status::Success);

    assert_eq!(bt.status, Status::Success);
    if let Behavior::Select(_, ref nodes) = bt.behavior {
        assert_eq!(nodes[0].borrow().status, Status::Failure);
        assert_eq!(nodes[1].borrow().status, Status::Failure);
        assert_eq!(nodes[2].borrow().status, Status::Success);
        assert_eq!(nodes[3].borrow().status, Status::Initialized);
        assert_eq!(nodes[4].borrow().status, Status::Initialized);
    } else {
        panic!("Expected sequence")
    }

    bt.reset();
    assert_eq!(bt.status, Status::Initialized);

    if let Behavior::Select(_, ref nodes) = bt.behavior {
        for node in nodes.iter() {
            assert_eq!(node.borrow().status, Status::Initialized);
        }
    } else {
        panic!("Expected sequence")
    }
}

#[test]
fn test_simple_select_inv() {
    let mut bt: Node<()> = Node::select(vec![
        Node::action("success", |_| Status::Success),
        NoTick::action(),
        NoTick::action(),
        NoTick::action(),
        NoTick::action(),
    ]);

    let res = bt.tick(1.0, &mut ());
    assert_eq!(res, Status::Success);
}

#[test]
fn test_simple_select_running() {
    let mut bt: Node<()> = Node::select(vec![
        Node::action("fail", |_| Status::Failure),
        Node::action("fail", |_| Status::Failure),
        Node::action("fail", |_| Status::Failure),
        AlwaysRunning::action(),
        NoTick::action(),
        NoTick::action(),
        NoTick::action(),
        NoTick::action(),
    ]);

    let res = bt.tick(1.0, &mut ());
    assert_eq!(res, Status::Running);
}

#[test]
fn test_simple_select_fail() {
    let mut bt: Node<()> = Node::select(vec![
        Node::action("fail", |_| Status::Failure),
        Node::action("fail", |_| Status::Failure),
        Node::action("fail", |_| Status::Failure),
        Node::action("fail", |_| Status::Failure),
        Node::action("fail", |_| Status::Failure),
    ]);

    let res = bt.tick(1.0, &mut ());
    assert_eq!(res, Status::Failure);
}

#[test]
fn test_condition_recheck() {
    let const_status = Rc::new(RefCell::new(Status::Failure));

    let mut bt: Node<()> = Node::select(vec![
        Node::stateful_action(
            "const",
            Box::new(ConstAction {
                return_status: const_status.clone(),
            }),
        ),
        Node::stateful_action("counter", Box::new(Counter::default())),
        AlwaysRunning::action(),
    ]);

    let status = bt.tick(1.0, &mut ());
    assert_eq!(status, Status::Success);

    *const_status.borrow_mut() = Status::Success;

    let status = bt.tick(1.0, &mut ());
    assert_eq!(status, Status::Success);

    *const_status.borrow_mut() = Status::Running;

    let status = bt.tick(1.0, &mut ());
    assert_eq!(status, Status::Running);
}
