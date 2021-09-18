use behavior_tree::*;

#[test]
fn wait_behavior_test() {
    let mut tree: Node<()> = Node::wait(0.5);

    // Ticks bigger than the wait time get clamped to the wait time.
    let (status, _) = tree.tick(0.3, &mut ());
    assert_eq!(status, Status::Running);
    let (status, _) = tree.tick(0.3, &mut ());
    assert_eq!(status, Status::Success);
    let (status, _) = tree.tick(0.3, &mut ());
    assert_eq!(status, Status::Running);
    let (status, _) = tree.tick(0.3, &mut ());
    assert_eq!(status, Status::Success);
    let (status, _) = tree.tick(0.3, &mut ());
    assert_eq!(status, Status::Running);
    let (status, _) = tree.tick(0.3, &mut ());
    assert_eq!(status, Status::Success);

    let (status, _) = tree.tick(1.5, &mut ());
    assert_eq!(status, Status::Success);

    let (status, _) = tree.tick(0.1, &mut ());
    assert_eq!(status, Status::Running);
    let (status, _) = tree.tick(0.1, &mut ());
    assert_eq!(status, Status::Running);
    let (status, _) = tree.tick(0.1, &mut ());
    assert_eq!(status, Status::Running);
    let (status, _) = tree.tick(0.1, &mut ());
    assert_eq!(status, Status::Running);

    // Now that 0.4 has passed the timer is at ~0.1
    match tree.behavior {
        Behavior::Wait { curr, max: _ } => {
            assert!((curr - 0.1).abs() < 1e-5);
        }
        _ => panic!("Unexpected behavior type."),
    }

    let (status, _) = tree.tick(0.12, &mut ());
    assert_eq!(status, Status::Success);

    let (status, _) = tree.tick(0.1, &mut ());
    assert_eq!(status, Status::Running);
}
