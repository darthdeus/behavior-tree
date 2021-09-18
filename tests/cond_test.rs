use behavior_tree::*;
mod common;
use crate::common::*;

#[derive(Default)]
    struct Blackboard {
        is_foo: bool,
        result: String,
    }


#[test]
fn test_cond_simple_true() {
    let mut bt: Node<Blackboard> = Node::cond(
        "is_foo",
        |data| data.is_foo,
        YesTick::action(),
        NoTick::action(),
    );

    let mut bb = Blackboard::default();
    bb.is_foo = true;
    bt.tick(1.0, &mut bb);
}

#[test]
fn test_cond_simple_false() {
    let mut bt: Node<Blackboard> = Node::cond(
        "is_foo",
        |data| data.is_foo,
        NoTick::action(),
        YesTick::action(),
    );

    let mut bb = Blackboard::default();
    bt.tick(1.0, &mut bb);
}

#[test]
fn test_cond() {
    let mut bt: Node<Blackboard> = Node::cond(
        "is_foo",
        |data| data.is_foo,
        Node::action_success("yes", |data| data.result = "yes".to_owned()),
        Node::action_success("no", |data| data.result = "no".to_owned()),
    );

    let mut bb = Blackboard {
        is_foo: false,
        result: "".to_string(),
    };

    let (status, debug) = bt.tick(1.0, &mut bb);
    assert_eq!(bb.result, "no");
    assert_eq!(status, Status::Success);
    assert_eq!(debug.cursor.index(), 1);

    bb.is_foo = true;

    let (status, debug) = bt.tick(1.0, &mut bb);
    assert_eq!(bb.result, "yes");
    assert_eq!(status, Status::Success);
    assert_eq!(debug.cursor.index(), 0);
}
