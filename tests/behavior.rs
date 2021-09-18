use behavior_tree::*;

use crate::common::*;
mod common;

#[test]
fn test_simple_action() {
    let mut bt: Behavior<Counter> = Behavior::Action("inc_once", inc_once);

    let mut data = Counter { value: 0 };

    let (status, _) = bt.tick(0.0, &mut data);
    assert_eq!(status, Status::Running);
    assert_eq!(data.value, 1);

    let (status, _) = bt.tick(0.0, &mut data);
    assert_eq!(status, Status::Success);
    assert_eq!(data.value, 2);
}
