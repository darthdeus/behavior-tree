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

    let (res, debug_repr) = bt.tick(1.0, &mut ());
    assert_eq!(res, Status::Success);
    assert_eq!(debug_repr.cursor.index(), 2);
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

    let (res, debug_repr) = bt.tick(1.0, &mut ());
    assert_eq!(res, Status::Success);
    assert_eq!(debug_repr.cursor.index(), 0);
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

    let (res, debug_repr) = bt.tick(1.0, &mut ());
    assert_eq!(res, Status::Running);
    assert_eq!(debug_repr.cursor.index(), 3);
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

    let (res, debug_repr) = bt.tick(1.0, &mut ());
    assert_eq!(res, Status::Failure);
    assert_eq!(debug_repr.cursor.index(), 5);
}
