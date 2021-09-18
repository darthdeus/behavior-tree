use behavior_tree::*;

#[test]
#[should_panic]
fn test_yes_tick_panics_without_tick() {
    let _bt: Node<()> = YesTick::action();
}

#[test]
fn test_yes_tick_likes_being_ticked() {
    let mut bt: Node<()> = YesTick::action();
    bt.tick(1.0, &mut ());
}


#[test]
fn test_no_tick_without_tick() {
    let _bt: Node<()> = NoTick::action();
}

#[test]
#[should_panic]
fn test_no_tick_crash_with_tick() {
    let mut bt: Node<()> = NoTick::action();
    bt.tick(1.0, &mut ());
}

