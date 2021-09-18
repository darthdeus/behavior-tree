use behavior_tree::*;

#[test]
fn test_if() {
    struct Blackboard {
        is_foo: bool,
        result: String,
    }

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
