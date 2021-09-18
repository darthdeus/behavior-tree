use behavior_tree::*;

#[test]
#[ignore]
fn test_simple_select() {
    let mut bt: Node<()> = Node::select(vec![
        Node::action("fail", |_| Status::Failure),
        YesTick::action(),
        NoTick::action(),
    ]);

    let (res, debug_repr) = bt.tick(1.0, &mut ());
    assert_eq!(res, Status::Success);
    assert_eq!(debug_repr.cursor.index(), 1);
}

// #[test]
// fn test_simple_select_inv() {
//     let mut bt: Node<()> = Node::select(vec![
//         Node::action("success", |_| Status::Success),
//         NoTick::action(),
//     ]);
// 
//     let (res, debug_repr) = bt.tick(1.0, &mut ());
//     assert_eq!(res, Status::Failure);
//     assert_eq!(debug_repr.cursor.index(), 1);
// }
