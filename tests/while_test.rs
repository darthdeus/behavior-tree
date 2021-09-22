use behavior_tree::*;

struct Blackboard {
    cond: bool,
}

#[test]
fn test_simple_while_positive() {
    let mut bb = Blackboard { cond: false };
    let mut bt: Node<Blackboard> =
        Node::named_while_single("test", |data| data.cond, NoTick::action());

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
        Node::named_while_single("test", |data| data.cond, YesTick::action());

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
        Node::named_while_single("test", |data| data.cond, AlwaysRunning::action());

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
        Node::named_while_single("test", |data| data.cond, AlwaysRunning::action()),
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
        Node::named_while_single("test", |data| data.cond, NoTick::action()),
        YesTick::action(),
    ]);

    let (status, _) = bt.tick(1.0, &mut bb);
    assert_eq!(status, Status::Success);
    let (status, _) = bt.tick(1.0, &mut bb);
    assert_eq!(status, Status::Success);
    let (status, _) = bt.tick(1.0, &mut bb);
    assert_eq!(status, Status::Success);
}
