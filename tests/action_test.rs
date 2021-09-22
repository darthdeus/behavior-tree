use behavior_tree::*;

use crate::common::*;
mod common;

#[test]
fn test_simple_action() {
    let mut bt: Node<EvenCounter> = Node::action("inc_pingpong", inc_pingpong);

    let mut data = EvenCounter { value: 0 };

    let (status, _) = bt.tick(0.0, &mut data);
    assert_eq!(status, Status::Running);
    assert_eq!(data.value, 1);

    let (status, _) = bt.tick(0.0, &mut data);
    assert_eq!(status, Status::Success);
    assert_eq!(data.value, 2);

    let (status, _) = bt.tick(0.0, &mut data);
    assert_eq!(status, Status::Running);
    assert_eq!(data.value, 3);

    let (status, _) = bt.tick(0.0, &mut data);
    assert_eq!(status, Status::Success);
    assert_eq!(data.value, 4);

    let (status, _) = bt.tick(0.0, &mut data);
    assert_eq!(status, Status::Running);
    assert_eq!(data.value, 5);

    let (status, _) = bt.tick(0.0, &mut data);
    assert_eq!(status, Status::Success);
    assert_eq!(data.value, 6);
}
